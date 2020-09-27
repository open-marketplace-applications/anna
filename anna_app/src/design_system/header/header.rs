use crate::router::AppRoutes;
use css_in_rust::Style;
use yew::agent::{Bridge, Bridged};
use yew::{html, Classes, Component, ComponentLink, Html, Properties, ShouldRender};
use yew_router::prelude::*;

/// Header with menu and user controls.
pub struct Header {
    props: Props,
    style: Style,
}

#[derive(Properties, Clone, PartialEq, Debug)]
pub struct Props {
    #[prop_or_default]
    pub class: String,
}

#[derive(PartialEq, Debug)]
pub enum Msg {}

impl Component for Header {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let style =
            Style::create("header", include_str!("header.scss")).expect("An error occured while creating the style.");
        Self { props, style }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props != props {
            self.props = props;
            true
        } else {
            false
        }
    }

    fn view(&self) -> Html {
        html! {
            <div class=Classes::from(self.props.class.to_string()).extend(self.style.to_string())>
                <div class="menu">
                    <RouterAnchor<AppRoutes> route=AppRoutes::Home>
                        { "Home" }
                    </RouterAnchor<AppRoutes>>
                    <RouterAnchor<AppRoutes> route=AppRoutes::Products>
                        { "Marketplace" }
                    </RouterAnchor<AppRoutes>>
                    <RouterAnchor<AppRoutes> route=AppRoutes::Shop>
                        { "Shop" }
                    </RouterAnchor<AppRoutes>>
                    <RouterAnchor<AppRoutes> route=AppRoutes::Scanner>
                        { "Scanner" }
                    </RouterAnchor<AppRoutes>>
                    <RouterAnchor<AppRoutes> route=AppRoutes::Profile>
                        { "Profile" }
                    </RouterAnchor<AppRoutes>>
                    <RouterAnchor<AppRoutes> route=AppRoutes::ShoppingCart>
                        { "Cart" }
                    </RouterAnchor<AppRoutes>>
                    <RouterAnchor<AppRoutes> route=AppRoutes::ChatModel>
                        { "Chat" }
                    </RouterAnchor<AppRoutes>>
                </div>
            </div>
        }
    }
}
