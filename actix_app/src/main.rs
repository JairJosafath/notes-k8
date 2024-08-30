mod api;
mod models;
mod repository;

use actix_web::{get, web::Data, App, HttpResponse, HttpServer, Responder};
use api::note_api::{create_note, delete_note, get_note, get_notes, test, update_note};
use repository::note_repo::NoteRepository;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Actix: Hello!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let mongodb = NoteRepository::init().await;
    let note_repo = Data::new(mongodb);
    HttpServer::new(move || {
        App::new()
            .app_data(note_repo.clone())
            .service(hello)
            .service(get_notes)
            .service(get_note)
            .service(create_note)
            .service(update_note)
            .service(delete_note)
            .service(test)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
