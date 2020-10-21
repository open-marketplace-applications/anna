use yew::{html, Callback, Component, ComponentLink, Html, InputData, Properties, ShouldRender};

use serde::{Deserialize, Serialize};
use yew::{
    format::{Json, Nothing},
    services::fetch::{FetchService, FetchTask, Request, Response},
    utils, App, ChangeData,
};

use wasm_bindgen::JsCast;
use web_sys::{HtmlOptionElement, HtmlSelectElement};

use celes::Country;
use yew_styles::{
    forms::{
        form_component::Form,
        form_group::{FormGroup, Orientation},
        form_input::{FormInput, InputType},
        form_label::FormLabel,
        form_select::FormSelect,
        form_submit::FormSubmit,
    },
    styles::{Palette, Size, Style},
};

use design_system::Button;
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RegisterResponse {
    pub id: String,
    pub final_price: f64,
}

#[derive(Debug)]
pub enum Msg {
    SendOrder,
    ReceiveResponse(Result<RegisterResponse, anyhow::Error>),
    Select(String),
    Clicked(String),

    // Update Messages
    UpdateAmount(String),
    UpdateEmail(String),
    UpdateZipCode(String),
    UpdateAddress(String),
    UpdateFirstName(String),
    UpdateLastName(String),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Order {
    // person
    first_name: String,
    last_name: String,
    email: String,

    // address
    address: String,
    zip_code: String,
    city: String,
    country: String,

    // product
    amount: i32,

    // TODO: remove this
    final_price: f64,
}

#[derive(Debug)]
pub struct OrderForm {
    link: ComponentLink<Self>,
    fetch_task: Option<FetchTask>,
    url: String,
    order: Order,
    onsignal: Callback<RegisterResponse>,
    shipping_costs: f64,
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
            country: "DE".into(),
            email: "john.d@mail.ls".into(),
            amount: 1,
            final_price: 9.00,
        };

        Self {
            link,
            order,
            fetch_task: None,
            url: "http://localhost:5000/api/orders".to_string(),
            onsignal: props.onsignal,
            shipping_costs: 1.55,
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
                let callback =
                    self.link
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
            }
            Msg::ReceiveResponse(response) => {
                match response {
                    Ok(reg_response) => {
                        log::info!("reg_response: {:?}", reg_response);
                        // emit response to parent (shopping cart)
                        self.onsignal.emit(reg_response);
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
            Msg::Select(value) => {
                if value.eq(&"DE") {
                    log::info!("matches: {:?}", value);
                    self.shipping_costs = 1.55
                } else {
                    log::info!("matches not: {:?}", value);
                    self.shipping_costs = 3.70
                }
                self.order.country = value;
            }
            Msg::Clicked(value) => log::info!("logi: {:?}", value),

            // Update order varliales from forms messages
            Msg::UpdateAmount(value) => {
                self.order.amount = value.parse().unwrap();
            }
            Msg::UpdateEmail(value) => {
                self.order.email = value;
            }
            Msg::UpdateZipCode(value) => {
                self.order.zip_code = value;
            }
            Msg::UpdateAddress(value) => {
                self.order.address = value;
            }
            Msg::UpdateLastName(value) => {
                self.order.first_name = value;
            }
            Msg::UpdateFirstName(value) => {
                self.order.last_name = value;
            }
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.onsignal = props.onsignal;
        true
    }
    fn rendered(&mut self, first_render: bool) {
        if first_render {
            set_default_selected("country");
        }
    }

    fn view(&self) -> Html {
        let countries = Country::get_countries();

        let country = |model: &Country| {
            html! {
                <option value={model.alpha2.clone()}>{ model }</option>
            }
        };
        html! {
            <div>
            <Button
                onclick_signal=&self.link.callback(move |_| Msg::Clicked("Hello world".into()))
                button_size=Size::Medium
                >{"Greeting"}</Button>
                <Form onsubmit_signal=self.link.callback(|e| Msg::SendOrder)>

                    <FormGroup orientation=Orientation::Horizontal>
                        <FormLabel text="firstname: "/>
                        <input
                            oninput=self.link.callback(|e: InputData| Msg::UpdateFirstName(e.value))
                            class="firstname"
                            placeholder="firstname"
                            value=&self.order.first_name />
                    </FormGroup>
                    <FormGroup orientation=Orientation::Horizontal>
                        <FormLabel text="lastname: "/>
                        <input
                            oninput=self.link.callback(|e: InputData| Msg::UpdateLastName(e.value))
                            class="lastname"
                            placeholder="lastname"
                            value=&self.order.last_name />
                    </FormGroup>
                    <FormGroup orientation=Orientation::Horizontal>
                        <FormLabel text="address: "/>
                        <input
                            oninput=self.link.callback(|e: InputData| Msg::UpdateAddress(e.value))
                            class="address"
                            placeholder="address"
                            value=&self.order.address />
                    </FormGroup>
                    <FormGroup orientation=Orientation::Horizontal>
                        <FormLabel text="zip_code: "/>
                        <input
                            oninput=self.link.callback(|e: InputData| Msg::UpdateZipCode(e.value))
                            class="zip_code"
                            placeholder="zip_code"
                            value=&self.order.zip_code />
                    </FormGroup>
                    <FormGroup orientation=Orientation::Horizontal>
                        <FormLabel text="Country: "/>
                        <FormSelect
                            id="country"
                            select_size=Size::Medium
                            onchange_signal = &self.link.callback(|e: ChangeData|
                                match e {
                                    ChangeData::Select(element) => {
                                        let value = element.value();
                                        Msg::Select(value)
                                    },
                                    _ => unreachable!(),
                                }
                            )
                            options=html!{
                                <>
                                { for countries.iter().map(country) }
                                </>
                            }
                        />
                    </FormGroup>
                    <FormGroup orientation=Orientation::Horizontal>
                        <FormLabel text="Amount: "/>
                        <input
                            oninput=self.link.callback(|e: InputData| Msg::UpdateAmount(e.value))
                            class="amount"
                            placeholder="amount"
                            value=&self.order.amount />
                    </FormGroup>

                    <FormGroup orientation=Orientation::Horizontal>
                        <FormLabel text="Email: "/>
                        <FormInput
                            error_message="Email field is required"
                            input_type=InputType::Email
                            oninput_signal=self.link.callback(|e: InputData| Msg::UpdateEmail(e.value))
                        />
                    </FormGroup>
                    <p>{"Shipping Costs: "}{&self.shipping_costs}</p>
                    <p>{"Total: "}{(&self.order.final_price * f64::from(self.order.amount.clone())) + &self.shipping_costs}</p>

                    <FormGroup>
                        <FormSubmit
                            value="Submit application"
                            submit_palette=Palette::Success
                            submit_style=Style::Outline
                        />
                    </FormGroup>
                </Form>
            </div>

        }
    }
}

fn set_default_selected(select: &str) {
    let specialty_form_element = utils::document()
        .get_element_by_id(select)
        .unwrap()
        .dyn_into::<HtmlSelectElement>()
        .unwrap();
    let specialty_options = specialty_form_element.options();

    let option = specialty_options
        .get_with_index(76)
        .unwrap()
        .dyn_into::<HtmlOptionElement>()
        .unwrap();

    option.set_selected(true);
}
