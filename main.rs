use rand::Rng;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::Deserialize;
use tokio;

// Global state for the secret number
struct AppState {
    secret_number: i32,
}

// Handler for the index route
async fn index() -> impl Responder {
    let html = include_str!("../static/index.html");
    HttpResponse::Ok().content_type("text/html").body(html)
}

// Handler for the guessing route
async fn guess_number(data: web::Query<GuessData>, state: web::Data<AppState>) -> impl Responder {
    let guess_num = data.value;

    match guess_num.cmp(&state.secret_number) {
        std::cmp::Ordering::Less => HttpResponse::Ok().body("Your guess is too low!"),
        std::cmp::Ordering::Greater => HttpResponse::Ok().body("Your guess is too high!"),
        std::cmp::Ordering::Equal => HttpResponse::Ok().body("Congratulations! You guessed the secret number!"),
    }
}

// GuessData struct for query parameter
#[derive(Deserialize)]
struct GuessData {
    value: i32,
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // Create a random number generator
    let mut rng = rand::thread_rng();

    // Generate a secret random number in the range 1 to 100
    let secret_number: i32 = rng.gen_range(1..=100);

    // Set up the Actix server
    HttpServer::new(move || {
        App::new()
            .data(AppState {
                secret_number,
            })
            .route("/", web::get().to(index))
            .route("/guess", web::get().to(guess_number))
    })
    .bind("127.0.0.1:8081")?
    .run()
    .await
}
