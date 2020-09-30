use crate::router::{AppRoutes, Router};
use crate::types::CartProduct;
use yew::prelude::*;
use yew_router::prelude::*;
use yew_state::{GlobalHandle, SharedStateComponent};

// 📚 Design System
use anna_design_system::{Header, Logo, Menu, MenuItem};

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
            <Logo>
              <RouterAnchor<AppRoutes> route=AppRoutes::Home>
                {"ANNA"}
              </RouterAnchor<AppRoutes>>
            </Logo>

            <MenuItem>
              <RouterAnchor<AppRoutes> route=AppRoutes::Products>
                {"Marketplace"}
              </RouterAnchor<AppRoutes>>
            </MenuItem>

            <MenuItem>
              <RouterAnchor<AppRoutes> route=AppRoutes::Shop>
                  { "Shop" }
              </RouterAnchor<AppRoutes>>
            </MenuItem>

            <MenuItem>
              <RouterAnchor<AppRoutes> route=AppRoutes::Scanner>
                  { "Scanner" }
              </RouterAnchor<AppRoutes>>
            </MenuItem>

            <MenuItem>
              <RouterAnchor<AppRoutes> route=AppRoutes::Profile>
                  { "Profile" }
              </RouterAnchor<AppRoutes>>
            </MenuItem>

            <MenuItem>
              <RouterAnchor<AppRoutes> route=AppRoutes::ShoppingCart>
                  { "Cart" }
              </RouterAnchor<AppRoutes>>
            </MenuItem>

            <MenuItem>
              <RouterAnchor<AppRoutes> route=AppRoutes::ChatModel>
                  { "Chat" }
              </RouterAnchor<AppRoutes>>
            </MenuItem>
          </Menu>
        </Header>
    }
  }
}
