use crate::router::AppRoutes;
use yew::prelude::*;
use yew_router::prelude::*;
use yew_state::{GlobalHandle, SharedStateComponent};

// 📚 Design System
use design_system::{Header, Logo, Menu, MenuItem, Tooltip};

pub struct Nav {
    props: Props,
}

#[derive(Properties, Clone)]
pub struct Props {}

impl Component for Nav {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self { props }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, handle: Self::Properties) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        html! {
            <Header>
              <Menu>
                <RouterAnchor<AppRoutes> route=AppRoutes::Home>
                  <Logo />
                </RouterAnchor<AppRoutes>>

                <RouterAnchor<AppRoutes> route=AppRoutes::Products>
                  <MenuItem text="Discover" icon="label" />
                </RouterAnchor<AppRoutes>>

                <RouterAnchor<AppRoutes> route=AppRoutes::Shop>
                  <MenuItem text="Shop" />
                </RouterAnchor<AppRoutes>>
              </Menu>

              <Menu>
                <RouterAnchor<AppRoutes> route=AppRoutes::Scanner>
                  <MenuItem icon="scan">
                    <Tooltip title={"Scan QR Code"}></Tooltip>
                  </MenuItem>
                </RouterAnchor<AppRoutes>>

                <RouterAnchor<AppRoutes> route=AppRoutes::Cart>
                  <MenuItem icon="cart">
                    <Tooltip title={"Cart"}></Tooltip>
                  </MenuItem>
                </RouterAnchor<AppRoutes>>

                <RouterAnchor<AppRoutes> route=AppRoutes::ChatModel>
                  <MenuItem icon="chat">
                    <Tooltip title={"Messages"}></Tooltip>
                  </MenuItem>
                </RouterAnchor<AppRoutes>>

                <RouterAnchor<AppRoutes> route=AppRoutes::Profile>
                  <MenuItem text="Profile" />
                </RouterAnchor<AppRoutes>>
              </Menu>
            </Header>
        }
    }
}
