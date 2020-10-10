#![recursion_limit = "1024"]

#[macro_use()]
extern crate serde;


#[macro_use]
extern crate serde_json;

mod agents;
mod api;
mod app;
mod components;
mod pages;
mod router;
mod types;
mod utils;

use wasm_bindgen::prelude::*;

use image;
use rqrr;

use yew::{html, App, Component, ComponentLink, Html, ShouldRender};

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
pub enum Msg {
    SetOpposite(ComponentLink<Model>),
    SendToOpposite(String),
    SetTitle(String),
}

pub struct Model {
    link: ComponentLink<Self>,
    opposite: Option<ComponentLink<Model>>,
    selector: &'static str,
    title: String,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Model {
            link,
            opposite: None,
            selector: "",
            title: "Nothing".to_owned(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::SetOpposite(opposite) => {
                self.opposite = Some(opposite);
                false
            }
            Msg::SendToOpposite(title) => {
                self.opposite.as_mut().unwrap().send_message(Msg::SetTitle(title));
                false
            }
            Msg::SetTitle(title) => {
                let send_msg = match title.as_ref() {
                    "Ping" => Some(Msg::SetTitle("Pong".into())),
                    "Pong" => Some(Msg::SetTitle("Pong Done".into())),
                    "Pong Done" => Some(Msg::SetTitle("Ping Done".into())),
                    _ => None,
                };

                if let Some(send_msg) = send_msg {
                    self.opposite.as_mut().unwrap().send_message(send_msg);
                }

                self.title = title;
                true
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div>
                <h3>{ format!("{} received <{}>", self.selector, self.title) }</h3>
                <button onclick=self.link.callback(|_| Msg::SendToOpposite("One".into()))>{ "One" }</button>
                <button onclick=self.link.callback(|_| Msg::SendToOpposite("Two".into()))>{ "Two" }</button>
                <button onclick=self.link.callback(|_| Msg::SendToOpposite("Three".into()))>{ "Three" }</button>
                <button onclick=self.link.callback(|_| Msg::SendToOpposite("Ping".into()))>{ "Ping" }</button>
            </div>
        }
    }
}
fn mount_app(selector: &'static str, app: App<Model>) -> ComponentLink<Model> {
    let document = yew::utils::document();
    let element = document.query_selector(selector).unwrap().unwrap();
    app.mount(element)
}

#[wasm_bindgen]
pub fn run_app() -> Result<(), JsValue> {
    yew::initialize();

    let document = yew::utils::document();
    let element = document.query_selector(".app").unwrap().unwrap();
    let app: App<app::App> = App::new();
    app.mount(element);
    // let first_app = App::new();
    // let second_app = App::new();
    // let to_first = mount_app(".first-app", first_app);
    // let to_second = mount_app(".second-app", second_app);
    // to_first.send_message(Msg::SetOpposite(to_second.clone()));
    // to_second.send_message(Msg::SetOpposite(to_first));

    yew::run_loop();

    utils::set_panic_hook();
    wasm_logger::init(wasm_logger::Config::default());
    // yew::start_app::<app::App>();
    Ok(())
}

#[wasm_bindgen]
pub fn decode_qr(bytes: &[u8]) -> String {
    let img = match image::load_from_memory(&bytes) {
        Ok(v) => v,
        Err(_e) => return format!("{}", "[Error] Failed when trying to load image"),
    };

    let img = img.to_luma();

    // Prepare for detection
    let mut img = rqrr::PreparedImage::prepare(img);
    // Search for grids, without decoding
    let grids = img.detect_grids();

    if grids.len() != 1 {
        return format!("{}", "[Error] No QR code detected in image");
    }

    // Decode the grid
    let (_meta, content) = match grids[0].decode() {
        Ok(v) => v,
        Err(_e) => return format!("{}", "[Error] Failed decoding the image"),
    };

    return format!("{}", content);
}
