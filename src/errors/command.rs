use crate::errors::BotError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum CommandError {
    #[error("An internal error occurred. Please try again later.")]
    Internal,
}

impl From<BotError> for CommandError {
    fn from(error: BotError) -> Self {
        println!("{}", error);
        match error {
            BotError::Database { .. } => CommandError::Internal,
            BotError::Game { .. } => CommandError::Internal,
        }
    }
}
