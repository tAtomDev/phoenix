pub mod cooldown;
pub mod user_model;

use cooldown::{CooldownData, CooldownType};
use data::classes::CharacterClass;
use data::Stat;
use mongodb::{
    bson::{doc, oid::ObjectId},
    error::Error,
    results::UpdateResult,
    Client, Collection, Database as MongoDatabase,
};
use user_model::UserData;

#[derive(Debug, Clone)]
pub struct Database {
    client: Client,
    database_name: String,
}

impl Database {
    pub async fn new(uri: impl Into<String>, database_name: impl Into<String>) -> Self {
        let client = Client::with_uri_str(&uri.into())
            .await
            .expect("Failed to create a MongoDB Database");

        Database {
            client,
            database_name: database_name.into(),
        }
    }

    pub fn db(&self) -> MongoDatabase {
        self.client.database(&self.database_name)
    }

    pub fn user_collection(&self) -> Collection<UserData> {
        self.db().collection_with_type::<UserData>("user")
    }

    pub fn cooldown_collection(&self) -> Collection<CooldownData> {
        self.db().collection_with_type::<CooldownData>("cooldown")
    }

    pub async fn create_user_cooldown(
        &self,
        user_id: &String,
        cooldown_type: CooldownType,
        expires_at: i64,
    ) -> Result<(), Error> {
        if let Ok(Some(cooldown)) = self.get_user_cooldown(user_id, cooldown_type).await {
            if !cooldown.expired() {
                return Ok(());
            }

            self.delete_user_cooldown(user_id, cooldown_type).await?;
        }

        let cooldown_collection = self.cooldown_collection();

        let cooldown = CooldownData {
            id: ObjectId::new(),
            cooldown_type,
            expires_at,
            user_id: user_id.to_string(),
        };

        cooldown_collection.insert_one(cooldown, None).await?;
        Ok(())
    }

    pub async fn delete_user_cooldown(
        &self,
        user_id: &String,
        cooldown_type: CooldownType,
    ) -> Result<(), Error> {
        let cooldown_collection = self.cooldown_collection();

        cooldown_collection
            .delete_one(
                doc! {
                    "userId": user_id,
                    "cooldownType": cooldown_type.to_string()
                },
                None,
            )
            .await?;

        Ok(())
    }

    pub async fn delete_all_cooldowns(
        &self,
    ) -> Result<(), Error> {
        let cooldown_collection = self.cooldown_collection();

        cooldown_collection
            .delete_many(
                doc! {},
                None,
            )
            .await?;

        Ok(())
    }

    pub async fn get_user_cooldown(
        &self,
        user_id: &String,
        cooldown_type: CooldownType,
    ) -> Result<Option<CooldownData>, Error> {
        let cooldown_collection = self.cooldown_collection();

        Ok(cooldown_collection
            .find_one(
                doc! {
                    "userId": user_id,
                    "cooldownType": cooldown_type.to_string()
                },
                None,
            )
            .await?)
    }

    pub async fn register_user_data(
        &self,
        user_id: &String,
        class: CharacterClass,
    ) -> Result<(), Error> {
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

    async fn get_raw_user_data(&self, user_id: &String) -> Result<Option<UserData>, Error> {
        let user_collection = self.user_collection();

        Ok(user_collection
            .find_one(
                doc! {
                    "userId": user_id
                },
                None,
            )
            .await?)
    }

    pub async fn is_user_registered(&self, user_id: &String) -> bool {
        let Ok(data) = self.get_raw_user_data(user_id).await else {
            return false;
        };

        data.is_some()
    }

    pub async fn get_user_data(&self, user_id: &String) -> Result<Option<UserData>, Error> {
        let data = self.get_raw_user_data(user_id).await?;

        Ok(data)
    }

    pub async fn save_user_data(&self, data: UserData) -> Result<UpdateResult, Error> {
        let user_collection = self.user_collection();

        Ok(user_collection
            .replace_one(doc! { "userId": data.user_id.clone() }, data, None)
            .await?)
    }
}
