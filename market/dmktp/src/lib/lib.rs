#![allow(unused_parens)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![feature(try_trait)]
#![feature(custom_attribute)]
#![cfg_attr(test, feature(plugin))]

//#![cfg_attr(test, plugin(quickcheck_macros))]

#[cfg(test)]
extern crate rand;
#[macro_use]
extern crate clap;
#[macro_use]
extern crate log;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate failure_derive;
extern crate crypto;
extern crate failure;
extern crate sawtooth_sdk;
extern crate serde;
extern crate serde_cbor;
extern crate swth_cli_libv2;

pub mod address;
pub mod addressing;
pub mod adminmsg;
pub mod auction;
pub mod balancebook;
pub mod errors;
pub mod getset;
pub mod handler;
pub mod order;
pub mod orderbook;
pub mod routes;
pub mod sealedbid;
pub mod useracct;
pub mod validation;
pub mod wrapper;
