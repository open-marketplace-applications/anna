use css_in_rust::Style;
use yew::{html, Component, ComponentLink, Html, ShouldRender};

use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/js/scanner.js")]
extern "C" {
    fn start_scanner();
}

/// Scanner page for QR Code Reader.
#[derive(Debug)]
pub struct Scanner {
    link: ComponentLink<Self>,
    style: Style,
}

pub enum Msg {
    Scan,
}

impl Component for Scanner {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let style = Style::create(
            String::from("home-page"),
            String::from(
                r#"
                "#,
            ),
        )
        .expect("An error occured while creating the style.");
        Self { link, style }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        match _msg {
            Msg::Scan => {
                unsafe {
                    // log::info!("scanner #{:?}", Scanner)
                    start_scanner();
                }
            }
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div class=self.style.to_string()>
                <h1>{"Scanner Page"}</h1>
                <div id="result"></div>
                <video autoplay="autoplay" id="video"></video>
                <button onclick=self.link.callback(|_| Msg::Scan)>{ "📷 Scan QR code" }</button>
            </div>
        }
    }
}
