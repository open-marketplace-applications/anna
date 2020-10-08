use css_in_rust::Style;
use yew::prelude::*;
use yew::{html, Classes, Component, ComponentLink, Html, Properties, ShouldRender};

#[derive(PartialEq, Debug, Clone)]
pub enum IconType {
    Label,
    Scan,
}

fn inspect(event: IconType) -> String {
    match event {
        IconType::Label => "label.svg".to_string(),
        IconType::Scan => "scan.svg".to_string(),
    } 
}

#[derive(Debug)]
pub struct Icon {
    props: Props,
    style: Style,
    url: String,
} 

#[derive(Clone, PartialEq, Properties, Debug)]
pub struct Props {
    #[prop_or_default]
    pub class: String,
    #[prop_or_default]
    pub children: Children,
    pub icon: IconType,
}

#[derive(PartialEq, Debug)]
pub enum Msg {}

impl Component for Icon {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let style =
            Style::create("icon", include_str!("icon.scss")).expect("An error occured while creating the style.");

        let url = inspect(props.icon.clone());

        Self { props, style, url }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
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
            <div
                class=Classes::from(self.props.class.to_string()).extend(self.style.to_string())
            >
              
              { self.url.clone() } 
              { self.props.children.clone() }
            </div>
        }
    }
}
