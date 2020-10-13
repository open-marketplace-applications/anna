use css_in_rust::Style;
use yew::prelude::*;
use yew::{NodeRef, html, Component, ComponentLink, Html, Properties, ShouldRender};

use web_sys::Element;

#[derive(Debug)]
pub struct Icon {
    style: Style,
    props: Props,
    node_ref: NodeRef,
}

#[derive(Clone, PartialEq, Properties, Debug)]
pub struct Props {
    #[prop_or_default]
    pub class: String,
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub name: &'static str,
}

#[derive(PartialEq, Debug)]
pub enum Msg {}

impl Component for Icon {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        let style = Style::create("icon", include_str!("icon.scss")).expect("An error occured while creating the style.");


        Self { style, props, node_ref: NodeRef::default() }
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
            <div 
                class=Classes::from(self.props.class.to_string()).extend(self.style.to_string()) 
                ref=self.node_ref.clone()
            />
        }
    }
    
    fn rendered(&mut self, _first_render: bool) {
        let mut svg = "";

        match self.props.name {
            "label" => svg = include_str!("icons/label.svg"),
            "scan" => svg = include_str!("icons/scan.svg"),
            "chat" => svg = include_str!("icons/chat.svg"),
            "cart" => svg = include_str!("icons/cart.svg"),
            // TODO: ADD Default icon
            _ => println!("Ain't special"),
        }

        let el = self.node_ref.cast::<Element>().unwrap();
        el.set_inner_html(&svg);
    }
}
