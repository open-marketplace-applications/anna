use yew::prelude::*;
use yew_router::prelude::*;
use wasm_bindgen::prelude::*;
use anna_design_system::{
    Footer,
    Page,
    Theme,
};

use crate::router::{AppRoutes, Router};
use crate::types::CartProduct;
use crate::components::Nav;

#[wasm_bindgen(module = "/js/wasm_bridge.js")]
extern "C" {
    fn js_test();
}

pub struct App {
    state: State,
    link: ComponentLink<Self>,
}

pub enum Msg {}

struct State {
    cart_products: Vec<CartProduct>,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let cart_products = vec![];
        unsafe {
            js_test();
        }
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
            <Theme>
                <Page>
                    <Nav />
                    <Router />
                    <Footer />
                </Page>
            </Theme>
        }
    }
}
