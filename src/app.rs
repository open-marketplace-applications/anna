use crate::types::{CartProduct, Product};
use yew::prelude::*;
use yew_router::prelude::*;
use crate::components::Layout;
use yew_router::{route::Route, router::Router as YewRouter};

use crate::router::Router;
use crate::components::Navbar;

struct State {
    cart_products: Vec<CartProduct>,
}

pub struct App {
    state: State,
    link: ComponentLink<Self>,
}

pub enum Msg {
    AddToCart(Product),
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let cart_products = vec![];

        Self {
            state: State { cart_products },
            link,
        }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
       true
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        html! {
            <Layout>
                <Navbar />
                <Router />
            </Layout>
        }
    }
}