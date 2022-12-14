pub mod user_model;

use data::Stat;
use data::classes::CharacterClass;
use mongodb::{bson::doc, Database as MongoDatabase, Client, Collection, error::Error, results::UpdateResult};
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
        user.agility = class.agility;
        user.intelligence = class.intelligence;

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

    pub async fn is_user_registered(&self, user_id: String) -> bool {
        let Ok(data) = self.get_raw_user_data(user_id.clone()).await else {
            return false;
        };

        data.is_some()
    }

    pub async fn get_user_data(&self, user_id: String) -> Result<Option<UserData>, Error> {
        let data = self.get_raw_user_data(user_id.clone()).await?;

        Ok(data)
    }

    pub async fn save_user_data(&self, data: UserData) -> Result<UpdateResult, Error> {
        let user_collection = self.user_collection();

        Ok(
            user_collection.replace_one(
                doc! { "userId": data.user_id.clone() }, 
                data, 
                None
            ).await?
        )
    }
}