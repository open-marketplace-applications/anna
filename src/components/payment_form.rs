use crate::router::AppRoutes;
use anyhow::Error;
use yew::{html, Callback, Component, ComponentLink, Html, InputData, NodeRef, Properties, ShouldRender};

use yew::{
    format::{Json, Nothing},
    services::fetch::{FetchService, FetchTask, Request, Response},
};
use yew_router::components::RouterAnchor;

use crate::components::RegisterResponse;

use serde::{Deserialize, Serialize};

extern crate qrcode;
use qrcode::{render::svg, types::QrError, QrCode};
use web_sys::Element;

use yew::services::websocket::{WebSocketService, WebSocketStatus, WebSocketTask};

#[derive(Deserialize, Debug, Clone)]
pub struct PayWithPaypalResponse {
    pub message: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct IOTAPayment {
    pub id: String,
    pub address: String,
    pub live_price: f64,
    pub value: i64,
}

#[derive(Deserialize, Debug, Clone)]
pub struct PayWithIOTAResponse {
    pub payment: IOTAPayment,
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
    PayWithIOTA,
    ReceiveResponse(Result<PayWithPaypalResponse, anyhow::Error>),
    ReceiveIOTAResponse(Result<PayWithIOTAResponse, anyhow::Error>),

    // websockets
    FetchData(Format, AsBinary),
    WsAction(WsAction),
    FetchReady(Result<DataFromFile, Error>),
    WsReady(Result<WsResponse, Error>),
}

impl From<WsAction> for Msg {
    fn from(action: WsAction) -> Self {
        Msg::WsAction(action)
    }
}
#[derive(Debug)]
pub enum WsAction {
    Connect,
    SendData(AsBinary),
    Disconnect,
    Lost,
}

type AsBinary = bool;

#[derive(Debug)]
pub enum Format {
    Json,
}
#[derive(Deserialize, Debug)]
pub struct DataFromFile {
    value: u32,
}
#[derive(Deserialize, Debug)]
pub struct WsResponse {
    value: u32,
}
#[derive(Serialize, Debug)]
struct WsRequest {
    value: u32,
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

#[wasm_bindgen(module = "/src/js/paypal.js")]
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
#[derive(Debug)]
pub struct PaymentForm {
    props: Props,
    link: ComponentLink<Self>,
    fetch_task: Option<FetchTask>,
    url: String,
    content: PayWithPaypalRequestBody,
    node_ref: NodeRef,
    node_ref2: NodeRef,
    paypay_id: String,
    onsignal: Callback<PayWithPaypalResponse>,

    // websockets
    fetching: bool,
    data: Option<u32>,
    ft: Option<FetchTask>,
    ws: Option<WebSocketTask>,
}

impl PaymentForm {
    fn view_data(&self) -> Html {
        if let Some(value) = self.data {
            html! {
                <p>{ value }</p>
            }
        } else {
            html! {
                <p>{ "Data hasn't fetched yet." }</p>
            }
        }
    }

    fn fetch_json(&mut self, binary: AsBinary) -> yew::services::fetch::FetchTask {
        let callback = self
            .link
            .batch_callback(move |response: Response<Json<Result<DataFromFile, Error>>>| {
                let (meta, Json(data)) = response.into_parts();
                println!("META: {:?}, {:?}", meta, data);
                if meta.status.is_success() {
                    // Msg::FetchReady(data)
                    Vec::new() // FIXME: Handle this error accordingly.
                } else {
                    Vec::new() // FIXME: Handle this error accordingly.
                }
            });
        let request = Request::get("/data.json").body(Nothing).unwrap();
        if binary {
            FetchService::fetch_binary(request, callback).unwrap()
        } else {
            FetchService::fetch(request, callback).unwrap()
        }
    }
}
impl Component for PaymentForm {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            props: props.clone(),
            link,
            fetch_task: None,
            url: "http://localhost:5000/api".to_string(),
            content: PayWithPaypalRequestBody { id: "".into() },
            node_ref: NodeRef::default(),
            node_ref2: NodeRef::default(),
            paypay_id: "".into(),
            onsignal: props.onsignal,

