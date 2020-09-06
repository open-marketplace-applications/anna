#![recursion_limit = "256"]

use yew::prelude::*;

use crate::button::Button;
use serde::{Deserialize, Serialize};
use yew::services::{DialogService, StorageService};

#[derive(Debug)]
pub struct App {
    link: ComponentLink<Self>,
    scene: Scene,
    name: String,
    description: String
}

#[derive(Debug)]
pub enum Msg {
    StartDiscover,
    AddWebsite,
    SwitchTo(Scene),
    AddNew,
    UpdateUrl(String),
    UpdateDescription(String),
    Clear,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Website {
    url: String,
    description: String,
}

impl Website {
    fn empty() -> Self {
        Website {
            url: "".into(),
            description: "".into(),
        }
    }
}

#[derive(Debug)]
pub enum Scene {
    Home,
    NewWebsiteForm(Website),
    Settings
}


impl App {
    fn render_home(&self, link: &ComponentLink<App>) -> Html {
        let styled_page = String::from("
            height: 100vh;
            width: 100%;
            padding: 20px;
        ",);

        let styled_section = String::from("
            width: 100%;
            max-width: 600px;
            display: flex;
            flex-direction: column;
            justify-content: center;
            text-align: center;
            margin: 0 auto;
        ",);
        html! {
            <div style=styled_page>
                <div style=styled_section>
                    <h1 class="h1">{ &self.name }</h1>
                    <p>{ &self.description }</p>
                    <button onclick=self.link.callback(|_| Msg::StartDiscover)>{ "Discover and earn IOTA" }</button>
                    <button onclick=self.link.callback(|_| Msg::SwitchTo(Scene::NewWebsiteForm(Website::empty())))>{ "Add New" }</button>
                    <button onclick=self.link.callback(|_| Msg::SwitchTo(Scene::Settings))>{ "Settings" }</button>
                </div>
            </div>
        }
    }
    fn render_new_website(&self, link: &ComponentLink<App>, website: &Website) -> Html {
        html! {
            <div class="crm">
                <h1>{"Add a new website"}</h1>
                <div class="names">
                    <div>
                        { website.view_url_input(&self.link) }
                    </div>
                    <div>
                        { website.view_description_textarea(&self.link) }
                    </div>
                </div>
                <button disabled=website.url.is_empty() || website.description.is_empty()
                        onclick=self.link.callback(|_| Msg::AddNew)>{ "Add New" }</button>
                <button onclick=self.link.callback(|_| Msg::SwitchTo(Scene::Home))>{ "Go Back" }</button>
            </div>
        }
    }
    fn render_settings(&self, link: &ComponentLink<App>) -> Html {
        html! {
            <div>
                <h1>{"Settings"}</h1>
                <button onclick=self.link.callback(|_| Msg::Clear)>{ "Clear Database" }</button>
                <button onclick=self.link.callback(|_| Msg::SwitchTo(Scene::Home))>{ "Go Back" }</button>
            </div>
        }
    }
}

impl Component for App {
    type Message = Msg;
    type Properties = ();
    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            scene: Scene::Home,
            name: ("ANNA").into(),
            description: ("This is a beta application of the open marketplace, you can discover websites and earn IOTA. Have fun!").into()
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        log::info!("update::self {:?}", self);
        log::info!("update::msg {:?}", msg);

        let mut new_scene = None;
        match self.scene {
            Scene::Home => match msg {
                Msg::SwitchTo(Scene::NewWebsiteForm(client)) => {
                    new_scene = Some(Scene::NewWebsiteForm(client));
                }
                Msg::SwitchTo(Scene::Settings) => {
                    new_scene = Some(Scene::Settings);
                }
                Msg::StartDiscover => {
                    log::info!("Msg::StartDiscover {:?}", self);
                }
                Msg::AddWebsite => {
                    log::info!("Msg::AddWebsite {:?}", self);
                }
                unexpected => {
                    panic!(
                        "Unexpected message when clients list shown: {:?}",
                        unexpected
                    );
                }
            },
            Scene::NewWebsiteForm(ref mut website) => match msg {
                Msg::UpdateUrl(val) => {
                    println!("Input: {}", val);
                    website.url = val;
                }
                Msg::UpdateDescription(val) => {
                    println!("Input: {}", val);
                    website.description = val;
                }
                Msg::AddNew => {
                    let mut new_website = Website::empty();
                    println!("new_website: {:?}", new_website);
                }
                Msg::SwitchTo(Scene::Home) => {
                    new_scene = Some(Scene::Home);
                }
                unexpected => {
                    panic!(
                        "Unexpected message during new client editing: {:?}",
                        unexpected
                    );
                }
            },
            Scene::Settings => match msg {
                Msg::Clear => {
                    let ok = { DialogService::confirm("Do you really want to clear the data?") };
                    if ok {
                        println!("Clear data: {:?}", self);
                    }
                }
                Msg::SwitchTo(Scene::Home) => {
                    new_scene = Some(Scene::Home);
                }
                unexpected => {
                    panic!("Unexpected message for settings scene: {:?}", unexpected);
                }
            },
        }
        if let Some(new_scene) = new_scene.take() {
            self.scene = new_scene;
        }

        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        // log::info!("change::self: {:?}", self);
        // log::info!("change::_props: {:?}", _props);

        // Should only return "true" if new properties are different to
        // previously received properties.
        // This component has no properties so we will always return "false".
        false
    }

    fn view(&self) -> Html {
        match self.scene {
            Scene::Home => App::render_home(&self, &self.link),
            Scene::NewWebsiteForm(ref website) => App::render_new_website(&self, &self.link, website),
            Scene::Settings => App::render_settings(&self, &self.link),
        }
    }
}


impl Website {

    fn view_url_input(&self, link: &ComponentLink<App>) -> Html {
        html! {
            <input class="new-website url"
                   placeholder="Website url"
                   value=&self.url
                   oninput=link.callback(|e: InputData| Msg::UpdateUrl(e.value)) />
        }
    }

    fn view_description_textarea(&self, link: &ComponentLink<App>) -> Html {
        html! {
            <textarea class=("new-website", "description")
               placeholder="Description"
               value=&self.description
               oninput=link.callback(|e: InputData| Msg::UpdateDescription(e.value)) />
        }
    }    
}
