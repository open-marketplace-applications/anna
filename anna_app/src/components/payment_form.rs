use crate::router::AppRoutes;
use yew::{html, Callback, Component, ComponentLink, Html, InputData, NodeRef, Properties, ShouldRender};

use yew::{
    format::{Json, Nothing},
    services::fetch::{FetchService, FetchTask, Request, Response},
};
use yew_router::components::RouterAnchor;

use crate::components::RegisterResponse;

use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct PaymentForm {
    props: Props,
    link: ComponentLink<Self>,
    fetch_task: Option<FetchTask>,
    url: String,
    content: PayWithPaypalRequestBody,
    node_ref: NodeRef,
    paypay_id: String,
    onsignal: Callback<PayWithPaypalResponse>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct PayWithPaypalResponse {
    pub message: String,
}

#[derive(Properties, Clone, Debug)]
pub struct Props {
    pub order: RegisterResponse,
    #[prop_or_default]
    pub onsignal: Callback<PayWithPaypalResponse>,
}

#[derive(Debug)]
pub enum Msg {
    PayWithPaypal,
    ReceiveResponse(Result<PayWithPaypalResponse, anyhow::Error>),
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PayWithPaypalRequestBody {
    id: String,
}
use std::fmt;

impl fmt::Display for PayWithPaypalRequestBody {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        write!(f, "{:?}", self)
    }
}
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/js/paypal.js")]
extern "C" {
    fn show_button(val: &JsValue);
// pub fn handlePayPalPayment(val: &JsValue);
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PayPalPaymentObject {
    id: String,
}

#[wasm_bindgen]
pub fn handlePayPalPayment(data: &JsValue, callback: &JsValue) {
    log::info!("handlePayPalPayment paypal data {:?}", data);
    let example: PayPalPaymentObject = data.into_serde().unwrap();
    log::info!("handlePayPalPayment example {:?}", example);
    log::info!("handlePayPalPayment callback {:?}", callback);
}

impl Component for PaymentForm {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            props: props.clone(),
            link,
            fetch_task: None,
            url: "http://localhost:5000/api/pay_with_paypal".to_string(),
            content: PayWithPaypalRequestBody { id: "".into() },
            node_ref: NodeRef::default(),
            paypay_id: "".into(),
            onsignal: props.onsignal,
        }
    }
    fn rendered(&mut self, first_render: bool) {
        if first_render {
            unsafe {
                show_button(&JsValue::from_serde(&self.props.order).unwrap());
            }
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        log::info!("log");
        match msg {
            Msg::PayWithPaypal => {
                log::info!("order_form::SendOrder {:?}", self);
                // stdweb
                let window: web_sys::Window = web_sys::window().expect("window not available");
                let document = window.document().expect("should have a document on window");
                let paypal_id_el: web_sys::Element = document.get_element_by_id("paypal-id").unwrap();
                log::info!("self.paypal_id_el: {:?}", paypal_id_el);
                log::info!("self.paypal_id_el iinner: {:?}", paypal_id_el.inner_html());

                let obj = PayWithPaypalRequestBody {
                    id: paypal_id_el.inner_html(),
                };
                // 1. build the request
                let request = Request::post(self.url.clone() + "?id=".into() + &*self.props.order.id)
                    .header("Content-Type", "application/json")
                    .body(Json(&obj))
                    .expect("Could not build request.");

                // 2. construct a callback
                let callback = self.link.callback(
                    |response: Response<Json<Result<PayWithPaypalResponse, anyhow::Error>>>| {
                        log::info!("response: {:?}", response);
                        let Json(data) = response.into_body();
                        Msg::ReceiveResponse(data)
                    },
                );
                // 3. pass the request and callback to the fetch service
                let task = FetchService::fetch(request, callback).expect("failed to start request");
                log::info!("task: {:?}", task);
                // 4. store the task so it isn't canceled immediately
                self.fetch_task = Some(task);
            }
            Msg::ReceiveResponse(response) => {
                match response {
                    Ok(reg_response) => {
                        log::info!("reg_response: {:?}", reg_response);
                        self.onsignal.emit(reg_response);
                        // emit response to parent (shopping cart)
                        // self.onsignal.emit(reg_response);
                    }
                    Err(error) => {
                        log::info!("error: {:?}", error);
                        // self.error = Some(error.to_string())
                    }
                }
                self.fetch_task = None;
            }
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        log::info!("change");
        true
    }

    fn view(&self) -> Html {
        type Anchor = RouterAnchor<AppRoutes>;
        let onclick_paypay_id = self.link.callback(|_| Msg::PayWithPaypal);

        html! {
            <div class="payment_form">
                <button id="paypal-button" />
                <div>{&self.props.order.id}</div>
                <div>{&self.content}</div>
                <div id="paypal-id">{&self.paypay_id}</div>

                <button id="hangoutButtonId" onclick=onclick_paypay_id>{ "PayWithPaypal" }</button>
                <div ref=self.node_ref.clone()></div>

            </div>
        }
    }
}
