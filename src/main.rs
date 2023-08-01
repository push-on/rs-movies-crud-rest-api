// Import the necessary crates and modules
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Mutex;
use uuid::Uuid;

// Define the Movie struct with the required fields
#[derive(Debug, Serialize, Deserialize)]
struct Movie {
    id: Uuid,
    isbn: String,
    title: String,
    director: Director,
}

// Define the Director struct with the required fields
#[derive(Debug, Serialize, Deserialize, Clone)]
struct Director {
    firstname: String,
    lastname: String,
}

// Define a type alias for a shared state that holds a HashMap of movies
type MovieData = web::Data<Mutex<HashMap<Uuid, Movie>>>;

// Define a handler function for getting all movies
async fn get_movies(data: MovieData) -> impl Responder {
    // Lock the data and get a reference to the HashMap
    let movies = data.lock().unwrap();

    // Convert the HashMap into a Vec of values
    let movies: Vec<&Movie> = movies.values().collect();

    // Return a JSON response with the movies
    HttpResponse::Ok().json(movies)
}

// Define a handler function for getting a movie by ID
async fn get_movie_by_id(data: MovieData, id: web::Path<Uuid>) -> impl Responder {
    // Lock the data and get a reference to the HashMap
    let movies = data.lock().unwrap();

    // Try to find the movie by ID in the HashMap
    match movies.get(&id) {
        // If found, return a JSON response with the movie
        Some(movie) => HttpResponse::Ok().json(movie),
        // If not found, return a 404 response
        None => HttpResponse::NotFound().finish(),
    }
}

// Define a handler function for creating a new movie
async fn create_movie(data: MovieData, movie: web::Json<Movie>) -> impl Responder {
    // Lock the data and get a mutable reference to the HashMap
    let mut movies = data.lock().unwrap();

    // Generate a random ID for the new movie
    let id = Uuid::new_v4();

    // Create a new movie with the given fields and the generated ID
    let movie = Movie {
        id,
        isbn: movie.isbn.clone(),
        title: movie.title.clone(),
        director: movie.director.clone(),
    };

    // Insert the new movie into the HashMap with the ID as key
    movies.insert(id, movie);

    // Return a 201 response with the created movie
    HttpResponse::Created().json(movies.get(&id).unwrap())
}

// Define a handler function for updating a movie by ID
async fn update_movie_by_id(
    data: MovieData,
    id: web::Path<Uuid>,
    movie: web::Json<Movie>,
) -> impl Responder {
    // Lock the data and get a mutable reference to the HashMap
    let mut movies = data.lock().unwrap();

    // Try to find the movie by ID in the HashMap
    match movies.get_mut(&id) {
        // If found, update its fields with the given values
        Some(m) => {
            m.isbn = movie.isbn.clone();
            m.title = movie.title.clone();
            m.director = movie.director.clone();
            // Return a 200 response with the updated movie
            HttpResponse::Ok().json(m)
        }
        // If not found, return a 404 response
        None => HttpResponse::NotFound().finish(),
    }
}

// Define a handler function for deleting a movie by ID
async fn delete_movie_by_id(data: MovieData, id: web::Path<Uuid>) -> impl Responder {
    // Lock the data and get a mutable reference to the HashMap
    let mut movies = data.lock().unwrap();

    // Try to remove the movie by ID from the HashMap
    match movies.remove(&id) {
        // If found and removed, return a 204 response
        Some(_) => HttpResponse::NoContent().finish(),
        // If not found, return a 404 response
        None => HttpResponse::NotFound().finish(),
    }
}

// Define the main function that runs the server and registers the routes
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Create an initial HashMap of movies for testing purposes
    let mut movies = HashMap::new();

    // Insert some sample movies into the HashMap
    movies.insert(
        Uuid::new_v4(),
        Movie {
            id: Uuid::new_v4(),
            isbn: "978-3-16-148410-0".to_string(),
            title: "The Lord of the Rings".to_string(),
            director: Director {
                firstname: "Peter".to_string(),
                lastname: "Jackson".to_string(),
            },
        },
    );
    movies.insert(
        Uuid::new_v4(),
        Movie {
            id: Uuid::new_v4(),
            isbn: "978-0-06-055812-8".to_string(),
            title: "The Hitchhiker's Guide to the Galaxy".to_string(),
            director: Director {
                firstname: "Garth".to_string(),
                lastname: "Jennings".to_string(),
            },
        },
    );

    // Wrap the HashMap in a Mutex and a web::Data for shared state
    let data = web::Data::new(Mutex::new(movies));

    println!("Server starting on port 8080...");

    // Run the server and register the routes with the shared state
    HttpServer::new(move || {
        App::new()
            .app_data(data.clone())
            .route("/movies", web::get().to(get_movies))
            .route("/movies/{id}", web::get().to(get_movie_by_id))
            .route("/movies", web::post().to(create_movie))
            .route("/movies/{id}", web::put().to(update_movie_by_id))
            .route("/movies/{id}", web::delete().to(delete_movie_by_id))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await

}
