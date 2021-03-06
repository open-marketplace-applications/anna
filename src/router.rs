use yew::{html, Component, ComponentLink, Html, ShouldRender};
use yew_router::{route::Route, router::Router as YewRouter, switch::Permissive, Switch};

use crate::pages::{Cart, Home, ProductDetail, Products, Scanner};

use anna_chat::chat::ChatModel;
// use anna_home::Home;
use anna_profile::views::profile::Profile;
use anna_shop::views::shop::Shop;

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
    #[to = "/shop"]
    Shop,
    #[to = "/product/{id}"]
    ProductDetail(i32),
    #[to = "/cart"]
    Cart,
    #[to = "/404"]
    NotFound(Permissive<String>),
    #[to = "/"]
    Home,
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
                        AppRoutes::Shop => html! {<Shop />},
                        AppRoutes::ChatModel => html! {<ChatModel />},
                        AppRoutes::Cart => html! {<Cart />},
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
