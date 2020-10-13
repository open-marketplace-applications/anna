#![recursion_limit = "131072"]

pub mod components;
pub mod views;

#[macro_use]
extern crate log;
extern crate web_logger;

#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate validator_derive;
extern crate yew_form;
#[macro_use]
extern crate yew_form_derive;

#[macro_use]
extern crate serde_json;

pub mod context;
pub mod job;
