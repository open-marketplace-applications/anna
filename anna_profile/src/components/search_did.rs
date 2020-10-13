use identity_core::did::{Param, DID};
use yew::{html, Callback, Component, ComponentLink, Html, InputData, Properties, ShouldRender};

#[derive(Debug)]
pub enum Msg {
    Search,
    UpdateDID(String),
}

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct Props {}

pub struct SearchDID {
    props: Props,
    link: ComponentLink<Self>,
    did: DID,
}
impl Component for SearchDID {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let did = DID {
            method_name: "iota".into(),
            id_segments: vec!["".into()],
            ..Default::default()
        };

        Self { props, link, did }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Search => {
                log::info!("Search");
                true
            }
            Msg::UpdateDID(value) => {
                let did = DID {
                    method_name: "iota".into(),
                    id_segments: vec![value.into()],
                    ..Default::default()
                };
                self.did = did;
                true
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
        html! {
            <>
                <div class="search-did">
                    <input
                        class=("input-did")
                        placeholder="did:iota:xyz"
                        value=&self.did
                        oninput=self.link.callback(|e: InputData| Msg::UpdateDID(e.value))
                    />
                </div>

                <button
                    disabled=self.did.to_string().is_empty()
                    onclick=self.link.callback(|_| Msg::Search)
                >
                    { "Search" }
                </button>
            </>
        }
    }
}
