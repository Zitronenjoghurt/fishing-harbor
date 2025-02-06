use crate::repositories::bot_user_repository::BotUserRepositoryInterface;
use crate::service_provider::{
    GameAndServiceProviderInterface, ServiceProvider, ServiceProviderInterface,
};
use crate::services::user_service::UserServiceInterface;
use fish_lib::config::ConfigInterface;
use fish_lib::database::DatabaseInterface;
use std::sync::{Arc, RwLock};

pub trait AppStateInterface: Send + Sync {
    fn service_provider(&self) -> Arc<dyn ServiceProviderInterface>;
}

pub struct AppState {
    service_provider: Arc<dyn ServiceProviderInterface>,
}

impl AppState {
    pub fn new(db_url_bot: &str, db_url_game: &str, game_config: Arc<dyn ConfigInterface>) -> Self {
        let service_provider = Arc::new(ServiceProvider::new(db_url_bot, db_url_game, game_config));
        AppState { service_provider }
    }
}

impl AppStateInterface for AppState {
    fn service_provider(&self) -> Arc<dyn ServiceProviderInterface> {
        self.service_provider.clone()
    }
}

impl ServiceProviderInterface for AppState {
    fn database(&self) -> Arc<RwLock<dyn DatabaseInterface>> {
        self.service_provider.database()
    }

    fn game(&self) -> Arc<dyn GameAndServiceProviderInterface> {
        self.service_provider.game()
    }

    fn bot_user_repository(&self) -> Arc<dyn BotUserRepositoryInterface> {
        self.service_provider.bot_user_repository()
    }

    fn bot_user_service(&self) -> Arc<dyn UserServiceInterface> {
        self.service_provider.bot_user_service()
    }
}
