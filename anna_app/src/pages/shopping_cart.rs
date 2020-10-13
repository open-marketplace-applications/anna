use crate::types::CartProduct;
use css_in_rust::Style;
use yew::{html, Component, ComponentLink, Html, InputData, ShouldRender};
use yew_state::{GlobalHandle, SharedStateComponent};
use yewtil::NeqAssign;

use crate::components::{OrderForm, PayWithPaypalResponse, PaymentForm, RegisterResponse, ShopingCartItem};

pub struct Model {
    style: Style,
    cart_products: GlobalHandle<Vec<CartProduct>>,
    link: ComponentLink<Self>,
    value: String,
    scene: Scene,
    order: RegisterResponse,
}

pub enum Msg {
    HandleOrder(RegisterResponse),
    HandlePayment(PayWithPaypalResponse),
}

#[derive(Debug)]
pub enum Scene {
    ShippingForm,
    PaymentForm,
    PaymentSuccess,
}

impl Component for Model {
    type Message = Msg;
    type Properties = GlobalHandle<Vec<CartProduct>>;

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        log::info!("log info");
        let style = Style::create(
            String::from("shopping_cart"),
            String::from(
                r#"
                "#,
            ),
        )
        .expect("An error occured while creating the style.");
        Self {
            style,
            cart_products: _props,
            value: "".into(),
            link: _link,
            scene: Scene::ShippingForm,
            order: RegisterResponse {
                id: "".into(),
                final_price: 0.0,
            },
        }
    }

    fn update(&mut self, message: Self::Message) -> ShouldRender {
        match message {
            Msg::HandleOrder(response) => {
                log::info!("Handleresponse: {:?}", response);
                self.order = response;
                self.scene = Scene::PaymentForm;
                true
            }
            Msg::HandlePayment(response) => {
                log::info!("HandlePayment: ");
                self.scene = Scene::PaymentSuccess;
                true
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        self.cart_products.neq_assign(_props)
    }

    fn view(&self) -> Html {
        let cart_products = self.cart_products.state();
        log::info!("cart_products: {:?}", cart_products);

        let products: Vec<Html> = cart_products
            .iter()
            .map(|item: &CartProduct| {
                html! {
                <ShopingCartItem product={item} />
                }
            })
            .collect();

        match self.scene {
            Scene::ShippingForm => html! {
                <div class=self.style.to_string()>
                <h1>{"shopping_cart"}</h1>
                <div class="product_card_list">{products}</div>


                <OrderForm onsignal=self.link.callback(|response| Msg::HandleOrder(response))  />
            </div>
            },
            Scene::PaymentForm => html! {
                <div class=self.style.to_string()>
                    <h1>{"shopping_cart"}</h1>
                    <p>{"payment form"}</p>
                    <PaymentForm order=&self.order onsignal=self.link.callback(|response| Msg::HandlePayment(response)) />
                </div>
            },
            Scene::PaymentSuccess => html! {
                <div class=self.style.to_string()>
                    <h1>{"shopping_cart"}</h1>
                    <p>{"thanks for your payment!"}</p>
                </div>
            },
        }
    }
}

pub type Cart = SharedStateComponent<Model>;
