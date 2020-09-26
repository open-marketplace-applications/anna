use yew::{html, Component, ComponentLink, Html, ShouldRender};
use yew_router::switch::Permissive;
use yew_router::{route::Route, router::Router as YewRouter};

use yew_router::Switch;
// use yew_router::matcher::MatcherToken;

use crate::pages::{ ProductDetail, Products, MyProducts, Profile, Scanner, ShoppingCart, chat::ChatModel };

use anna_home::Home;

/// App routes
#[derive(Switch, Debug, Clone)]
pub enum AppRoutes {
    #[to = "/profile"]
    Profile,
    #[to = "/scanner"]
    Scanner,
    #[to = "/chat"]
    ChatModel,
    #[to = "/products"]
    Products,
    #[to = "/my_products"]
    MyProducts,
    #[to = "/product/{id}"]
    ProductDetail(i32),
    #[to = "/shopping_cart"]
    ShoppingCart,
    #[to = "/404"]
    NotFound(Permissive<String>),
    #[to = "/"]
    Home,
    // #[to = MatcherToken::Exact("/")]
    // // Home,
    // #[to = "/{*:any}"]
    // NotFound(Permissive<String>),
}

pub struct Router {
    link: ComponentLink<Self>,
}

impl Component for Router {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self { link: _link }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <YewRouter<AppRoutes>
                render= YewRouter::render(move |switch: AppRoutes| {
                    match switch {
                        AppRoutes::Home => html!{<Home />},
                        AppRoutes::Profile => html!{<Profile />},
                        AppRoutes::Scanner => html!{<Scanner />},
                        AppRoutes::ProductDetail(id) => html! {<ProductDetail id=id />},
                        AppRoutes::Products => html! {<Products />},
                        AppRoutes::MyProducts => html! {<MyProducts />},
                        AppRoutes::ChatModel => html! {<ChatModel />},
                        AppRoutes::ShoppingCart => html! {<ShoppingCart />},
                        AppRoutes::NotFound(Permissive(None)) => html!{"Page not found"},
                        AppRoutes::NotFound(Permissive(Some(missed_route))) => html!{format!("Page '{}' not found", missed_route)}
                    }
                })
                redirect = YewRouter::redirect(|route: Route| {
                    AppRoutes::NotFound(Permissive(Some(route.route)))
                })
            />
        }
    }
}
