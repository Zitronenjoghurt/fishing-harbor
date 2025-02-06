use crate::database::Database;
use crate::repositories::bot_user_repository::{BotUserRepository, BotUserRepositoryInterface};
use crate::services::user_service::{UserService, UserServiceInterface};
use fish_lib::config::ConfigInterface;
use fish_lib::database::DatabaseInterface;
use fish_lib::game::prelude::GameInterface;
use fish_lib::game::service_provider::ServiceProviderInterface as FishServiceProviderInterface;
use fish_lib::game::Game;
use std::sync::{Arc, RwLock};

pub trait GameAndServiceProviderInterface: GameInterface + FishServiceProviderInterface {}
impl GameAndServiceProviderInterface for Game {}

pub trait ServiceProviderInterface: Send + Sync {
    fn database(&self) -> Arc<RwLock<dyn DatabaseInterface>>;
    fn game(&self) -> Arc<dyn GameAndServiceProviderInterface>;
    fn bot_user_repository(&self) -> Arc<dyn BotUserRepositoryInterface>;
    fn bot_user_service(&self) -> Arc<dyn UserServiceInterface>;
}

pub struct ServiceProvider {
    database: Arc<RwLock<dyn DatabaseInterface>>,
    game: Arc<dyn GameAndServiceProviderInterface>,
    bot_user_repository: Arc<dyn BotUserRepositoryInterface>,
    bot_user_service: Arc<dyn UserServiceInterface>,
}

impl ServiceProvider {
    pub fn new(db_url_bot: &str, db_url_game: &str, game_config: Arc<dyn ConfigInterface>) -> Self {
        let game = Arc::new(Game::new(db_url_game, Some(game_config)).unwrap());

        let database = Database::create();
        database
            .write()
            .expect("Failed to get database write lock")
            .connect(db_url_bot)
            .unwrap();

        let bot_user_repository = Arc::new(BotUserRepository::new(database.clone()));

        let bot_user_service =
            Arc::new(UserService::new(bot_user_repository.clone(), game.clone()));

        Self {
            database,
            game,
            bot_user_repository,
            bot_user_service,
        }
    }
}

impl ServiceProviderInterface for ServiceProvider {
    fn database(&self) -> Arc<RwLock<dyn DatabaseInterface>> {
        self.database.clone()
    }

    fn game(&self) -> Arc<dyn GameAndServiceProviderInterface> {
        self.game.clone()
    }

    fn bot_user_repository(&self) -> Arc<dyn BotUserRepositoryInterface> {
        self.bot_user_repository.clone()
    }

    fn bot_user_service(&self) -> Arc<dyn UserServiceInterface> {
        self.bot_user_service.clone()
    }
}
