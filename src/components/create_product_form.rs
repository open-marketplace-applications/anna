use crate::types::Product;
use yew::{html, Callback, Component, ComponentLink, Html, InputData, Properties, ShouldRender};

#[derive(Debug)]
pub enum Msg {
    UpdateName(String),
    UpdateDescription(String),
    Add,
    Abort,
}

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct Props {
    pub on_add: Callback<Product>,
    pub on_abort: Callback<()>,
}

pub struct CreateProductForm {
    props: Props,
    link: ComponentLink<Self>,
    product: Product,
}
impl Component for CreateProductForm {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            props,
            link,
            product: Product::default(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        let product = &mut self.product;
        match msg {
            Msg::UpdateName(value) => {
                product.name = value;
                true
            }
            Msg::UpdateDescription(value) => {
                product.description = value;
                true
            }
            Msg::Add => {
                self.props.on_add.emit(std::mem::take(product));
                true
            }
            Msg::Abort => {
                self.props.on_abort.emit(());
                false
            }
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props == props {
            false
        } else {
            self.props = props;
            true
        }
    }

    fn view(&self) -> Html {
        let Self { link, product, .. } = self;
        html! {
            <>
                <div class="names">
                    <input
                        class=("new-product", "name")
                        placeholder="name"
                        value=&product.name
                        oninput=link.callback(|e: InputData| Msg::UpdateName(e.value))
                    />
                    <textarea
                        class=("new-product", "description")
                        placeholder="Description"
                        value=&product.description
                        oninput=link.callback(|e: InputData| Msg::UpdateDescription(e.value))
                    />
                </div>

                <button
                    disabled=product.name.is_empty() || product.description.is_empty()
                    onclick=link.callback(|_| Msg::Add)
                >
                    { "Add New" }
                </button>
                <button onclick=link.callback(|_| Msg::Abort)>
                    { "Go Back" }
                </button>
            </>
        }
    }
}