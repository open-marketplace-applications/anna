use crate::types::CartProduct;
use yew::prelude::*;
use yewtil::NeqAssign;
use yew_state::{GlobalHandle, SharedStateComponent};

pub struct Model {
    cart_products: GlobalHandle<Vec<CartProduct>>,
}

#[derive(Properties, Clone)]
pub struct Props {
    pub cart_products: Vec<CartProduct>,
}

impl Component for Model {
    type Message = ();
    type Properties = GlobalHandle<Vec<CartProduct>>;

    fn create(cart_products: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self { cart_products }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, handle: Self::Properties) -> ShouldRender {
        self.cart_products.neq_assign(handle)
    }

    fn view(&self) -> Html {
        let cart_value = &self
            .cart_products
            .state()
            .iter()
            .fold(0.0, |acc, cp| acc + (cp.quantity as f64 * cp.product.price));
        log::info!("self.handle.state(): {:?}", self.cart_products.state());

        html! {
            <div class="navbar">
                <div class="navbar_cart_value">{format!("Your cart: ${:.2}", cart_value)}</div>
            </div>
        }
    }
}

pub type Navbar = SharedStateComponent<Model>;
