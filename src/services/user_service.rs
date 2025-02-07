use crate::errors::BotResult;
use crate::models::bot_user::{BotUser, NewBotUser};
use crate::repositories::bot_user_repository::BotUserRepositoryInterface;
use crate::service_provider::GameAndServiceProviderInterface;
use fish_lib::models::user::User as FishUser;
use poise::serenity_prelude::User as PoiseUser;
use std::sync::Arc;

pub trait UserServiceInterface: Send + Sync {
    fn register_or_fetch_user(&self, user: &PoiseUser) -> BotResult<(BotUser, FishUser)>;
    fn register_or_fetch_bot_user(&self, user: &PoiseUser) -> BotResult<BotUser>;
    fn register_or_fetch_fish_user(&self, user: &PoiseUser) -> BotResult<FishUser>;
    fn create_and_save_bot_user(&self, id: i64, username: String) -> BotResult<BotUser>;
}

pub struct UserService {
    bot_user_repository: Arc<dyn BotUserRepositoryInterface>,
    game: Arc<dyn GameAndServiceProviderInterface>,
}

impl UserService {
    pub fn new(
        bot_user_repository: Arc<dyn BotUserRepositoryInterface>,
        game: Arc<dyn GameAndServiceProviderInterface>,
    ) -> Self {
        Self {
            bot_user_repository,
            game,
        }
    }
}

impl UserServiceInterface for UserService {
    fn register_or_fetch_user(&self, user: &PoiseUser) -> BotResult<(BotUser, FishUser)> {
        Ok((
            self.register_or_fetch_bot_user(user)?,
            self.register_or_fetch_fish_user(user)?,
        ))
    }

    fn register_or_fetch_bot_user(&self, user: &PoiseUser) -> BotResult<BotUser> {
        let id = user.id.get() as i64;
        let name = user.name.clone();

        let mut bot_user = match self.bot_user_repository.find(id)? {
            Some(user) => user,
            None => self.create_and_save_bot_user(id, name.clone())?,
        };

        bot_user.username = name;
        Ok(self.bot_user_repository.save(bot_user)?)
    }

    fn register_or_fetch_fish_user(&self, user: &PoiseUser) -> BotResult<FishUser> {
        let id = user.id.get() as i64;
        match self.game.user_find(id) {
            Ok(user) => Ok(user),
            Err(error) if error.is_not_found() => Ok(self.game.user_register(id)?),
            Err(error) => Err(error.into()),
        }
    }

    fn create_and_save_bot_user(&self, id: i64, username: String) -> BotResult<BotUser> {
        let new_user = NewBotUser { id, username };
        Ok(self.bot_user_repository.create(new_user)?)
    }
}
