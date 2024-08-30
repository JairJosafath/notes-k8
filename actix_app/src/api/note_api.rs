use crate::{models::note_model::Note, repository::note_repo::NoteRepository};

use actix_web::{delete, get, patch, post, web, HttpResponse, Responder};
use mongodb::bson::Document;
use serde::Serialize;

#[get("/notes")]
pub async fn get_notes(note_repo: web::Data<NoteRepository>) -> impl Responder {
    match note_repo.get_notes().await {
        Ok(notes) => HttpResponse::Ok().json(notes),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[get("/notes/{id}")]
pub async fn get_note(
    note_repo: web::Data<NoteRepository>,
    id: web::Path<String>,
) -> impl Responder {
    match note_repo.get_note(id.into_inner()).await {
        Ok(note) => HttpResponse::Ok().json(note),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[post("/notes")]
pub async fn create_note(
    note_repo: web::Data<NoteRepository>,
    note: web::Json<Note>,
) -> impl Responder {
    match note_repo.create_note(note.into_inner()).await {
        Ok(result) => HttpResponse::Ok().body(result.inserted_id.to_string()),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[patch("/notes/{id}")]
pub async fn update_note(
    note_repo: web::Data<NoteRepository>,
    id: web::Path<String>,
    note: web::Json<Note>,
) -> impl Responder {
    match note_repo
        .update_note(id.into_inner(), note.into_inner())
        .await
    {
        Ok(result) => HttpResponse::Ok().json(result),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[delete("/notes/{id}")]
pub async fn delete_note(
    note_repo: web::Data<NoteRepository>,
    id: web::Path<String>,
) -> impl Responder {
    match note_repo.delete_note(id.into_inner()).await {
        Ok(result) => HttpResponse::Ok().json(result),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}
#[derive(Serialize)]
struct TestResponse {
    message: String,
    result: Document,
}

#[get("/mongo")]
pub async fn test(note_repo: web::Data<NoteRepository>) -> impl Responder {
    match note_repo.test().await {
        Ok(result) => HttpResponse::Ok().json(TestResponse {
            message: "hello from mongo".to_string(),
            result,
        }),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}
