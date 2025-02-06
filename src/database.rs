use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use diesel::PgConnection;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use fish_lib::database::DatabaseInterface;
use fish_lib::game::errors::database::GameDatabaseError;
use std::sync::{Arc, RwLock};

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

pub struct Database {
    connection_pool: Option<Pool<ConnectionManager<PgConnection>>>,
}

impl Database {
    pub fn new() -> Self {
        Self {
            connection_pool: None,
        }
    }

    pub fn create() -> Arc<RwLock<dyn DatabaseInterface>> {
        Arc::new(RwLock::new(Self::new()))
    }
}

impl DatabaseInterface for Database {
    fn connect(&mut self, postgres_url: &str) -> Result<(), GameDatabaseError> {
        let connection_manager = ConnectionManager::<PgConnection>::new(postgres_url);
        let pool = Pool::builder()
            .build(connection_manager)
            .map_err(|e| GameDatabaseError::connection_failed(&e.to_string()))?;
        self.connection_pool = Some(pool);
        self.run_migrations()?;
        Ok(())
    }

    fn run_migrations(&self) -> Result<(), GameDatabaseError> {
        let mut connection = self.get_connection()?;
        connection
            .run_pending_migrations(MIGRATIONS)
            .map_err(|e| GameDatabaseError::migrations_failed(&e.to_string()))?;
        Ok(())
    }

    fn get_connection(
        &self,
    ) -> Result<PooledConnection<ConnectionManager<PgConnection>>, GameDatabaseError> {
        match &self.connection_pool {
            Some(pool) => pool
                .get()
                .map_err(|e| GameDatabaseError::connection_failed(&e.to_string())),
            None => Err(GameDatabaseError::missing_connection()),
        }
    }

    fn clear(&self) -> Result<(), GameDatabaseError> {
        let mut connection = self.get_connection()?;

        connection
            .revert_all_migrations(MIGRATIONS)
            .map_err(|e| GameDatabaseError::migrations_failed(&e.to_string()))?;

        connection
            .run_pending_migrations(MIGRATIONS)
            .map_err(|e| GameDatabaseError::migrations_failed(&e.to_string()))?;

        Ok(())
    }
}
