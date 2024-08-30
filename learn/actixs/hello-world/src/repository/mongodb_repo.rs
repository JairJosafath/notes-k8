use dotenv::dotenv;
use std::env;

use crate::models::user_model::User;
use futures::stream::TryStreamExt;
use mongodb::{
    bson::{doc, extjson::de::Error, oid::ObjectId},
    results::{DeleteResult, InsertOneResult, UpdateResult}, //modify here
    Client,
    Collection,
};
pub struct MongoRepo {
    col: Collection<User>,
}

impl MongoRepo {
    pub async fn init() -> Self {
        dotenv().ok();
        // let uri = match env::var("MONGOURI") {
        //     Ok(v) => v.to_string(),
        //     Err(_) => formazt!("Error loading env variable"),
        // };
        let username = env::var("ME_CONFIG_MONGODB_ADMINUSERNAME")
            .expect("Error loading username env variable")
            .trim()
            .to_string();
        let password = env::var("ME_CONFIG_MONGODB_ADMINPASSWORD")
            .expect("Error loading password env variable")
            .trim()
            .to_string();
        let host = env::var("ME_CONFIG_MONGODB_SERVER")
            .expect("Error loading server env variable")
            .trim()
            .to_string();
        let uri = format!("mongodb://{}:{}@{}", username, password, host);
        // let uri = format!("{}", host);
        println!("url: {}", uri);
        let client = Client::with_uri_str(uri)
            .await
            .expect("Error connecting to MongoDB");
        let db = client.database("rustDB");
        let col: Collection<User> = db.collection("User");
        MongoRepo { col }
    }

    pub async fn create_user(&self, new_user: User) -> Result<InsertOneResult, Error> {
        let new_doc = User {
            id: None,
            name: new_user.name,
            location: new_user.location,
            title: new_user.title,
        };
        let user = self
            .col
            .insert_one(new_doc)
            .await
            .ok()
            .expect("Error creating user");
        Ok(user)
    }

    pub async fn get_user(&self, id: &String) -> Result<User, Error> {
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": obj_id};
        let user_detail = self
            .col
            .find_one(filter)
            .await
            .ok()
            .expect("Error getting user's detail");
        Ok(user_detail.unwrap())
    }
    pub async fn update_user(&self, id: &String, new_user: User) -> Result<UpdateResult, Error> {
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": obj_id};
        let new_doc = doc! {
            "$set":
                {
                    "id": new_user.id,
                    "name": new_user.name,
                    "location": new_user.location,
                    "title": new_user.title
                },
        };
        let updated_doc = self
            .col
            .update_one(filter, new_doc)
            .await
            .ok()
            .expect("Error updating user");
        Ok(updated_doc)
    }

    pub async fn delete_user(&self, id: &String) -> Result<DeleteResult, Error> {
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": obj_id};
        let delete_result = self
            .col
            .delete_one(filter)
            .await
            .ok()
            .expect("Error deleting user");
        Ok(delete_result)
    }

    pub async fn get_all_users(&self) -> Result<Vec<User>, Error> {
        let mut cursors = self
            .col
            .find(doc! {})
            .await
            .ok()
            .expect("Error getting list of users");
        let mut users: Vec<User> = Vec::new();
        while let Some(user) = cursors
            .try_next()
            .await
            .ok()
            .expect("Error mapping through cursor")
        {
            users.push(user)
        }
        Ok(users)
    }
}
