use serde::{Deserialize, Serialize};

mod auth;
mod product;

pub use product::Product;
pub use product::CartProduct;

pub use auth::Auth;
pub use auth::Login;
pub use auth::Signup;
pub use auth::User;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ServerResponse<T> {
    pub message: String,
    pub data: T,
}