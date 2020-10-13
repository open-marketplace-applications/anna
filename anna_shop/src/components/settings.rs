use yew::prelude::*;

pub struct ShopModel {
    name: String,
}
pub struct Settings {
    props: Props,
    link: ComponentLink<Self>,
    shop: ShopModel,
}

pub enum Msg {
    Publish,
    UpdateShopName(String),
}

#[derive(Properties, Clone)]
pub struct Props {}

impl Component for Settings {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            props,
            link,
            shop: ShopModel { name: "My shop".into() },
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Publish => {
                log::info!("Publish");
            }
            Msg::UpdateShopName(name) => {
                log::info!("UpdateShopName {:?}", name);
                self.shop.name = name;
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
            <div class="settings">
                <h2>{"Settings"}</h2>
                <input
                    placeholder="Shop name"
                    value=&self.shop.name
                    oninput=self.link.callback(|e: InputData| Msg::UpdateShopName(e.value))
                />
                <button
                    disabled=self.shop.name.is_empty()
                    onclick= onclick
                >
                    { "Update Name" }
                </button>
            </div>
        }
    }
}
