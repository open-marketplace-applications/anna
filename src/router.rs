use crate::pages::{AppRoutes, Home, ProductDetail, Products, Profile, ShoppingCart};
// use std::marker::PhantomData;
use yew::{html, Component, ComponentLink, Html, ShouldRender};
use yew_router::switch::Permissive;
use yew_router::{route::Route, router::Router as YewRouter};

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
                        AppRoutes::ProductDetail(id) => html! {<ProductDetail id=id />},
                        AppRoutes::Products => html! {<Products />},
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
