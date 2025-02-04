// Third Party Dependencies
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
// Local Dependencies
use crate::database::schema::identities;

#[derive(Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = identities)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Identity {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable, Serialize, Deserialize)]
#[diesel(table_name = identities)]
pub struct CreateIdentity {
    pub name: String,
    pub email: String,
}

#[derive(Deserialize, AsChangeset)]
#[diesel(table_name = identities)]
pub struct UpdateIdentity {
    pub name: Option<String>,
    pub email: Option<String>,
}
