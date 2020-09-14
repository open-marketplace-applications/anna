mod home;
mod product_detail;
mod products;
mod profile;
mod shopping_cart;

use yew_router::switch::Permissive;
use yew_router::Switch;
// use yew_router::matcher::MatcherToken;

pub use home::Home;
pub use product_detail::ProductDetail;
pub use products::Products;
pub use profile::Profile;
pub use shopping_cart::ShoppingCart;

pub mod chat;
/// App routes
#[derive(Switch, Debug, Clone)]
pub enum AppRoutes {
    #[to = "/profile"]
    Profile,
    #[to = "/chat"]
    ChatModel,
    #[to = "/products"]
    Products,
    #[to = "/product/{id}"]
    ProductDetail(i32),
    #[to = "/shopping_cart"]
    ShoppingCart,
    #[to = "/404"]
    NotFound(Permissive<String>),
    #[to = "/"]
    Home,
    // #[to = MatcherToken::Exact("/")]
    // Home,
    // #[to = "/{*:any}"]
    // NotFound(Permissive<String>),
}
