use crate::components::AddToCartButton;
use crate::router::AppRoutes;
use crate::types::Product;
use yew::prelude::*;
use yew_router::components::RouterAnchor;

use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/js/ipfs.js")]
extern "C" {
    fn write_file();
}

pub struct MyProductCard {
    props: Props,
    link: ComponentLink<Self>,
}

pub enum Msg {
    Publish,
}

#[derive(Properties, Clone)]
pub struct Props {
    pub product: Product,
    #[prop_or_default]
    pub on_add_to_cart: Callback<Product>,
}

impl Component for MyProductCard {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { props, link }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Publish => {
                log::info!("Publish");
                unsafe {
                    write_file();
                }
            }

        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        type Anchor = RouterAnchor<AppRoutes>;
        let onclick = self.link.callback(|_| Msg::Publish);

        html! {
            <div class="product_card_container">
                <Anchor route=AppRoutes::ProductDetail(self.props.product.id) classes="product_card_anchor">
                    <img class="product_card_image" src={&self.props.product.image}/>
                    <div class="product_card_name">{&self.props.product.name}</div>
                    <div class="product_card_price">{"$"}{&self.props.product.price}</div>
                </Anchor>
                <button onclick = onclick>{"Publish"}</button>
            </div>
        }
    }
}
