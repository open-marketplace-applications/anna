use yew::prelude::*;

pub enum Msg {
    SendOrder,
    UpdateFirstName(String),
    UpdateLastName(String),
}


#[derive(Debug)]
pub struct OrderForm {
    link: ComponentLink<Self>,
    first_name: String,
    last_name: String,
}

impl Component for OrderForm {
    type Message = Msg;
    type Properties = ();

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { 
            link,
            first_name: "".into(),
            last_name: "".into(),
         }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::SendOrder => {
                log::info!("order_form::SendOrder {:?}", self);
            },
            Msg::UpdateFirstName(val) => {
                log::info!("order_form::UpdateFirstName {:?}", val);
            },
            Msg::UpdateLastName(val) => {
                log::info!("order_form::UpdateLastName {:?}", val);
            }
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        html! {
            <div>
                <input class="firstname"
                    placeholder="First name"
                    value=&self.first_name
                    oninput=self.link.callback(|e: InputData| Msg::UpdateFirstName(e.value)) />
                <input 
                    class="lastname"
                    placeholder="Last name"
                    value=&self.last_name
                    oninput=self.link.callback(|e: InputData| Msg::UpdateLastName(e.value)) />
                <textarea rows=5
                    placeholder="placeholder">
                </textarea>
                <datepicker>
                </datepicker>
                <button onclick=self.link.callback(|_| Msg::SendOrder)>{ "Buy now" }</button>
            </div>

        }
    }
}


