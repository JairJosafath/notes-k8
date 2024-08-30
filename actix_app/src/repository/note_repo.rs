use crate::models::note_model::Note;
use futures::stream::TryStreamExt;
use mongodb::{
    bson::{doc, oid::ObjectId, Document},
    error::Error,
    results::{DeleteResult, InsertOneResult, UpdateResult},
    Client, Collection,
};

use dotenv::dotenv;
use std::env;

pub struct NoteRepository {
    pub collection: Collection<Note>,
    client: Client,
}

impl NoteRepository {
    pub async fn init() -> Self {
        dotenv().ok();
        const DATABASE_NAME: &str = "notesDatabase";
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

        client
            .database(DATABASE_NAME)
            .run_command(doc! {"ping": 1})
            .await
            .expect("Failed to ping MongoDB");

        print!("Connected to MongoDB\n");

        NoteRepository { collection, client }
    }

    pub async fn get_notes(&self) -> Result<Vec<Note>, Error> {
        let mut notes: Vec<Note> = Vec::new();
        let mut cursor = self
            .collection
            .find(doc! {})
            .await
            .expect("Failed to execute find.");
        while let Some(note) = cursor
            .try_next()
            .await
            .ok()
            .expect("Failed to iterate over results.")
        {
            notes.push(note);
        }
        Ok(notes)
    }

    pub async fn get_note(&self, id: String) -> Result<Option<Note>, Error> {
        let _id = ObjectId::parse_str(id).expect("Failed to parse ObjectId");
        let note = self
            .collection
            .find_one(doc! {"_id": _id})
            .await
            .expect("Failed to execute find_one.");
        Ok(note)
    }

    pub async fn create_note(&self, note: Note) -> Result<InsertOneResult, Error> {
        let result = self
            .collection
            .insert_one(note)
            .await
            .expect("Failed to insert document.");
        Ok(result)
    }

    pub async fn update_note(&self, id: String, note: Note) -> Result<UpdateResult, Error> {
        let _id = ObjectId::parse_str(id).expect("Failed to parse ObjectId");
        let result = self
            .collection
            .update_one(
                doc! {"_id":_id},
                doc! {"$set": {
                    "title": note.title,
                    "description": note.description,
                    "content": note.content
                }},
            )
            .await
            .expect("Failed to update document.");
        Ok(result)
    }

    pub async fn delete_note(&self, id: String) -> Result<DeleteResult, Error> {
        let _id = ObjectId::parse_str(id).expect("Failed to parse ObjectId");
        let result = self
            .collection
            .delete_one(doc! {"_id": _id})
            .await
            .expect("Failed to delete document.");
        Ok(result)
    }

    pub async fn test(&self) -> Result<Document, Error> {
        let result = self
            .client
            .database("notesDatabase")
            .run_command(doc! {"ping": 1})
            .await
            .expect("Failed to ping MongoDB");
        Ok(result)
    }
}
