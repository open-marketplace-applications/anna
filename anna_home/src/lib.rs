use css_in_rust::Style;
use yew::{html, Component, ComponentLink, Html, ShouldRender};

/// Home page for logging in or displaying the Dashboard.
pub struct Home {
    style: Style,
}

pub enum Msg {}

use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/src/js/paypal.js")]
extern "C" {
    fn show_button();
}

impl Component for Home {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        
        log::info!("home create ");
        
        unsafe {
            show_button();
        }

        let style = Style::create(
            String::from("home-page"),
            String::from(
                r#"
                "#,
            ),
        )
        .expect("An error occured while creating the style.");
        Self { style }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div class=self.style.to_string()>
                <h1>{"Open Marketplace Page"}</h1>
                <h2>{"Spotlight"}</h2>
                <p id="paypal-button">{"einfachIOTA Magazine 2 Vorverkauf"}</p>
                <a href="/cart">{"Jetzt vorbestellen!"}</a>
            </div>
        }
    }
}
