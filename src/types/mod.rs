use serde::{Deserialize, Serialize};

mod auth;
mod product;

pub use product::{CartProduct, Product};

pub use auth::{Auth, Login, Signup, User};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ServerResponse<T> {
    pub message: String,
    pub data: T,
}
