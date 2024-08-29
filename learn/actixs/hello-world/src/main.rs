use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/notes")]
async fn add_note(req_body: String) -> impl Responder {
    println!("request body: {}", req_body);
    HttpResponse::Ok().body(req_body)
}

#[get("/notes")]
async fn get_notes() -> impl Responder {
    HttpResponse::Ok().body("get notes")
}

#[get("/notes/{id}")]
async fn get_note(id: web::Path<String>) -> impl Responder {
    println!("get note with id: {}", id);
    HttpResponse::Ok().body(format!("get note with id: {}", id))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(add_note)
            .service(get_notes)
            .service(get_note)

    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}