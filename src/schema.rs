// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "role_t"))]
    pub struct RoleT;
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::RoleT;

    users (id) {
        id -> Uuid,
        name -> Nullable<Varchar>,
        email -> Varchar,
        phone -> Varchar,
        password -> Text,
        role -> RoleT,
    }
}
