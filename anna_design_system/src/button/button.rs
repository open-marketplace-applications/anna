use css_in_rust::Style;
use yew::prelude::*;

use yew_styles::button::{Button as Btn, Props, Msg};

pub struct Button {
    link: ComponentLink<Self>,
    // title: String,
    // onsignal: Callback<()>,
    // color: String,
    // style: Style,
    props: Props,
}

// #[derive(Debug)]
// pub enum Msg {
//     Clicked,
// }

// #[derive(Clone, PartialEq, Properties, Debug)]
// pub struct Props {
//     #[prop_or_default]
//     pub title: String,
//     pub color: String,
//     pub onsignal: Callback<()>,
//     #[prop_or_default]
//     pub class: String,
// }

impl Component for Button {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        // let style =
        //     Style::create("button", include_str!("button.scss")).expect("An error occured while creating the style.");

        Button {
            props,
            link
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Clicked(mouse_event) => {
                self.props.onclick_signal.emit(mouse_event);
            }
        };
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {

        true
    }

    fn view(&self) -> Html {
        html! {
            <Btn
                onclick_signal=self.link.callback(Msg::Clicked)
            > { self.props.children.clone() }
            </Btn>
        }
    }
}
