use bcrypt::{hash, DEFAULT_COST};
use serde::Deserialize;
use sqlx::{postgres::PgRow, Row};
use std::fmt;

pub(crate) const TABLE_NAME: &str = "star_wars_car_warranty_office";

pub mod queries {
    pub(crate) fn create_table() -> String {
        "
            CREATE TABLE IF NOT EXISTS star_wars_car_warranty_office (
                car varchar(255),
                email varchar(255),
                bank_details varchar(255),
                password varchar(255)
            )
        "
        .to_string()
    }

    pub(crate) fn create_user() -> String {
        format!(
            "INSERT INTO {} (car, email, bank_details, password) VALUES ($1, $2, $3, $4)",
            super::TABLE_NAME,
        )
    }

    pub(crate) fn login() -> String {
        format!(
            "select password from {} where password = $1",
            super::TABLE_NAME,
        )
    }

    pub(crate) fn get_details() -> String {
        format!(
            "select car,email,bank_details from {} where email = $1",
            super::TABLE_NAME,
        )
    }
}

pub(crate) fn hash_password(password: &str) -> Result<String, bcrypt::BcryptError> {
    hash(password, DEFAULT_COST)
}

#[derive(Deserialize, Debug, Clone)]
pub(crate) struct AccountLogin {
    pub(crate) email: String,
    pub(crate) pass: String,
}

#[derive(Deserialize, Debug, Clone)]
pub(crate) struct Account {
    pub(crate) email: String,
    pub(crate) car: String,
    pub(crate) bank_details: String,
    pub(crate) pass: String,
}

impl fmt::Display for AccountLogin {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.email)
    }
}

impl fmt::Display for Account {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.email)
    }
}
