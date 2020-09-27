use css_in_rust::Style;
use yew::{html, Component, ComponentLink, Html, ShouldRender};
use identity_core::did::{Param, DID};

/// Profile.
pub struct Profile {
    style: Style,
}

pub enum Msg {}

impl Component for Profile {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        
        let did = DID {
            method_name: "iota".into(),
            id_segments: vec!["123456".into()],
            ..Default::default()
        }
        .init()
        .unwrap();


        log::info!("starting up: DID {}", did.to_string());

        let style = Style::create(
            String::from("profile"),
            String::from(
                r#"
                "#,
            ),
        )
        .expect("An error occured while creating the style.");
        Self { style }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        render(self)
    }
}


pub fn render(context: &Profile) -> Html {
    html! {
        <div class=context.style.to_string()>
            <h1>{"Profile Page"}</h1>
            <p>{"Profile Page Content from external crate"}</p>
        </div>
    }
} 