use yew::{html, Component, ComponentLink, Html, ShouldRender};
use wasm_bindgen::prelude::*;
use css_in_rust::Style;
use anna_design_system::{Page, Section, Container, H1};

#[derive(Debug)]
pub struct Home {
    style: Style,
}

pub enum Msg {}

impl Component for Home {
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
            <Page>
                <Section class="light">
                    <Container>
                        <H1>{"Open Marketplace"}</H1>
                        <h2>{"Spotlight"}</h2>
                        <p>{"einfachIOTA Magazine 2 Vorverkauf"}</p>
                        <a href="/cart">{"Jetzt vorbestellen!"}</a>
                    </Container>
                </Section>
            </Page>
        }
    }
}
