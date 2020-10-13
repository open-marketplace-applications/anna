use css_in_rust::Style;
use yew::{html, Component, ComponentLink, Html, InputData, MouseEvent, ShouldRender};

use validator::{Validate, ValidationError};
use yew_form::{Field, Form};

use regex::Regex;

use identity_comm::did_comm::TrustPing;
use identity_comm::types::TRUSTPING;
use identity_comm::DIDComm_message;
use identity_common::Timestamp;
use identity_core::did::{Param, DID};

use serde::{Deserialize, Serialize};

use yew::worker::{Bridge, Bridged};

use crate::context;
use crate::job;

// local storage
use yew::services::storage::Area;
use yew::services::{DialogService, StorageService};

use yew::format::Json;

use crate::components::connect_pod::ConnectPod;
use crate::components::search_did::SearchDID;

#[derive(Debug)]
pub enum Msg {
    Update,
    Submit,
    CreateDid,
    SendDIDCommMessage,
    UpdateId(String),
    // worker
    SendToJob,
    SendToContext,
    DataReceived,
    // local storage
    GetProfile,
    GetProfileSuccess(ProfileModel),
    SaveProfile(ProfileModel),
}

/// storage key for the profile
const KEY: &str = "oma.anna.profile";

lazy_static! {
    static ref PROVINCE_RE: Regex = Regex::new("^[A-Z]{2}$").unwrap();
}

fn must_be_true(value: &bool) -> Result<(), ValidationError> {
    if value == &true {
        Ok(())
    } else {
        Err(ValidationError::new("Must accept terms before continuing"))
    }
}

#[derive(Model, Validate, PartialEq, Clone)]
struct Address {
    #[validate(length(min = 1, message = "Street is required"))]
    street: String,
    #[validate(length(min = 1, message = "City name is required"))]
    city: String,
    #[validate(length(min = 1, message = "Enter 2 digit province code"))]
    province: String,
    postal_code: String,
    country: String,
}

#[derive(Model, Validate, PartialEq, Clone)]
struct Registration {
    #[validate(length(min = 1, message = "First name is required"))]
    first_name: String,
    #[validate(length(min = 1, message = "Last name is required"))]
    last_name: String,
    #[validate]
    address: Address,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ProfileModel {
    did: DID,
}

/// Profile.
pub struct Profile {
    link: ComponentLink<Self>,
    style: Style,
    form: Form<Registration>,
    submitted: bool,
    id: String,
    did: DID,
    // worker
    job: Box<dyn Bridge<job::Worker>>,
    context: Box<dyn Bridge<context::Worker>>,
    context_2: Box<dyn Bridge<context::Worker>>,
    storage: StorageService,
}

impl Component for Profile {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        // get profile from local storage
        let profil = ProfileModel {
            did: DID {
                method_name: "iota".into(),
                id_segments: vec!["".into()],
                ..Default::default()
            },
        };
        link.send_message(Msg::GetProfile);
        let storage = StorageService::new(Area::Local).expect("storage was disabled by the user");

        // Create model initial state
        let model = Registration {
            first_name: String::from("Alice"),
            last_name: String::from("C."),
            address: Address {
                street: String::new(),
                city: String::from("Ottawa"),
                province: String::from("ONT"),
                postal_code: String::from("K2P 0A4"),
                country: String::new(),
            },
        };

        let style = Style::create(
            String::from("profile"),
            String::from(
                r#"
                "#,
            ),
        )
        .expect("An error occured while creating the style.");

        // workers

        let callback = link.callback(|_| Msg::DataReceived);
        let job = job::Worker::bridge(callback);

        let callback = link.callback(|_| Msg::DataReceived);
        let context = context::Worker::bridge(callback);

        let callback = link.callback(|_| Msg::DataReceived);
        let context_2 = context::Worker::bridge(callback);

