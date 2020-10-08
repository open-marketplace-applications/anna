use identity_core::did::{Param, DID};
use yew::{html, Callback, Component, ComponentLink, Html, InputData, Properties, ShouldRender};

use anyhow::Error;
use yew::{
    format::{Json, Nothing},
    services::fetch::{FetchService, FetchTask, Request, Response},
};

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug, Clone)]
pub struct RegisterResponse {
    message: String,
}

#[derive(Debug)]
pub enum Msg {
    Search,
    UpdateDID(String),
    ReceiveResponse(Result<RegisterResponse, anyhow::Error>),
}

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct Props {}

pub type FetchResponse<T> = Response<Json<Result<T, Error>>>;
type FetchCallback<T> = Callback<FetchResponse<T>>;

pub struct ConnectPod {
    props: Props,
    link: ComponentLink<Self>,
    url: String,
    fetch_task: Option<FetchTask>,
}
impl Component for ConnectPod {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            props,
            link,
            url: "http://127.0.0.1:8080/register".to_string(),
            fetch_task: None,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Search => {
                log::info!("Connect");

                // let dump = Json(&register);
                let john = json!({
                    "username": "John Doe"
                });
                 // 1. build the request
                 let request = Request::post(self.url.clone())
                 .header("Content-Type", "application/json")
                 .body(Json(&john))
                 .expect("Could not build request.");
             
                // 2. construct a callback
                let callback = self
                .link
                .callback(|response: Response<Json<Result<RegisterResponse, anyhow::Error>>>| {
                    log::info!("response: {:?}", response);
                    let Json(data) = response.into_body();
                    Msg::ReceiveResponse(data)
                });
                // 3. pass the request and callback to the fetch service
                let task = FetchService::fetch(request, callback).expect("failed to start request");
                log::info!("task: {:?}", task);
                // 4. store the task so it isn't canceled immediately
                self.fetch_task = Some(task);

                true
            }
            Msg::UpdateDID(input) => {
                log::info!("UpdateDID: {:?}", input);
                self.url = input;
                true
            },
            Msg::ReceiveResponse(response) => {
                match response {
                    Ok(location) => {
                        log::info!("location: {:?}", location);
                        // self.iss = Some(location);
                    }
                    Err(error) => {
                        log::info!("error: {:?}", error);
                        // self.error = Some(error.to_string())
                    }
                }
                self.fetch_task = None;
                // we want to redraw so that the page displays the location of the ISS instead of
                // 'fetching...'
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
                        placeholder="https://solid_pod_url"
                        value=&self.url
                        oninput=self.link.callback(|e: InputData| Msg::UpdateDID(e.value))
                    />
                </div>

                <button
                    disabled=self.url.is_empty()
                    onclick=self.link.callback(|_| Msg::Search)
                >
                    { "Search" }
                </button>
            </>
        }
    }
}
