use crate::service_provider::ServiceProviderInterface;
use crate::state::AppState;
use fish_lib::config::{Config, ConfigBuilderInterface, ConfigInterface};
use std::sync::Arc;

const TEST_DATABASE_URL_BOT: &str = "postgresql://admin:root@db:5432/test_db";
const TEST_DATABASE_URL_GAME: &str = "postgresql://admin:root@db:5432/game_db";

pub fn mock_default_game_config() -> Arc<dyn ConfigInterface> {
    Config::builder().build()
}

pub fn mock_app_state(game_config: Arc<dyn ConfigInterface>) -> AppState {
    let app_state = AppState::new(TEST_DATABASE_URL_BOT, TEST_DATABASE_URL_GAME, game_config);
    // Clear both game and bot-specific data
    app_state
        .game()
        .database()
        .write()
        .unwrap()
        .clear()
        .unwrap();
    app_state.database().write().unwrap().clear().unwrap();
    app_state
}

pub fn mock_default_app_state() -> AppState {
    mock_app_state(mock_default_game_config())
}
