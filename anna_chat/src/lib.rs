use css_in_rust::Style;
use yew::{html, Component, ComponentLink, Html, ShouldRender};

/// Chat.
pub struct Chat {
    style: Style,
}

pub enum Msg {}

impl Component for Chat {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        let style = Style::create(
            String::from("chat"),
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
                <h1>{"Chat Page"}</h1>
                <p>{"Chat Page Content from external crate"}</p>
            </div>
        }
    }
}
