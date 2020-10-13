use css_in_rust::Style;
use yew::{html, prelude::*, Classes, Component, ComponentLink, Html, Properties, ShouldRender};

#[derive(Debug)]
pub struct Layout {
    style: Style,
    props: Props,
}

#[derive(Debug)]
pub enum Msg {}

#[derive(Properties, Clone, PartialEq, Debug)]
pub struct Props {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub class: String,
}

impl Component for Layout {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        let style = Style::create(
            String::from("site-layout"),
            String::from(
                r#"
                display: flex;
                flex-direction: column;
                min-height: 100vh;
                .content {
                    flex-grow: 1;
                    padding: 130px 50px 50px 50px;
                }
                "#,
            ),
        )
        .expect("An error occured while creating the style.");
        Self { props, style }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
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
                <div class="content">
                    { self.props.children.clone() }
                </div>
            </div>
        }
    }
}
