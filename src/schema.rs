diesel::table! {
    bot_users (id) {
        id -> BigInt,
        username -> VarChar,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}
