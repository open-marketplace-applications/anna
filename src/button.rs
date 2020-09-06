use yew::prelude::*;

#[derive(Debug)]
pub struct Button {
    link: ComponentLink<Self>,
    title: String,
    onsignal: Callback<()>,
    color: String
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
}

impl Component for Button {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Button {
            link,
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
        let styled_button = String::from("
            padding: 16px 24px;
            border-radius: var(--radius);
            outline: none;
            margin-right: 20px;
            border: var(--border);
            font-weight: var(--font-bold);
            font-size: 16px;
            color: var(--color-white); 
            background-color: var(--color-primary);
            transition: var(--transition);
            &:hover {
                background-color: var(--color-primary-darken);
            }
        ",);

        html! {
            <button style=styled_button onclick=self.link.callback(|_| Msg::Clicked)>
                { &self.title }
            </button>
        }
    }
}