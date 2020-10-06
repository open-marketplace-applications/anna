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
                <svg
                  xmlns="http://www.w3.org/2000/svg" width="40" height="40.027" viewBox="0 0 40 40.027">
                  <g id="Gruppe_17105" data-name="Gruppe 17105" transform="translate(-24 -125)">
                    <path id="Pfad_1847" data-name="Pfad 1847" d="M1254.739,1356.85h0a99.1,99.1,0,0,1-17.41.071l-.844-.071a10.248,10.248,0,0,1-10.248-10.248h0a109.234,109.234,0,0,1,0-18.692v-.01a10.248,10.248,0,0,1,10.248-10.248l.389-.033a106.963,106.963,0,0,1,18.313.033h0a10.248,10.248,0,0,1,10.248,10.248h0a109.313,109.313,0,0,1,0,18.7v0h0A10.834,10.834,0,0,1,1254.739,1356.85Z" transform="translate(-1201.836 -1192.242)" fill="#fff"/>
                    <circle id="Ellipse_4" data-name="Ellipse 4" cx="2.3" cy="2.3" r="2.3" transform="translate(31.698 154.633) rotate(-83.677)" fill="#0f282b"/>
                    <circle id="Ellipse_5" data-name="Ellipse 5" cx="2.3" cy="2.3" r="2.3" transform="matrix(0.11, -0.994, 0.994, 0.11, 51.615, 154.642)" fill="#0f282b"/>
                    <circle id="Ellipse_6" data-name="Ellipse 6" cx="2.3" cy="2.3" r="2.3" transform="matrix(0.11, -0.994, 0.994, 0.11, 41.664, 137.389)" fill="#0f282b"/>
                    <path id="Pfad_71" data-name="Pfad 71" d="M190.983,81.3a3.957,3.957,0,0,1,.729.242,7.669,7.669,0,0,0-4.654-6.841,3.074,3.074,0,0,1-.446,2.338,5.366,5.366,0,0,1,2.8,4.564A1.941,1.941,0,0,1,190.983,81.3Z" transform="translate(-139.847 60.017)" fill="#0f282b"/>
                    <path id="Pfad_72" data-name="Pfad 72" d="M207.782,132.343a1.206,1.206,0,0,1,1.452-.894,1.341,1.341,0,0,1,.859.483,1.419,1.419,0,0,1,.035.969,1.206,1.206,0,1,1-2.346-.558Z" transform="translate(-158.269 10.588)" fill="#0f282b"/>
                    <path id="Pfad_73" data-name="Pfad 73" d="M108.185,132.85a1.206,1.206,0,0,1-1.5-.81,1.343,1.343,0,0,1-.012-.986,1.419,1.419,0,0,1,.822-.515,1.206,1.206,0,1,1,.69,2.311Z" transform="translate(-70.1 11.396)" fill="#0f282b"/>
                    <path id="Pfad_74" data-name="Pfad 74" d="M103.582,226.053a2.006,2.006,0,0,1-.629-1.532,5.368,5.368,0,0,1-5.251-.278,2.993,2.993,0,0,1-1.813,1.536,7.671,7.671,0,0,0,8.249.72A3.992,3.992,0,0,1,103.582,226.053Z" transform="translate(-60.781 -70.313)" fill="#0f282b"/>
                    <path id="Pfad_75" data-name="Pfad 75" d="M159.162,217.892a1.206,1.206,0,0,1,.049,1.7,1.342,1.342,0,0,1-.848.5,1.42,1.42,0,0,1-.857-.454,1.206,1.206,0,1,1,1.656-1.753Z" transform="translate(-114.152 -64.491)" fill="#0f282b"/>
                    <path id="Pfad_76" data-name="Pfad 76" d="M111.611,75.316a3.08,3.08,0,0,1,.067-.639,7.67,7.67,0,0,0-4.66,6.755,3.887,3.887,0,0,1,.642-.25,1.949,1.949,0,0,1,1.661.259,5.367,5.367,0,0,1,2.8-4.425A3.058,3.058,0,0,1,111.611,75.316Z" transform="translate(-70.48 60.037)" fill="#0f282b"/>
                    <path id="Pfad_77" data-name="Pfad 77" d="M73.295,145.525a2.981,2.981,0,0,1,1.69-.394,5.367,5.367,0,0,1,2.488-4.747,1.948,1.948,0,0,1-1.052-1.213,3.989,3.989,0,0,1-.155-.746,7.671,7.671,0,0,0-3.52,7.506A3,3,0,0,1,73.295,145.525Z" transform="translate(-40.554 4.48)" fill="#0f282b"/>
                    <path id="Pfad_78" data-name="Pfad 78" d="M180.825,225.619a2.993,2.993,0,0,1-1.191-1.271,5.367,5.367,0,0,1-5.266.25,2,2,0,0,1-.547,1.465,4.055,4.055,0,0,1-.6.532,7.671,7.671,0,0,0,8.224-.707A3.011,3.011,0,0,1,180.825,225.619Z" transform="translate(-128.177 -70.405)" fill="#0f282b"/>
                    <path id="Pfad_79" data-name="Pfad 79" d="M225.227,138.462a3.738,3.738,0,0,1-.107.751,1.941,1.941,0,0,1-1,1.276,5.368,5.368,0,0,1,2.381,4.669,3.007,3.007,0,0,1,2.239.8A7.671,7.671,0,0,0,225.227,138.462Z" transform="translate(-172.542 4.447)" fill="#0f282b"/>
                  </g>
                </svg>
              </RouterAnchor<AppRoutes>>
            </Logo>

            <MenuItem>
              <RouterAnchor<AppRoutes> route=AppRoutes::Products>
                <svg
                  xmlns="http://www.w3.org/2000/svg" width="13.933" height="12.648" viewBox="0 0 13.933 12.648">
                  <g id="Gruppe_17107" data-name="Gruppe 17107" transform="translate(-14.113 -13.642)">
                    <path id="Pfad_300" data-name="Pfad 300" d="M11.04,23.224H6.694a1.038,1.038,0,0,0-.734.3L.3,29.184a1.038,1.038,0,0,0,0,1.468L4.65,35a1.038,1.038,0,0,0,1.468,0l5.657-5.656a1.041,1.041,0,0,0,.3-.735V24.262A1.038,1.038,0,0,0,11.04,23.224ZM9.224,26.857A.778.778,0,1,1,10,26.078.779.779,0,0,1,9.224,26.857Z" transform="translate(14.113 -9.582)" fill="#fff"/>
                    <path id="Pfad_301" data-name="Pfad 301" d="M225,55.224v4.831a.9.9,0,0,1-.265.638l-5.748,5.748.088.088a1.038,1.038,0,0,0,1.468,0l5.191-5.19a1.037,1.037,0,0,0,.3-.734V56.262A1.038,1.038,0,0,0,225,55.224Z" transform="translate(-197.993 -40.544)" fill="#fff"/>
                  </g>
                </svg>
                <span>{"Discover"}</span>
              </RouterAnchor<AppRoutes>>
            </MenuItem>

            <MenuItem>
              <RouterAnchor<AppRoutes> route=AppRoutes::Shop>
                  <span>{ "Shop" }</span>
              </RouterAnchor<AppRoutes>>
            </MenuItem>

            <MenuItem>
              <RouterAnchor<AppRoutes> route=AppRoutes::Scanner>
                  <svg xmlns="http://www.w3.org/2000/svg" width="17.21" height="17.209" viewBox="0 0 17.21 17.209">
                  <g id="name_tag" transform="translate(-4 -4.001)">
                    <path id="Pfad_15432" data-name="Pfad 15432" d="M16.191,18.343H9.02a2.154,2.154,0,0,1-2.151-2.151V9.021A2.154,2.154,0,0,1,9.02,6.869H16.19a2.154,2.154,0,0,1,2.151,2.151v7.171A2.153,2.153,0,0,1,16.191,18.343ZM4.717,8.3a.717.717,0,0,0,.717-.717V6.152a.718.718,0,0,1,.717-.717H7.585A.717.717,0,0,0,7.585,4H6.151A2.154,2.154,0,0,0,4,6.152V7.586A.717.717,0,0,0,4.717,8.3ZM19.059,4H17.625a.717.717,0,1,0,0,1.434h1.434a.718.718,0,0,1,.717.717V7.586a.717.717,0,0,0,1.434,0V6.152A2.154,2.154,0,0,0,19.059,4ZM7.585,19.776H6.151a.718.718,0,0,1-.717-.717V17.625a.717.717,0,1,0-1.434,0v1.434A2.154,2.154,0,0,0,6.151,21.21H7.585a.717.717,0,0,0,0-1.434Zm12.907-2.868a.716.716,0,0,0-.717.717v1.434a.717.717,0,0,1-.717.717H17.625a.717.717,0,1,0,0,1.434h1.434a2.154,2.154,0,0,0,2.151-2.151V17.625A.716.716,0,0,0,20.493,16.908Z" fill="#fff"/>
                  </g>
                </svg>
              </RouterAnchor<AppRoutes>>
            </MenuItem>


            <MenuItem>
              <RouterAnchor<AppRoutes> route=AppRoutes::ShoppingCart>
                  <svg
                    xmlns="http://www.w3.org/2000/svg" width="17.13" height="15.701" viewBox="0 0 17.13 15.701">
                    <path id="buy" d="M16.981,24.463a.7.7,0,0,0-.555-.271H3.033l-.2-2.218a.7.7,0,0,0-.7-.641H.7a.7.7,0,1,0,0,1.407H1.49l.771,8.591c0,.01,0,.02,0,.03a.708.708,0,0,0,.01.079l.028.314,0,.018A2.708,2.708,0,0,0,3.312,33.59,2.132,2.132,0,1,0,7,34.176h3.851a2.133,2.133,0,1,0,2.006-1.407H5a1.307,1.307,0,0,1-1.2-.792l11.243-.661a.7.7,0,0,0,.641-.532l1.43-5.718A.7.7,0,0,0,16.981,24.463Z" transform="translate(0 -21.334)" fill="#fff"/>
                  </svg>
              </RouterAnchor<AppRoutes>>
            </MenuItem>

            <MenuItem>
              <RouterAnchor<AppRoutes> route=AppRoutes::ChatModel>
                  <svg
                    xmlns="http://www.w3.org/2000/svg" width="18.181" height="17.896" viewBox="0 0 18.181 17.896">
                    <g id="conversation" transform="translate(0 -4.011)">
                      <g id="Gruppe_17052" data-name="Gruppe 17052" transform="translate(0 12.603)">
                        <g id="Gruppe_17051" data-name="Gruppe 17051" transform="translate(0)">
                          <path id="Pfad_15425" data-name="Pfad 15425" d="M4.023,251.3a8.7,8.7,0,0,1-2.578-5.334,5.556,5.556,0,0,0-1.2,3.471,5.62,5.62,0,0,0,.54,2.393l-.727,2.117A1,1,0,0,0,1,255.266a1.006,1.006,0,0,0,.328-.055l2.117-.727a5.62,5.62,0,0,0,2.393.54,5.558,5.558,0,0,0,3.528-1.249A8.71,8.71,0,0,1,4.023,251.3Z" transform="translate(0 -245.961)" fill="#fff"/>
                        </g>
                      </g>
                      <g id="Gruppe_17054" data-name="Gruppe 17054" transform="translate(2.48 4.011)">
                        <g id="Gruppe_17053" data-name="Gruppe 17053" transform="translate(0 0)">
                          <path id="Pfad_15426" data-name="Pfad 15426" d="M85.477,18.138l-1.056-3.073A7.688,7.688,0,0,0,77.646,4.012a7.68,7.68,0,1,0-.124,15.358h.012a7.7,7.7,0,0,0,3.362-.78l3.073,1.056a1.2,1.2,0,0,0,.389.065,1.189,1.189,0,0,0,1.119-1.574Zm-7.214-3.782H74.3a.541.541,0,0,1,0-1.082h3.965a.541.541,0,0,1,0,1.082Zm2.482-2.225H74.3a.541.541,0,1,1,0-1.082h6.447a.541.541,0,0,1,0,1.082Zm0-2.225H74.3a.541.541,0,1,1,0-1.082h6.447a.541.541,0,0,1,0,1.082Z" transform="translate(-69.842 -4.011)" fill="#fff"/>
                        </g>
                      </g>
                    </g>
                  </svg>
              </RouterAnchor<AppRoutes>>
            </MenuItem>

            <MenuItem>
              <RouterAnchor<AppRoutes> route=AppRoutes::Profile>
                  <span>{ "Profile" }</span>
              </RouterAnchor<AppRoutes>>
            </MenuItem>
          </Menu>
        </Header>
    }
  }
}