            // websockets
            fetching: false,
            data: None,
            ft: None,
            ws: None,
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
                let request = Request::post(self.url.clone() + "/pay_with_paypal?id=".into() + &*self.props.order.id)
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
            Msg::PayWithIOTA => {
                log::info!("PayWithIOTA {:?}", self);
                // 1. build the request
                let request = Request::post(self.url.clone() + "/pay_with_iota?id=".into() + &*self.props.order.id)
                    .header("Content-Type", "application/json")
                    .body(Nothing)
                    .expect("Could not build request.");

                // 2. construct a callback
                let callback =
                    self.link
                        .callback(|response: Response<Json<Result<PayWithIOTAResponse, anyhow::Error>>>| {
                            log::info!("response: {:?}", response);
                            let Json(data) = response.into_body();
                            Msg::ReceiveIOTAResponse(data)
                        });
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
            Msg::ReceiveIOTAResponse(response) => {
                match response {
                    Ok(reg_response) => {
                        log::info!("reg_response: {:?}", reg_response);
                        match qrcode(&reg_response.payment.address, 300, 300) {
                            Ok(v) => {
                                let el = self.node_ref2.cast::<Element>().unwrap();
                                el.set_inner_html(&v);
                            }
                            Err(e) => {
                                format!("{}", e);
                            }
                        };
                    }
                    Err(error) => {
                        log::info!("error: {:?}", error);
                        // self.error = Some(error.to_string())
                    }
                }
                self.fetch_task = None;
            }
            Msg::FetchData(format, binary) => {
                self.fetching = true;
                let task = match format {
                    Format::Json => self.fetch_json(binary),
                };
                self.ft = Some(task);
            }
            Msg::WsAction(action) => match action {
                WsAction::Connect => {
                    let callback = self.link.callback(|Json(data)| Msg::WsReady(data));
                    let notification = self.link.batch_callback(|status| match status {
                        WebSocketStatus::Opened => Vec::new(),
                        WebSocketStatus::Closed | WebSocketStatus::Error => {
                            // WsAction::Lost
                            Vec::new()
                        }
                    });
                    let task = WebSocketService::connect(
                        "ws://localhost:5000/iotapay/socket/?EIO=3&transport=websocket",
                        callback,
                        notification,
                    );

                    match task {
                        Ok(_task) => {
                            self.ws = Some(_task);
                        }
                        Err(err) => log::info!("error ws: {:?}", err),
                    }
                }
                WsAction::SendData(binary) => {
                    let request = WsRequest { value: 321 };
                    if binary {
                        self.ws.as_mut().unwrap().send_binary(Json(&request));
                    } else {
                        self.ws.as_mut().unwrap().send(Json(&request));
                    }
                }
                WsAction::Disconnect => {
                    self.ws.take();
                }
                WsAction::Lost => {
                    self.ws = None;
                }
            },
            Msg::FetchReady(response) => {
                self.fetching = false;
                self.data = response.map(|data| data.value).ok();
            }
            Msg::WsReady(response) => {
                self.data = response.map(|data| data.value).ok();
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
        let onclick_paypay = self.link.callback(|_| Msg::PayWithPaypal);
        let onclick_iota = self.link.callback(|_| Msg::PayWithIOTA);

        html! {
            <div class="payment_form">
                <button id="paypal-button" />
                <div>{&self.props.order.id}</div>
                <div>{&self.content}</div>
                <div id="paypal-id">{&self.paypay_id}</div>

                <button id="hangoutButtonId" onclick=onclick_paypay>{ "PayWithPaypal" }</button>
                <div ref=self.node_ref.clone()></div>
                <button id="PayWithIOTA" onclick=onclick_iota>{ "PayWithIOTA" }</button>
                <div ref=self.node_ref2.clone()></div>
                <button onclick=self.link.callback(|_| Msg::FetchData(Format::Json, false))>
                { "Fetch Data" }
                </button>
                <button onclick=self.link.callback(|_| Msg::FetchData(Format::Json, true))>
                    { "Fetch Data [binary]" }
                </button>
                { self.view_data() }
                <button disabled=self.ws.is_some()
                        onclick=self.link.callback(|_| WsAction::Connect)>
                    { "Connect To WebSocket" }
                </button>
                <button disabled=self.ws.is_none()
                        onclick=self.link.callback(|_| WsAction::SendData(false))>
                    { "Send To WebSocket" }
                </button>
                <button disabled=self.ws.is_none()
                        onclick=self.link.callback(|_| WsAction::SendData(true))>
                    { "Send To WebSocket [binary]" }
                </button>
                <button disabled=self.ws.is_none()
                        onclick=self.link.callback(|_| WsAction::Disconnect)>
                    { "Close WebSocket connection" }
                </button>
            </div>
        }
    }
}

fn qrcode<T>(data: T, width: u32, height: u32) -> Result<String, QrError>
where
    T: AsRef<[u8]>,
{
    QrCode::with_error_correction_level(data.as_ref(), qrcode::EcLevel::Q).map(|code| {
        code.render::<svg::Color>()
            .max_dimensions(width, height)
            .min_dimensions(width, height)
            .build()
    })
}
