use crate::models::note_model::Note;
use mongodb::{Client, Collection};

use dotenv::dotenv;
use std::env;

const DATABASE_NAME: &str = "notesDatabase";

pub struct NoteRepository {
    collection: Collection<Note>,
}

impl NoteRepository {
    pub async fn init() -> Self {
        dotenv().ok();

        let user_name = env::var("MONGO_USERNAME").expect("MONGO_USERNAME must be set");
        let password = env::var("MONGO_PASSWORD").expect("MONGO_PASSWORD must be set");
        let host = env::var("MONGO_HOST").expect("MONGO_HOST must be set");

        let uri = format!("mongodb://{}:{}@{}", user_name, password, host)
            .trim()
            .to_string();

        let client = Client::with_uri_str(uri)
            .await
            .expect("Failed to connect to MongoDB");

        let db = client.database(DATABASE_NAME);
        let collection: Collection<Note> = db.collection("notes");

        NoteRepository { collection }
    }
}
