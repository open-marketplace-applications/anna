use crate::components::AddToCartButton;
use crate::types::CartProduct;
use yew::prelude::*;
use yew_router::components::RouterAnchor;
use crate::pages::{AppRoutes};

pub struct ShopingCartItem {
    props: Props,
}

#[derive(Properties, Clone)]
pub struct Props {
    pub product: CartProduct,
    #[prop_or_default]
    pub on_add_to_cart: Callback<CartProduct>,
}

impl Component for ShopingCartItem {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self { props }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        type Anchor = RouterAnchor<AppRoutes>;
        let sum = self.props.product.product.price * self.props.product.quantity as f64;
        html! {
            <div class="product_card_container">
                <Anchor route=AppRoutes::ProductDetail(self.props.product.product.id) classes="product_card_anchor">
                    <img class="product_card_image" src={&self.props.product.product.image}/>
                    <div class="product_card_name">{&self.props.product.product.name}</div>
                    <div class="product_card_price">{"Single Price: $"}{&self.props.product.product.price}</div>
                    <div class="product_card_price">{"Quantity: "}{&self.props.product.quantity}</div>
                    <div class="product_card_price">{format!("Sum: ${:.2}", sum)}</div>
                </Anchor>
            </div>
        }
    }
}