use fish_lib::game::errors::repository::GameRepositoryError;
use fish_lib::game::errors::GameError;
use thiserror::Error;

pub mod command;

pub type BotResult<T> = Result<T, BotError>;

#[derive(Error, Debug)]
pub enum BotError {
    #[error("A database error occurred: {msg}")]
    Database {
        msg: String,
        #[source]
        source: Box<dyn std::error::Error>,
    },
    #[error("A game error occurred: {msg}")]
    Game {
        msg: String,
        #[source]
        source: Box<dyn std::error::Error>,
    },
}

impl BotError {
    pub fn database(error: Box<dyn std::error::Error>) -> Self {
        Self::Database {
            msg: error.to_string(),
            source: error,
        }
    }

    pub fn game(error: Box<dyn std::error::Error>) -> Self {
        Self::Game {
            msg: error.to_string(),
            source: error,
        }
    }
}

impl From<GameRepositoryError> for BotError {
    fn from(value: GameRepositoryError) -> Self {
        Self::database(value.into())
    }
}

impl From<GameError> for BotError {
    fn from(value: GameError) -> Self {
        if value.is_database_error() {
            value.into()
        } else {
            Self::game(value.into())
        }
    }
}
