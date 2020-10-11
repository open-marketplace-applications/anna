use crate::router::AppRoutes;
use yew::{html, Callback, Component, ComponentLink, Html, InputData, Properties, ShouldRender};

use yew::{
    format::{Json, Nothing},
    services::fetch::{FetchService, FetchTask, Request, Response},
};use yew_router::components::RouterAnchor;

use crate::components::RegisterResponse;

use serde::{Deserialize, Serialize};


#[derive(Debug)]
pub struct PaymentForm {
    props: Props,
    link: ComponentLink<Self>,
    fetch_task: Option<FetchTask>,
    url: String,
}

#[derive(Clone)]
pub struct Test {

}

#[derive(Deserialize, Debug, Clone)]
pub struct PayWithPaypalResponse {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
}

#[derive(Properties, Clone, Debug)]
pub struct Props {
    pub order: RegisterResponse,
    #[prop_or_default]
    pub onsignal: Callback<Test>,
}

#[derive(Debug)]
pub enum Msg {
    PayWithPaypal,
    ReceiveResponse(Result<PayWithPaypalResponse, anyhow::Error>),

}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PayWithPaypalRequestBody {
    id: String

}
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/js/paypal.js")]
extern "C" {
    fn show_button(val: &JsValue);
    // pub fn handlePayPalPayment(val: &JsValue);
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PayPalPaymentObject {
    id: String
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
            props,
            link,
            fetch_task: None, 
            url: "http://localhost:5000/api/pay_with_paypal".to_string(),
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
        match msg {
            Msg::PayWithPaypal => {
                log::info!("order_form::SendOrder {:?}", self);

                 // 1. build the request
                 let request = Request::post(self.url.clone() + "?id=".into() + &*self.props.order.id)
                 .header("Content-Type", "application/json")
                 .body(Nothing)
                 .expect("Could not build request.");
             
                // 2. construct a callback
                let callback = self
                .link
                .callback(|response: Response<Json<Result<PayWithPaypalResponse, anyhow::Error>>>| {
                    log::info!("response: {:?}", response);
                    let Json(data) = response.into_body();
                    Msg::ReceiveResponse(data)
                });
                // 3. pass the request and callback to the fetch service
                let task = FetchService::fetch(request, callback).expect("failed to start request");
                log::info!("task: {:?}", task);
                // 4. store the task so it isn't canceled immediately
                self.fetch_task = Some(task);
                
            },
            Msg::ReceiveResponse(response) => {
                match response {
                    Ok(reg_response) => {
                        log::info!("reg_response: {:?}", reg_response);

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
        true
    }

    fn view(&self) -> Html {
        type Anchor = RouterAnchor<AppRoutes>;

        html! {
            <div class="payment_form">
                <button id="paypal-button" />
                <div>{&self.props.order.id}</div>
                
                <button id="hangoutButtonId" onclick=self.link.callback(|_| Msg::PayWithPaypal)>{ "PayWithPaypal" }</button>

            </div>
        }
    }
}
