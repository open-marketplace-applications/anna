mod home;
mod profile;
mod products;
mod product_detail;

use yew_router::switch::Permissive;
use yew_router::Switch;
// use yew_router::matcher::MatcherToken;

pub use home::Home;
pub use profile::Profile;
pub use products::Products;
pub use product_detail::ProductDetail;

/// App routes
#[derive(Switch, Debug, Clone)]
pub enum AppRoutes {
    #[to = "/profile"]
    Profile,
    #[to = "/products"]
    Products,
    #[to = "/product/{id}"]
    ProductDetail(i32),
    #[to = "/404"]
    NotFound(Permissive<String>),
    #[to = "/"]
    Home,
    // #[to = MatcherToken::Exact("/")]
    // Home,
    // #[to = "/{*:any}"]
    // NotFound(Permissive<String>),
}