use css_in_rust::Style;
use yew::{html, Component, ComponentLink, Html, ShouldRender};

/// Scanner page for QR Code Reader.
pub struct Scanner {
    style: Style,
}

pub enum Msg {}

impl Component for Scanner {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        let style = Style::create(
            String::from("home-page"),
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
        html! {
            <div class=self.style.to_string()>
                <h1>{"Scanner Page"}</h1>
                <p>{"Scanner Page Content"}</p>
            </div>
        }
    }
}
