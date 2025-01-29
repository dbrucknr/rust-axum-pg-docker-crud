// Third Party Dependencies
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct CreateIdentity {
    pub email: String,
    pub password: String,
}

#[derive(Deserialize, Debug)]
pub struct UpdateIdentity {
    pub email: String,
    pub password: String,
}
