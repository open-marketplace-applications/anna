use crate::models::product::Product;

use yew::prelude::*;

pub struct ProductCard {
    props: Props,
    link: ComponentLink<Self>,
}

pub enum Msg {
    Publish,
}

#[derive(Properties, Clone, Debug)]
pub struct Props {
    pub product: Product,
    #[prop_or_default]
    pub on_add_to_cart: Callback<Product>,
}

impl Component for ProductCard {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { props, link }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Publish => {
                Product::publish(self.props.product.to_owned());
            }
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        let onclick = self.link.callback(|_| Msg::Publish);

        html! {
            <div class="product_card_container">
                    <img class="product_card_image" src={&self.props.product.image}/>
                    <div class="product_card_name">{&self.props.product.name}</div>
                    <div class="product_card_price">{"€"}{&self.props.product.price.amount}</div>
                <button onclick = onclick>{"Publish"}</button>
            </div>
        }
    }
}
