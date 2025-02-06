use crate::models::bot_user::{BotUser, NewBotUser};
use crate::schema::bot_users;
use diesel::internal::derives::multiconnection::chrono::Utc;
use diesel::prelude::*;
use fish_lib::database::DatabaseInterface;
use fish_lib::game::errors::repository::GameRepositoryError;
use fish_lib::traits::repository::Repository;
use std::sync::{Arc, RwLock};

pub trait BotUserRepositoryInterface: Repository<BotUser> + Send + Sync {}

pub struct BotUserRepository {
    db: Arc<RwLock<dyn DatabaseInterface>>,
}

impl BotUserRepository {
    pub fn new(db: Arc<RwLock<dyn DatabaseInterface>>) -> Self {
        Self { db }
    }
}

impl BotUserRepositoryInterface for BotUserRepository {}

impl Repository<BotUser> for BotUserRepository {
    fn get_db(&self) -> Arc<RwLock<dyn DatabaseInterface>> {
        self.db.clone()
    }

    fn create(&self, new_entity: NewBotUser) -> Result<BotUser, GameRepositoryError> {
        let mut connection = self.get_connection()?;

        let new_result = diesel::insert_into(bot_users::table)
            .values(new_entity)
            .get_result::<BotUser>(&mut connection)?;

        Ok(new_result)
    }

    fn find(&self, id: i64) -> Result<Option<BotUser>, GameRepositoryError> {
        let mut connection = self.get_connection()?;
        let user = bot_users::table
            .find(id)
            .first::<BotUser>(&mut connection)
            .optional()?;
        Ok(user)
    }

    fn save(&self, mut entity: BotUser) -> Result<BotUser, GameRepositoryError> {
        let mut connection = self.get_connection()?;
        entity.updated_at = Utc::now();

        let updated_user = diesel::update(bot_users::table)
            .filter(bot_users::id.eq(entity.id))
            .set(entity)
            .get_result::<BotUser>(&mut connection)?;

        Ok(updated_user)
    }

    fn delete(&self, entity: BotUser) -> Result<bool, GameRepositoryError> {
        let mut connection = self.get_connection()?;

        let deleted_count = diesel::delete(bot_users::table)
            .filter(bot_users::id.eq(entity.id))
            .execute(&mut connection)?;

        Ok(deleted_count > 0)
    }
}