        Self {
            link: link,
            style,
            form: Form::new(model),
            submitted: false,
            id: "".into(),
            did: DID {
                method_name: "iota".into(),
                id_segments: vec!["".into()],
                ..Default::default()
            },

            // workers
            job,
            context,
            context_2,
            // local
            storage,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Update => true, // Force update
            Msg::Submit => {
                if self.form.validate() {
                    self.submitted = true;
                }
                true
            }
            Msg::CreateDid => {
                self.did = DID {
                    method_name: "iota".into(),
                    id_segments: vec![self.id.to_owned()],
                    ..Default::default()
                }
                .init()
                .unwrap();

                log::info!("starting up: DID {}", self.did.to_string());

                let profile = ProfileModel {
                    did: DID {
                        method_name: "iota".into(),
                        id_segments: vec![self.id.to_owned()],
                        ..Default::default()
                    },
                };

                self.link.send_message(Msg::SaveProfile(profile));

                true
            }
            Msg::SendDIDCommMessage => {
                log::info!("SendDIDCommMessage from: {}", self.did.to_string());
                let mut did_comm_message: DIDComm_message = DIDComm_message::new();
                did_comm_message.set_id("123".to_string());

                did_comm_message.set_from(self.did.to_string());

                did_comm_message.set_type(TRUSTPING); // https:://didcomm.org/v1/messages/TrustPing

                let ping = TrustPing {
                    response_requested: true,
                };
                let value = serde_json::to_value(ping).unwrap();
                let object = value.as_object().unwrap();
                did_comm_message.set_body(object.clone());

                // TODO: sign message

                log::info!("did_comm_message from: {:?}", did_comm_message);

                let did_comm_message_string = serde_json::to_string(&did_comm_message).unwrap();
                log::info!("did_comm_message_string from:{:?}", did_comm_message_string);

                true
            }
            Msg::UpdateId(value) => {
                self.id = value;
                true
            }
            Msg::SendToJob => {
                self.job.send(job::Request::GetDataFromServer);
                false
            }
            Msg::SendToContext => {
                self.context.send(context::Request::GetDataFromServer);
                self.context_2.send(context::Request::GetDataFromServer);
                false
            }
            Msg::DataReceived => {
                log::info!("DataReceived");
                false
            }
            Msg::GetProfile => {
                log::info!("GetProfile");
                let Json(profile) = self.storage.restore(KEY);
                let profile: ProfileModel = profile.ok().unwrap_or(ProfileModel {
                    did: DID {
                        method_name: "iota".into(),
                        id_segments: vec![self.id.to_owned()],
                        ..Default::default()
                    },
                });
                log::info!("GetProfile::profile {:?}", profile);
                self.link.send_message(Msg::GetProfileSuccess(profile));
                true
            }
            Msg::GetProfileSuccess(profile) => {
                log::info!("GetProfileSuccess profile: {:?}", profile);
                self.did = profile.did.to_owned();
                self.id = profile.did.id_segments.first().unwrap().into();
                true
            }
            Msg::SaveProfile(profile) => {
                log::info!("SaveProfile profile: {:?}", profile);
                self.storage.store(KEY, Json(&profile));
                false
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div class=self.style.to_string()>
                <h1>{"Profile Page"}</h1>
                <input
                    placeholder="First name"
                    value=&self.id
                    oninput=self.link.callback(|e: InputData| Msg::UpdateId(e.value))
                />

                <button
                    disabled=self.id.is_empty()
                    onclick=self.link.callback(|_| Msg::CreateDid)
                >
                    { "Update DID" }
                </button>
                <button
                    disabled=self.did.id_segments.first().unwrap().is_empty()
                    onclick=self.link.callback(|_| Msg::SendDIDCommMessage)
                >
                    { "Send Trustping" }
                </button>
                <h1>{"Search did:"}</h1>
                <SearchDID />
                <h1>{"Connect to pod:"}</h1>
                <ConnectPod />
                <h1>{"Workers:"}</h1>
                <nav class="menu">
                <button onclick=self.link.callback(|_| Msg::SendToJob)>{ "Send to Job" }</button>
                <button onclick=self.link.callback(|_| Msg::SendToContext)>{ "Send to Context" }</button>
                </nav>
                <div class="container-sm">
                <h1>{"Yew Form Example"}</h1>
                <p>{format!("Hello, {} {} and welcome to ANNA!", self.form.model().first_name, self.form.model().last_name)}</p>
                <form>
                    // TODO: support additional attributes
                    // TODO: Remove hard-coded Bootstrap classes
                    // TODO: Update form without needing oninput
                    <div class="form-group">
                        <label for="first_name">{"First Name: "}</label>
                        <Field<Registration> form=&self.form field_name="first_name" oninput=self.link.callback(|_: InputData| Msg::Update) />
                        <div class="invalid-feedback">
                            {&self.form.field_message("first_name")}
                        </div>
                    </div>
                    <div class="form-group">
                        <label for="last_name">{"Last Name: "}</label>
                        <Field<Registration> form=&self.form field_name="last_name" oninput=self.link.callback(|_: InputData| Msg::Update) />
                        <div class="invalid-feedback">
                            {&self.form.field_message("last_name")}
                        </div>
                    </div>
                    <div class="form-group">
                        <label for="address.street">{"Street: "}</label>
                        <Field<Registration> form=&self.form field_name="address.street" oninput=self.link.callback(|_: InputData| Msg::Update) />
                        <div class="invalid-feedback">
                            {&self.form.field_message("address.street")}
                        </div>
                    </div>
                    <div class="form-group">
                        <label for="address.city">{"City: "}</label>
                        <Field<Registration> form=&self.form field_name="address.city" oninput=self.link.callback(|_: InputData| Msg::Update) />
                        <div class="invalid-feedback">
                            {&self.form.field_message("address.city")}
                        </div>
                    </div>
                    <div class="form-group">
                        <label for="address.province">{"Province: "}</label>
                        <Field<Registration> form=&self.form field_name="address.province" oninput=self.link.callback(|_: InputData| Msg::Update) />
                        <div class="invalid-feedback">
                            {&self.form.field_message("address.province")}
                        </div>
                    </div>
                    <div class="form-group">
                        <label for="address.country">{"Country (optional): "}</label>
                        <Field<Registration> form=&self.form field_name="address.country" oninput=self.link.callback(|_: InputData| Msg::Update) />
                        <div class="invalid-feedback">
                            {&self.form.field_message("address.country")}
                        </div>
                    </div>
                    <div class="form-group">
                        <button type="button" onclick=self.link.callback(|e: MouseEvent| {e.prevent_default(); Msg::Submit})>{"Submit"}</button>
                    </div>
                </form>
                <div hidden=!self.submitted>
                    <h2>{"Form data"}</h2>
                    <p>{"First Name: "}{&self.form.model().first_name}</p>
                    <p>{"Last Name: "}{&self.form.model().last_name}</p>
                    <p>{"Street: "}{&self.form.model().address.street}</p>
                    <p>{"City: "}{&self.form.model().address.city}</p>
                    <p>{"Province: "}{&self.form.model().address.province}</p>
                    <p>{"Country: "}{&self.form.model().address.country}</p>
                </div>
            </div>
            </div>
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use yew_form::model::FormValue;

    #[test]
    fn test_address() {
        let mut address = Address {
            street: "street_i".to_string(),
            city: "city_i".to_string(),
            province: "prov_i".to_string(),
            postal_code: "po_i".to_string(),
            country: "country_i".to_string(),
        };

        let mut fields = vec![];
        address.fields("", &mut fields);

        assert_eq!(fields.len(), 5);
        assert!(fields.contains(&String::from("street")));

        assert_eq!(address.value("street"), String::from(address.street.clone()));

        assert!(address.set_value("street", "street_o").is_ok());

        assert_eq!(address.value("street"), String::from("street_o"));
    }

    #[test]
    fn test_composite() {
        let mut registration = Registration {
            first_name: "first_name_i".to_string(),
            last_name: "last_name_i".to_string(),
            address: Address {
                street: "street_i".to_string(),
                city: "city_i".to_string(),
                province: "prov_i".to_string(),
                postal_code: "po_i".to_string(),
                country: "country_i".to_string(),
            },
        };

        let mut fields = vec![];
        registration.fields("", &mut fields);

        assert_eq!(fields.len(), 10);
        assert!(fields.contains(&String::from("address.street")));

        assert_eq!(&registration.value("quantity"), "10");
        assert_eq!(&registration.value("price"), "5.99");

        let result = registration.set_value("quantity", "A");
        assert!(result.is_err());

        let result = registration.set_value("quantity", "12");
        assert!(result.is_ok());
        assert_eq!(&registration.value("quantity"), "12");

        assert_eq!(
            registration.value("address.street"),
            String::from(registration.address.street.clone())
        );

        registration.set_value("address.street", "street_o");

        assert_eq!(registration.address.value("street"), String::from("street_o"));
    }

    #[test]
    #[should_panic]
    fn test_invalid_field_name() {
        let address = Address {
            street: "street_i".to_string(),
            city: "city_i".to_string(),
            province: "prov_i".to_string(),
            postal_code: "po_i".to_string(),
            country: "country_i".to_string(),
        };

        address.value("not_exist");
    }
}
