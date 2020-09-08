use crate::components::AddToCartButton;
use crate::types::Product;
use yew::prelude::*;
use yew_router::components::RouterAnchor;
use crate::pages::{AppRoutes};

pub struct ProductCard {
    props: Props,
}

#[derive(Properties, Clone)]
pub struct Props {
    pub product: Product,
    #[prop_or_default]
    pub on_add_to_cart: Callback<Product>,
}

impl Component for ProductCard {
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

        html! {
            <div class="product_card_container">
                <Anchor route=AppRoutes::ProductDetail(self.props.product.id) classes="product_card_anchor">
                    <img class="product_card_image" src={&self.props.product.image}/>
                    <div class="product_card_name">{&self.props.product.name}</div>
                    <div class="product_card_price">{"$"}{&self.props.product.price}</div>
                </Anchor>
                <AddToCartButton product=self.props.product.clone() on_add_to_cart=self.props.on_add_to_cart.clone() />
            </div>
        }
    }
}