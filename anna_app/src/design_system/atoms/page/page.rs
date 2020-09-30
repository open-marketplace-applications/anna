use css_in_rust::Style;
use yew::prelude::*;

#[derive(Debug)]
pub struct Page {
  link: ComponentLink<Self>,
  style: Style,
  props: Props,
}

#[derive(Debug)]
pub enum Msg {}

#[derive(Clone, PartialEq, Properties, Debug)]
pub struct Props {
  #[prop_or_default]
  pub children: Children,
  #[prop_or_default]
  pub class: String,
}

impl Component for Page {
  type Message = Msg;
  type Properties = Props;

  fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
    log::info!("main");
    let style = Style::create("page", include_str!("style.scss")).expect("An error occured while creating the style.");
    log::info!("main");
    log::info!("öpggoo: {:?}", style);
    Page {
      link,
      style,
      props: props.to_owned(),
    }
  }

  fn update(&mut self, msg: Self::Message) -> ShouldRender {
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
        <h1>{"Hello world"}</h1>
        { self.props.children.clone() }
      </div>
    }
  }
}
