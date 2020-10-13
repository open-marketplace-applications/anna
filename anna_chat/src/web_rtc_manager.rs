use js_sys::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::*;

use crate::chat::*;

use std::cell::RefCell;
use std::rc::Rc;
use std::str;

use base64;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use yew::services::{ConsoleService, IntervalService, Task, TimeoutService};
#[allow(unused_imports)]
use yew::{html, App, Callback, Component, ComponentLink, Html, InputData, ShouldRender};

type SingleArgClosure = Closure<dyn FnMut(JsValue)>;
type SingleArgJsFn = Box<dyn FnMut(JsValue)>;

#[allow(dead_code)]
type NoArgClosure = Closure<dyn FnMut()>;
#[allow(dead_code)]
type NoArgJsFn = Box<dyn FnMut()>;
#[allow(dead_code)]
type PromiseConstructorType = Box<dyn FnMut(Function, Function)>;

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ConnectionState {
    pub ice_gathering_state: Option<RtcIceGatheringState>,
    pub ice_connection_state: Option<RtcIceConnectionState>,
    pub data_channel_state: Option<RtcDataChannelState>,
}

impl ConnectionState {
    pub fn new() -> ConnectionState {
        ConnectionState {
            ice_gathering_state: None,
            ice_connection_state: None,
            data_channel_state: None,
        }
    }
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub enum OfferError {
    InvalidBase64,
    InvalidString,
    SerializationError,
    InvalidOffer,
    //InvalidCandidate,
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub enum State {
    DefaultState,
    Server(ConnectionState),
    Client(ConnectionState),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct IceCandidate {
    candidate: String,
    sdp_mid: String,
    sdp_m_line_index: u16,
}

pub struct WebRTCManager {
    state: State,
    rtc_peer_connection: Option<RtcPeerConnection>,
    data_channel: Option<RtcDataChannel>,
    create_offer_closure: Option<SingleArgClosure>,
    create_answer_closure: Option<SingleArgClosure>,

    channel_status_change_closure: Option<SingleArgClosure>,
    on_ice_candidate_closure: Option<SingleArgClosure>,
    on_ice_connection_state_change_closure: Option<SingleArgClosure>,
    on_ice_gathering_state_change_closure: Option<SingleArgClosure>,
    on_data_channel_closure: Option<SingleArgClosure>,

    on_data_closure: Option<SingleArgClosure>,
    set_candidate_closure: Option<SingleArgClosure>,
    promise_exception_handlers: Vec<SingleArgClosure>,
    exit_offer_or_answer_early: bool,
    ice_candidates: Vec<IceCandidate>,
    offer: String,
    parent_link: ComponentLink<ChatModel>,
}

impl WebRTCManager {
    pub fn create_default(link: ComponentLink<ChatModel>) -> WebRTCManager {
        let web_rtc_manager = WebRTCManager {
            state: State::DefaultState,
            rtc_peer_connection: None,
            data_channel: None,
            create_offer_closure: None,
            channel_status_change_closure: None,
            on_ice_candidate_closure: None,
            on_ice_connection_state_change_closure: None,
            on_ice_gathering_state_change_closure: None,
            on_data_channel_closure: None,
            create_answer_closure: None,
            on_data_closure: None,
            set_candidate_closure: None,
            promise_exception_handlers: vec![],
            ice_candidates: Vec::new(),
            offer: "".into(),
            parent_link: link,
            exit_offer_or_answer_early: false,
        };

        web_rtc_manager
    }

    pub fn send_message(&self, message_content: String) {
        self.data_channel
            .as_ref()
            .expect("must have a data channel")
            .send_with_str(&message_content)
            .expect("channel is open");

        //TODO error handling ?
    }

    pub fn get_state(&self) -> State {
        self.state.clone()
    }

    pub fn set_state(&mut self, new_state: State) {
        self.state = new_state;
    }

    pub fn get_offer(&self) -> String {
        self.offer.clone()
    }

    pub fn get_ice_candidates(&self) -> Vec<IceCandidate> {
        self.ice_candidates.clone()
    }

    // TODO : handle error when adding ice_candidate
    pub fn set_candidates(
        web_rtc_manager: Rc<RefCell<WebRTCManager>>,
        connection_string: &ConnectionString,
    ) {
        if web_rtc_manager.borrow().exit_offer_or_answer_early {
            return;
        }

        for candidate in &connection_string.ice_candidates {
            let mut ice_candidate_init = RtcIceCandidateInit::new("");

            ice_candidate_init.candidate(&candidate.candidate);
            ice_candidate_init.sdp_mid(Some(&candidate.sdp_mid));
            ice_candidate_init.sdp_m_line_index(Some(candidate.sdp_m_line_index));

            let ice_candidate = RtcIceCandidate::new(&ice_candidate_init).expect("valid candidate");

            let add_candidate_exception_handler = WebRTCManager::get_exception_handler(
                web_rtc_manager.clone(),
                "add_candidate closure has encountered an exception".into(),
            );

            let _promise = web_rtc_manager
                .borrow()
                .rtc_peer_connection
                .as_ref()
                .unwrap()
                .add_ice_candidate_with_opt_rtc_ice_candidate(Some(&ice_candidate))
                .catch(&add_candidate_exception_handler);

            web_rtc_manager
                .borrow_mut()
                .promise_exception_handlers
                .push(add_candidate_exception_handler);
        }
    }

    fn parse_base64_str_to_connection(str: &String) -> Result<ConnectionString, OfferError> {
        let decoded = base64::decode(str);

        let decoded = decoded.map_err(|_| OfferError::InvalidBase64);

        let decoded_str = decoded.and_then(|a| {
            let to_str = str::from_utf8(&a);
            match to_str {
                Ok(a) => Ok(a.to_string()),
                Err(_) => Err(OfferError::InvalidString),
            }
        });

        let connection_string: Result<ConnectionString, OfferError> =
            decoded_str.and_then(|a_str| {
                serde_json::from_str::<ConnectionString>(&a_str)
                    .map_err(|_| OfferError::SerializationError)
            });

        let connection_string = match connection_string {
            Ok(a) => {
                let remote_description = JSON::parse(&a.offer);
                if remote_description.is_err() {
                    // TODO : additional check
                    return Err(OfferError::InvalidOffer);
                }
                    
                // TODO : additional check
                Ok(a)
            }
            Err(e) => Err(e),
        };

        connection_string
    }

    pub fn validate_answer(
        web_rtc_manager: Rc<RefCell<WebRTCManager>>,
        str: &String,
    ) -> Result<(), OfferError> {
        let connection_string = WebRTCManager::parse_base64_str_to_connection(str);

        if connection_string.is_err() {
            return Err(connection_string.err().unwrap());
        }

        let connection_string = connection_string.ok().unwrap();

        let remote_description_js_value: JsValue =
            JSON::parse(&connection_string.offer).expect("Expected valid json");

        let remote_description =
            remote_description_js_value.unchecked_into::<RtcSessionDescriptionInit>();
    
        let web_rtc_manager_rc_clone = web_rtc_manager.clone();

        let set_remote_description_exception_handler = Closure::wrap(Box::new(move |_a: JsValue| {
            web_rtc_manager_rc_clone
                .borrow_mut()
                .exit_offer_or_answer_early = true;

            // console::log_1(&"Exception handler !".into());
            // console::log_1(&a);

            web_sys::Window::alert_with_message(
                &web_sys::window().unwrap(),
                &format!("Promise set_remote_description encountered an exception. See console for details"),
            )
            .expect("alert should work");

            web_rtc_manager_rc_clone
                .borrow()
                .parent_link
                .send_message(Msg::ResetWebRTC);
        }) as SingleArgJsFn);

        let connection_string = Rc::new(connection_string.clone());
        let web_rtc_manager_rc_clone = web_rtc_manager.clone();
        let set_candidates_function: SingleArgJsFn = Box::new(move |_: JsValue| {
            WebRTCManager::set_candidates(web_rtc_manager_rc_clone.clone(), &*connection_string);
        });
        let set_candidates_closure = Closure::wrap(set_candidates_function);

        let _promise = web_rtc_manager
            .borrow()
            .rtc_peer_connection
            .as_ref()
            .unwrap()
            .set_remote_description(&remote_description)
            .catch(&set_remote_description_exception_handler)
            .then(&set_candidates_closure);

        web_rtc_manager
            .borrow_mut()
            .promise_exception_handlers
            .push(set_remote_description_exception_handler);

        web_rtc_manager.borrow_mut().set_candidate_closure = Some(set_candidates_closure);

        Ok(())
    }

    pub fn validate_offer(
        web_rtc_manager: Rc<RefCell<WebRTCManager>>,
        str: &String,
    ) -> Result<(), OfferError> {
        let connection_string = WebRTCManager::parse_base64_str_to_connection(str);

        if connection_string.is_err() {
            return Err(connection_string.err().unwrap());
        }
        let connection_string = connection_string.ok().unwrap();

        let remote_description_js_value: JsValue =
            JSON::parse(&connection_string.offer).expect("Expected valid json");

        let remote_description =
            remote_description_js_value.unchecked_into::<RtcSessionDescriptionInit>();

        let web_rtc_manager_rc_clone = web_rtc_manager.clone();

        let set_local_description_function: SingleArgJsFn = Box::new(move |answer: JsValue| {
            let answer = answer.unchecked_into::<RtcSessionDescriptionInit>();

            let set_local_description_exception_handler = WebRTCManager::get_exception_handler(
                web_rtc_manager_rc_clone.clone(),
                "set_local_description closure has encountered an exception".into(),
            );

            // console::log_1(&"setting local description".into());

            let _promise = web_rtc_manager_rc_clone
                .borrow()
                .rtc_peer_connection
                .as_ref()
                .unwrap()
                .set_local_description(&answer)
                .catch(&set_local_description_exception_handler);

            web_rtc_manager_rc_clone
                .borrow_mut()
                .promise_exception_handlers
                .push(set_local_description_exception_handler);

            console::log_1(&answer.clone().into());

            web_rtc_manager_rc_clone.borrow_mut().offer =
                String::from(JSON::stringify(&answer).unwrap());
        });

        let set_local_description_closure = Closure::wrap(set_local_description_function);
        let web_rtc_manager_rc_clone = web_rtc_manager.clone();

        let create_answer_function: Box<dyn FnMut(JsValue)> = Box::new(move |a: JsValue| {
            let connection_string = Rc::new(connection_string.clone());
            let clone = web_rtc_manager_rc_clone.clone();

            let set_candidates_function: SingleArgJsFn = Box::new(move |_: JsValue| {
                WebRTCManager::set_candidates(clone.clone(), &*connection_string);
            });

            let set_candidates_closure = Closure::wrap(set_candidates_function);
            let web_rtc_manager_rc_clone_for_error_handler = web_rtc_manager_rc_clone.clone();

            let create_answer_exception_handler = Closure::wrap(Box::new(
                move |_send_channel: JsValue| {
                    web_rtc_manager_rc_clone_for_error_handler
                        .borrow_mut()
                        .exit_offer_or_answer_early = true;

                    console::log_1(&"Exception handler !".into());
                    console::log_1(&a);

                    web_sys::Window::alert_with_message(
                    &web_sys::window().unwrap(),
                    &format!("Promise create_answer encountered an exception. See console for details"),
                )
                .expect("alert should work");
                },
            ) as SingleArgJsFn);

            let _promise = web_rtc_manager_rc_clone
                .borrow()
                .rtc_peer_connection
                .as_ref()
                .unwrap()
                .create_answer()
                .then(&set_local_description_closure)
                .catch(&create_answer_exception_handler)
                .then(&set_candidates_closure);

            web_rtc_manager_rc_clone
                .borrow_mut()
                .promise_exception_handlers
                .push(create_answer_exception_handler);

            web_rtc_manager_rc_clone.borrow_mut().set_candidate_closure =
                Some(set_candidates_closure);
        
        });

        let create_answer_closure = Closure::wrap(create_answer_function);

        let web_rtc_manager_rc_clone = web_rtc_manager.clone();
        let set_remote_description_exception_handler =
            Closure::wrap(Box::new(move |_send_channel: JsValue| {
                web_rtc_manager_rc_clone
                    .borrow_mut()
                    .exit_offer_or_answer_early = true;
            }) as SingleArgJsFn);

        let _promise = web_rtc_manager
            .borrow()
            .rtc_peer_connection
            .as_ref()
            .unwrap()
            .set_remote_description(&remote_description)
            .catch(&set_remote_description_exception_handler)
            .then(&create_answer_closure);

        web_rtc_manager
            .borrow_mut()
            .promise_exception_handlers
            .push(set_remote_description_exception_handler);

        web_rtc_manager.borrow_mut().create_answer_closure = Some(create_answer_closure);

        Ok(())
    }

    fn get_channel_status_change_closure(
        web_rtc_manager: Rc<RefCell<WebRTCManager>>,
    ) -> SingleArgClosure {
        let channel_status_change_closure = Closure::wrap(Box::new(move |_send_channel: JsValue| {
            let state = web_rtc_manager
                .borrow()
                .data_channel
                .as_ref()
                .unwrap()
                .ready_state();

            let self_state = web_rtc_manager.borrow().get_state();

            let new_state = match self_state {
                State::Server(mut connection_state) => {
                    connection_state.data_channel_state = Some(state);
                    State::Server(connection_state)
                }
                State::Client(mut connection_state) => {
                    connection_state.data_channel_state = Some(state);
                    State::Client(connection_state)
                }
                a => a,
            };

            web_rtc_manager.borrow_mut().set_state(new_state);

            let web_rtc_state = web_rtc_manager.borrow().get_state();

            web_rtc_manager
                .borrow()
                .parent_link
                .send_message(Msg::UpdateWebRTCState(web_rtc_state));
        }) as SingleArgJsFn);

        channel_status_change_closure
    }

    fn get_on_data_closure(web_rtc_manager: Rc<RefCell<WebRTCManager>>) -> SingleArgClosure {
        let on_data = Closure::wrap(Box::new(move |arg: JsValue| {
            let message_event = arg.unchecked_into::<web_sys::MessageEvent>();

            let msg_content: String = message_event.data().as_string().unwrap();
            let msg = Message::new(msg_content, MessageSender::Other);

            web_rtc_manager
                .borrow()
                .parent_link
                .send_message(Msg::NewMessage(msg));
        }) as SingleArgJsFn);

        on_data
    }

    fn get_on_ice_candidate_closure(
        web_rtc_manager: Rc<RefCell<WebRTCManager>>,
    ) -> SingleArgClosure {
        let on_ice_candidate_closure =
            Closure::wrap(Box::new(move |ice_connection_event: JsValue| {
                // console::log_1(&ice_connection_event.clone().into());

                let ice_connection_event_obj: RtcPeerConnectionIceEvent =
                    ice_connection_event.unchecked_into::<RtcPeerConnectionIceEvent>();

                if let Some(candidate) = ice_connection_event_obj.candidate() {
                    let candidate_str = candidate.candidate();

                    if !candidate_str.is_empty() {
                        // console::log_1(&candidate_str.clone().into());

                        let saved_candidate = IceCandidate {
                            candidate: candidate_str,
                            sdp_mid: candidate.sdp_mid().unwrap(),
                            sdp_m_line_index: candidate.sdp_m_line_index().unwrap(),
                        };

                        web_rtc_manager
                            .borrow_mut()
                            .ice_candidates
                            .push(saved_candidate);
                    }
                }
            }) as SingleArgJsFn);

        on_ice_candidate_closure
    }

    fn get_on_ice_connection_state_change_closure(
        web_rtc_manager: Rc<RefCell<WebRTCManager>>,
    ) -> SingleArgClosure {
        let on_ice_connection_state_change =
            Closure::wrap(Box::new(move |_ice_connection_state_event: JsValue| {
                let ice_new_state: RtcIceConnectionState = {
                    let inner = web_rtc_manager.borrow();
                    let connection: &RtcPeerConnection =
                        inner.rtc_peer_connection.as_ref().unwrap();
                    connection.ice_connection_state()
                };

                let self_state = web_rtc_manager.borrow().get_state();

                let new_state = match self_state {
                    State::Server(mut connection_state) => {
                        connection_state.ice_connection_state = Some(ice_new_state);
                        State::Server(connection_state)
                    }
                    State::Client(mut connection_state) => {
                        connection_state.ice_connection_state = Some(ice_new_state);
                        State::Client(connection_state)
                    }
                    a => a,
                };

                web_rtc_manager.borrow_mut().set_state(new_state);

                let web_rtc_state = web_rtc_manager.borrow().get_state();

                web_rtc_manager
                    .borrow()
                    .parent_link
                    .send_message(Msg::UpdateWebRTCState(web_rtc_state));
            }) as SingleArgJsFn);

        on_ice_connection_state_change
    }

    fn get_on_ice_gathering_state_change_closure(
        web_rtc_manager: Rc<RefCell<WebRTCManager>>,
    ) -> SingleArgClosure {
        let on_ice_gathering_state_change =
            Closure::wrap(Box::new(move |_ice_gathering_state: JsValue| {
                let ice_new_state: RtcIceGatheringState = {
                    let inner = web_rtc_manager.borrow();
                    let connection: &RtcPeerConnection =
                        inner.rtc_peer_connection.as_ref().unwrap();
                    connection.ice_gathering_state()
                };

                let self_state = web_rtc_manager.borrow().get_state();

                let new_state = match self_state {
                    State::Server(mut connection_state) => {
                        connection_state.ice_gathering_state = Some(ice_new_state);
                        State::Server(connection_state)
                    }
                    State::Client(mut connection_state) => {
                        connection_state.ice_gathering_state = Some(ice_new_state);
                        State::Client(connection_state)
                    }
                    a => a,
                };

                web_rtc_manager.borrow_mut().set_state(new_state);
                let web_rtc_state = web_rtc_manager.borrow().get_state();

                web_rtc_manager
                    .borrow()
                    .parent_link
                    .send_message(Msg::UpdateWebRTCState(web_rtc_state));
            }) as SingleArgJsFn);

        on_ice_gathering_state_change
    }

    fn get_exception_handler(
        _web_rtc_manager: Rc<RefCell<WebRTCManager>>,
        _message: String,
    ) -> SingleArgClosure {
        let exception_handler = Closure::wrap(Box::new(move |_a: JsValue| {
            // TODO
            // console::log_1(&"Exception handler !".into());
            // console::log_1(&JsValue::from_str(&message));
            // console::log_1(&a);

            web_sys::Window::alert_with_message(
                &web_sys::window().unwrap(),
                &format!("Promise encountered an exception. See console for details"),
            )
            .expect("alert should work");
        }) as SingleArgJsFn);

        exception_handler
    }

    fn set_data_channel(web_rtc_manager: Rc<RefCell<WebRTCManager>>, data_channel: RtcDataChannel) {
        let channel_status_change_closure =
            WebRTCManager::get_channel_status_change_closure(web_rtc_manager.clone());

        data_channel.set_onopen(Some(channel_status_change_closure.as_ref().unchecked_ref()));
        data_channel.set_onclose(Some(channel_status_change_closure.as_ref().unchecked_ref()));

        web_rtc_manager.borrow_mut().channel_status_change_closure =
            Some(channel_status_change_closure);

        let on_data_closure = WebRTCManager::get_on_data_closure(web_rtc_manager.clone());
        data_channel.set_onmessage(Some(on_data_closure.as_ref().unchecked_ref()));

        web_rtc_manager.borrow_mut().on_data_closure = Some(on_data_closure);
        web_rtc_manager.borrow_mut().data_channel = Some(data_channel);
    }

    pub fn start_web_rtc(web_rtc_manager: Rc<RefCell<WebRTCManager>>) {
        let ice_servers = Array::new();
        {
            let server_entry = Object::new();
            let _ = Reflect::set(
                &server_entry,
                &"urls".into(),
                &"stun:stun.l.google.com:19302".into(),
            );

            ice_servers.push(&*server_entry);
        }

        let mut rtc_configuration = RtcConfiguration::new();
        rtc_configuration.ice_servers(&ice_servers);

        let rtc_peer_connection = RtcPeerConnection::new_with_configuration(&rtc_configuration)
            .expect("RtcPeerConnection constructor failure");

        let create_offer_exception_handler = WebRTCManager::get_exception_handler(
            web_rtc_manager.clone(),
            "create_offer closure has encountered an exception".into(),
        );

        let state = web_rtc_manager.borrow().state.clone();

        match state {
            State::Server(_connection_state) => {
                let web_rtc_manager_rc_clone = web_rtc_manager.clone();

                let mut data_channel_init = RtcDataChannelInit::new();
                data_channel_init.ordered(true);

                let data_channel: RtcDataChannel = rtc_peer_connection
                    .create_data_channel_with_data_channel_dict("sendChannel", &data_channel_init);

                WebRTCManager::set_data_channel(web_rtc_manager.clone(), data_channel);

                let create_offer_function: SingleArgJsFn = Box::new(move |offer: JsValue| {
                    let rtc_session_description: RtcSessionDescriptionInit =
                    offer.unchecked_into::<RtcSessionDescriptionInit>();

                    web_rtc_manager_rc_clone.borrow_mut().offer =
                        String::from(JSON::stringify(&rtc_session_description).unwrap());

                    let set_local_description_exception_handler =
                        WebRTCManager::get_exception_handler(
                            web_rtc_manager_rc_clone.clone(),
                            "set_local_description closure has encountered an exception".into(),
                        );

                    let _promise = web_rtc_manager_rc_clone
                        .borrow_mut()
                        .rtc_peer_connection
                        .as_ref()
                        .unwrap()
                        .set_local_description(&rtc_session_description)
                        .catch(&set_local_description_exception_handler);

                    web_rtc_manager_rc_clone
                        .borrow_mut()
                        .promise_exception_handlers
                        .push(set_local_description_exception_handler);
                });

                let create_offer_closure = Closure::wrap(create_offer_function);

                let _create_offer_promise = rtc_peer_connection
                    .create_offer()
                    .then(&create_offer_closure)
                    .catch(&create_offer_exception_handler);

                web_rtc_manager.borrow_mut().create_offer_closure = Some(create_offer_closure);
                
            }

            State::Client(_connection_state) => {
                let clone = web_rtc_manager.clone();

                let on_data_channel_closure =
                    Closure::wrap(Box::new(move |data_channel_event: JsValue| {
                        let data_channel_event =
                            data_channel_event.unchecked_into::<RtcDataChannelEvent>();
                        let data_channel = data_channel_event.channel();
                        WebRTCManager::set_data_channel(clone.clone(), data_channel);
                    }) as SingleArgJsFn);

                rtc_peer_connection
                    .set_ondatachannel(Some(on_data_channel_closure.as_ref().unchecked_ref()));

                web_rtc_manager.borrow_mut().on_data_channel_closure =
                    Some(on_data_channel_closure);
            }

            _ => {
                panic!("Not implemented");
            }
        };

        let on_ice_candidate_closure =
            WebRTCManager::get_on_ice_candidate_closure(web_rtc_manager.clone());

        let on_ice_connection_state_change_closure =
            WebRTCManager::get_on_ice_connection_state_change_closure(web_rtc_manager.clone());

        let on_ice_gathering_state_change_closure =
            WebRTCManager::get_on_ice_gathering_state_change_closure(web_rtc_manager.clone());

        rtc_peer_connection
            .set_onicecandidate(Some(on_ice_candidate_closure.as_ref().unchecked_ref()));

        rtc_peer_connection.set_oniceconnectionstatechange(Some(
            on_ice_connection_state_change_closure
                .as_ref()
                .unchecked_ref(),
        ));

        rtc_peer_connection.set_onicegatheringstatechange(Some(
            on_ice_gathering_state_change_closure
                .as_ref()
                .unchecked_ref(),
        ));

        web_rtc_manager.borrow_mut().rtc_peer_connection = Some(rtc_peer_connection);

        web_rtc_manager
            .borrow_mut()
            .promise_exception_handlers
            .push(create_offer_exception_handler);

        web_rtc_manager.borrow_mut().on_ice_candidate_closure = Some(on_ice_candidate_closure);

        web_rtc_manager
            .borrow_mut()
            .on_ice_connection_state_change_closure = Some(on_ice_connection_state_change_closure);

        web_rtc_manager
            .borrow_mut()
            .on_ice_gathering_state_change_closure = Some(on_ice_gathering_state_change_closure);
    }
}