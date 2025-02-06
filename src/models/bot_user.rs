use diesel::internal::derives::multiconnection::chrono::{DateTime, Utc};
use diesel::prelude::*;
use fish_lib::traits::model::Model;

#[derive(Debug, Default, Clone, PartialEq, Queryable, Selectable, AsChangeset)]
#[diesel(table_name = crate::schema::bot_users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct BotUser {
    pub id: i64,
    pub username: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Model for BotUser {
    type Table = crate::schema::bot_users::table;
    type PrimaryKeyType = i64;
    type InsertType = NewBotUser;

    fn table() -> Self::Table {
        crate::schema::bot_users::table
    }

    fn id(&self) -> Self::PrimaryKeyType {
        self.id
    }
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::bot_users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewBotUser {
    pub id: i64,
    pub username: String,
}
