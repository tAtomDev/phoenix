pub mod common;
pub mod user_model;

use common::Stat;
use data::classes::CharacterClass;
use mongodb::{bson::doc, Database as MongoDatabase, Client, Collection, error::Error};
use user_model::UserData;

#[derive(Debug, Clone)]
pub struct Database {
    client: Client,
    database_name: String
}

impl Database {
    pub async fn new(uri: impl Into<String>, database_name: impl Into<String>) -> Self {
        let client = Client::with_uri_str(&uri.into()).await.expect("Failed to create a MongoDB Database");

        Database {
            client,
            database_name: database_name.into()
        }
    }

    pub fn db(&self) -> MongoDatabase {
        self.client.database(&self.database_name)
    }

    pub fn user_collection(&self) -> Collection<UserData> {
        self.db().collection_with_type::<UserData>("user")
    }

    pub async fn register_user_data(&self, user_id: String, class: CharacterClass) -> Result<(), mongodb::error::Error>  {
        let user_collection = self.user_collection();

        let mut user = UserData::new(user_id.into(), class.class_type);
        user.health = Stat::new(class.health);
        user.mana = Stat::new(class.mana);
        user.strength = class.strength;

        user_collection.insert_one(user, None).await?;
        Ok(())
    }

    async fn get_raw_user_data(&self, user_id: String) -> Result<Option<UserData>, Error> {
        let user_collection = self.user_collection();
        
        Ok(
            user_collection.find_one(doc! {
                "userId": user_id
            }, None).await?
        )
    }

    pub async fn get_user_data(&self, user_id: String) -> Result<Option<UserData>, Error> {
        let data = self.get_raw_user_data(user_id.clone()).await?;

        Ok(data)
    }
}