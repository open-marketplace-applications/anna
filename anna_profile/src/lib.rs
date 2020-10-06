#![recursion_limit = "131072"]

pub mod views;
pub mod components;

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



pub mod context;
pub mod job;
