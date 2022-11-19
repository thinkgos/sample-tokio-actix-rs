use actix_web::{web, App, HttpServer};
use std::sync::{
    atomic::{AtomicI64, Ordering},
    Mutex,
};

struct AppStateWithCounter {
    number: AtomicI64,
    counter: Mutex<i32>, // <- Mutex is necessary to mutate safely across threads
}

async fn index(data: web::Data<AppStateWithCounter>) -> String {
    let mut counter = data.counter.lock().unwrap(); // <- get counter's MutexGuard
    *counter += 1; // <- access counter inside MutexGuard

    let number = data.number.fetch_add(1, Ordering::Acquire);

    format!("Request count: {counter}\n Request number: {number}") // <- response with count
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // 必须: Note: web::Data created _outside_ HttpServer::new closure
    let counter = web::Data::new(AppStateWithCounter {
        number: AtomicI64::new(0),
        counter: Mutex::new(0),
    });

    HttpServer::new(move || {
        // move counter into the closure
        App::new()
            .app_data(counter.clone()) // <- register the created data
            .route("/", web::get().to(index))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
