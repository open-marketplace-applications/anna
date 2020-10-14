use css_in_rust::Style;
use yew::{html, prelude::*, Classes, Component, ComponentLink, Html, Properties, ShouldRender};

#[derive(Debug)]
pub struct H1 {
    props: Props,
    style: Style,
}

#[derive(Clone, PartialEq, Properties, Debug)]
pub struct Props {
    #[prop_or_default]
    pub class: String,
    #[prop_or_default]
    pub children: Children,
}

#[derive(PartialEq, Debug)]
pub enum Msg {}

impl Component for H1 {
  type Message = Msg;
  type Properties = Props;

  fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
    let style =
        Style::create("h1", include_str!("h1.scss")).expect("An error occured while creating the style.");
    Self { props, style }
  }

  fn update(&mut self, _msg: Self::Message) -> ShouldRender {
    true
  }

  fn change(&mut self, props: Self::Properties) -> ShouldRender {
    true
  }

  fn view(&self) -> Html {
    html! {
      <div
        class=Classes::from(self.props.class.to_string()).extend(self.style.to_string())
      >
        { self.props.children.clone() }
      </div>
    }
  }
}
