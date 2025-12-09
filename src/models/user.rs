use chrono::NaiveDateTime;
use derive_more::{Display, From};

#[derive(Debug, sqlx::FromRow)]

pub struct User {
    pub username: String,
    pub role: Role,
    // Optional field
    // #[sqlx(default)]
    // pub created_at: Option<NaiveDateTime>,
    // #[sqlx(default)]
    // pub updated_at: Option<NaiveDateTime>,
}

#[derive(Debug, Default, Display, From)]
pub enum Role {
    #[default]
    #[display("customer")]
    Customer,
    #[display("seller")]
    Seller,
    #[display("admin")]
    Admin,
}

impl From<String> for Role {
    fn from(value: String) -> Self {
        match value.as_str() {
            "customer" => Self::Customer,
            "seller" => Self::Seller,
            "admin" => Self::Admin,
            _ => Self::default(),
        }
    }
}

pub struct Address {
    pub addr1: String,
    pub addr2: String,
    pub city: String,
    pub state: String,
    pub province: String,
    pub postal_code: String,
}
