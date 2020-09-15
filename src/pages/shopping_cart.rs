use crate::types::CartProduct;
use css_in_rust::Style;
use yew::{html, Component, ComponentLink, Html, ShouldRender, InputData};
use yew_state::{GlobalHandle, SharedStateComponent};
use yewtil::NeqAssign;

use crate::components::ShopingCartItem;
use crate::components::OrderForm;


pub struct Model {
    style: Style,
    cart_products: GlobalHandle<Vec<CartProduct>>,
    link: ComponentLink<Self>,
    value: String
}

pub enum Msg {
    GotInput(String),
    Clicked,
}

impl Component for Model {
    type Message = Msg;
    type Properties = GlobalHandle<Vec<CartProduct>>;

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
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
        }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
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

        html! {
            <div class=self.style.to_string()>
                <h1>{"shopping_cart"}</h1>
                <div class="product_card_list">{products}</div>

   
                <OrderForm />
            </div>
        }
    }
}

pub type ShoppingCart = SharedStateComponent<Model>;
