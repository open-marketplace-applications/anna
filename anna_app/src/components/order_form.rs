use yew::{html, Callback, Component, ComponentLink, Html, InputData, Properties, ShouldRender};

use yew::{
    format::{Json, Nothing},
    services::fetch::{FetchService, FetchTask, Request, Response},
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RegisterResponse {
    pub id: String,
    pub final_price: f64,
}

#[derive(Debug)]
pub enum Msg {
    SendOrder,
    ReceiveResponse(Result<RegisterResponse, anyhow::Error>),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Order {
     // user
     first_name: String,
     last_name: String,
     email: String,
     
     // address
     address: String,
     zip_code: String,
     city: String,
     country: String,
 
     // product
     amount: String,
     
     // TODO: remove this
     final_price: String,
}


#[derive(Debug)]
pub struct OrderForm {
    link: ComponentLink<Self>,
    fetch_task: Option<FetchTask>,
    url: String,
    order: Order,
    onsignal: Callback<RegisterResponse>,
    
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub onsignal: Callback<RegisterResponse>,
}

impl Component for OrderForm {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        
        let order = Order {
            first_name: "John".into(),
            last_name: "Doe".into(),
            address: "Kells".into(),
            zip_code: "5546".into(),
            city: "Downtown".into(),
            country: "PO".into(),
            email: "john.d@mail.ls".into(),
            amount: "1".into(),
            final_price: "9.00".into(),
        };
        
        Self { 
            link,
            order,
            fetch_task: None,
            url: "http://localhost:5000/api/orders".to_string(),
            onsignal: props.onsignal,

         }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::SendOrder => {
                log::info!("order_form::SendOrder {:?}", self);
                // sign message
                
                // encrypt message with app public key

                // send message to address
                
                 // 1. build the request
                 let request = Request::post(self.url.clone())
                 .header("Content-Type", "application/json")
                 .body(Json(&self.order))
                 .expect("Could not build request.");
             
                // 2. construct a callback
                let callback = self
                .link
                .callback(|response: Response<Json<Result<RegisterResponse, anyhow::Error>>>| {
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
                        // self.iss = Some(location);
                        self.onsignal.emit(reg_response);

                        // emit response to parent (shopping cart)
                    }
                    Err(error) => {
                        log::info!("error: {:?}", error);
                        // self.error = Some(error.to_string())
                    }
                }
                self.fetch_task = None;
                // we want to redraw so that the page displays the location of the ISS instead of
                // 'fetching...'
                
            }
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.onsignal = props.onsignal;
        true
    }

    fn view(&self) -> Html {
        html! {
            <div>
                <input class="firstname"
                    placeholder="First name"
                    value=&self.order.first_name />
                <input 
                    class="lastname"
                    placeholder="Last name"
                    value=&self.order.last_name />
                <input 
                    class="address"
                    placeholder="address"
                    value=&self.order.address />
                <input 
                    class="zip_code"
                    placeholder="zip_code"
                    value=&self.order.zip_code />
                <input 
                    class="country"
                    placeholder="country"
                    value=&self.order.country />
                <input 
                    class="email"
                    placeholder="email"
                    value=&self.order.email />
                <input 
                    class="amount"
                    placeholder="amount"
                    value=&self.order.amount />
                <button onclick=self.link.callback(|_| Msg::SendOrder)>{ "Buy now" }</button>
            </div>

        }
    }
}


