// newbutton.rs
use yew::prelude::*;
use yew_state::{GlobalHandle, SharedStateComponent, SharedState};
use crate::types::{CartProduct, Product};

pub struct Model {
    props: Props,
    link: ComponentLink<Self>
}

#[derive(Properties, Clone)]
pub struct Props {
    pub product: Product,
    #[prop_or_default]
    pub on_add_to_cart: Callback<Product>,
    #[prop_or_default]
    handle: GlobalHandle<Vec<CartProduct>>
}

impl SharedState for Props {
    type Handle = GlobalHandle<Vec<CartProduct>>;

    fn handle(&mut self) -> &mut Self::Handle {
        &mut self.handle
    }
}

pub enum Msg {
    AddToCart,
}

impl Component for Model {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { props, link}
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::AddToCart => {
                let state = self.props.handle.state();

                let cart_product = self
                    .props
                    .handle
                    .state()
                    .iter()
                    .find(|&x| x.product.id == self.props.product.id);
                

                if let Some(cp) = cart_product {
                    // product found int cart, increment the quantity

                    // clone stuff
                    let mut temp_obj = cp.clone();
                    let mut temp_arr = state.clone();

                    // find object in array and remove it
                    let index = temp_arr.iter().position(|x| *x == temp_obj).unwrap();
                    temp_arr.remove(index);

                    // set the quantity plus one
                    temp_obj.quantity += 1;
                    // add the edited object to the array
                    temp_arr.push(temp_obj);
                    // update the state
                    self.props.handle.reduce(move |state| *state = temp_arr.clone());

                } else {
                    // product not found in card, add it.
                    let mut temp_arr = state.clone();
                    // add the new object to the cart
                    temp_arr.push(CartProduct {
                        product: self.props.product.clone(),
                        quantity: 1,
                    });
                    // update the state
                    self.props.handle.reduce(move |state| *state = temp_arr.clone());
                }

                // emit to parent
                self.props.on_add_to_cart.emit(self.props.product.clone())
            },
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        let onclick = self.link.callback(|_| Msg::AddToCart);
        html! {
            <button onclick = onclick>{"Add To Cart"}</button>
        }
    }
}

pub type AddToCartButton = SharedStateComponent<Model>;