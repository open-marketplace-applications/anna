use yew::prelude::*;
use css_in_rust::Style;

#[derive(Debug)]
pub struct Button {
    link: ComponentLink<Self>,
    title: String,
    onsignal: Callback<()>,
    color: String,
    style: Style,
    props: Props,
}

#[derive(Debug)]
pub enum Msg {
    Clicked,
}

#[derive(Clone, PartialEq, Properties, Debug)]
pub struct Props {
    #[prop_or_default]
    pub title: String,
    pub color: String,
    pub onsignal: Callback<()>,
    #[prop_or_default]
    pub class: String,
}

impl Component for Button {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let style = Style::create("button", include_str!("button.scss"))
            .expect("An error occured while creating the style.");

        Button {
            link,
            style,
            props: props.to_owned(),
            title: props.title,
            color: props.color,
            onsignal: props.onsignal,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        log::info!("button::update::self {:?}", self);
        match msg {
            Msg::Clicked => {
                self.onsignal.emit(());
            }
        }
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.title = props.title;
        self.onsignal = props.onsignal;
        self.color = props.color;
        true
    }

    fn view(&self) -> Html {

        html! {
            <button 
                class=Classes::from(self.props.class.to_string()).extend(self.style.to_string()) 
                onclick=self.link.callback(|_| Msg::Clicked)>
                { &self.title }
            </button>
        }
    }
}