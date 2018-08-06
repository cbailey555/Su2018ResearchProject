#![allow(unused_parens)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![feature(try_trait)]
#![feature(custom_attribute)]
#![cfg_attr(test, feature(plugin))]

#[macro_use] extern crate serde_derive;
#[macro_use] extern crate failure_derive;
#[macro_use] extern crate log;
extern crate sawtooth_sdk;
extern crate swth_cli_libv2;
extern crate serde;
extern crate serde_cbor;
extern crate failure;
extern crate log4rs;
//extern crate rand;

pub mod errors;
pub mod getset;
pub mod addressing;
pub mod handler;
pub mod wrapper;
pub mod voterid;
pub mod voterlist;
pub mod ballots;
pub mod ballotboxes;
pub mod electionstatus;
pub mod routes;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
