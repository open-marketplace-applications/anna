use yew::prelude::*;

use crate::router::Router;
use crate::types::CartProduct;

// 📚 Design System
use crate::design_system::atoms::Page;
use crate::design_system::Footer;
use crate::design_system::Header;

struct State {
    cart_products: Vec<CartProduct>,
}

pub struct App {
    state: State,
    link: ComponentLink<Self>,
}

pub enum Msg {}

use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/js/wasm_bridge.js")]
extern "C" {
    fn js_test();
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
            <Page>
                <Header />
                <div class="main"></div>
                <Footer />
                <Router />
            </Page>
        }
    }
}
