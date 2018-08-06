
#![feature(try_trait)] // Allows ParseNoneError handling via Failure
#[macro_use] extern crate clap;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate failure_derive;

extern crate serde;
extern crate serde_json;
extern crate serde_cbor;
extern crate failure;
extern crate sawtooth_sdk;
extern crate swth_cli_libv2;
extern crate futures;
extern crate hyper;
extern crate base64;
extern crate tokio_core;
extern crate rand;
extern crate dvotp;

use std::env;
use clap::{ App, Arg };
use swth_cli_libv2::clireq::{ CliRequest, ClientRequest };
use failure::Error;
use dvotp::ballots::{ CBallot, RBallot };
use dvotp::errors::LibError;
use dvotp::wrapper::Wrapper;
use swth_cli_libv2::errors::CliError;
use swth_cli_libv2::mkbatch::exec_wo_deps;
use generators::{ gen_wrapped_seeds };

//pub mod pubkeys;
pub mod generators;
pub mod getstate;
pub mod utils;

fn run() -> Result<(), CliError>  {
    let matches = App::new("Research project Voting processor")
                       .version(crate_version!())
                       .author(crate_authors!())
                       .about("leave as much functionality as possible to library")

                        .arg(Arg::with_name("verb")
                            .help("can 'set' a new value, 'inc'rement, 'dec'rement, show one value by key, or list all values"))

                        .arg(Arg::with_name("value")
                            .help("the value you want to assign, increment by, or decrement by"))

                        .arg(Arg::with_name("name")
                            .short("n")
                            .long("name")
                            .value_name("name")
                            .help("Specify real world identity for VoterID type")
                            .takes_value(true))

                        .arg(Arg::with_name("keyfile")
                            .short("k")
                            .long("keyfile")
                            .value_name("keyfile")
                            .help("Specify name of signing key. Defaults to your environment username. Enter as -k <name> for <name>.priv.")
                            .takes_value(true))

                        .arg(Arg::with_name("url")
                            .short("u")
                            .long("url")
                            .value_name("url")
                            .help("specify REST API url. Defaults to local host.")
                            .takes_value(true))
                        
                .get_matches();

    match matches.value_of("verb") {
        Some("range") => {
            let parsed = utils::parse_rb(matches.value_of("value")?);
            println!("parsed: {:?}\n", parsed);
            let id = generators::gen_default_id(matches.value_of("name"), matches.value_of("keyfile"))?;
            println!("made id: {:?}\n", id);
            let mut clireqs: Vec<CliRequest> = vec![Wrapper::Rb((RBallot::from_prefs(parsed), id)).to_cli_request().unwrap()];
            exec_wo_deps(clireqs, matches.value_of("keyfile"), matches.value_of("url"))?
        }
        Some("cont") => {
            let parsed = matches.value_of("value")?.parse::<u64>()?;
            println!("parsed: {:?}\n", parsed);
            let id = generators::gen_default_id(matches.value_of("name"), matches.value_of("keyfile"))?;
            println!("made id: {:?}\n", id);
            let mut clireqs: Vec<CliRequest> = vec![Wrapper::Cb((CBallot::from_prefs(parsed), id)).to_cli_request().unwrap()];
            exec_wo_deps(clireqs, matches.value_of("keyfile"), matches.value_of("url"))?
        }

        Some("reg_local") => {
            let mut clireqs: Vec<CliRequest> = generators::gen_wrapped_regs().into_iter().map(|x| x.to_cli_request().unwrap()).collect();
            exec_wo_deps(clireqs, matches.value_of("keyfile"), matches.value_of("url"))?
        }

        Some("c_seed") => {
            let mut clireqs: Vec<CliRequest> = generators::gen_wrapped_cbals().into_iter().map(|x| x.to_cli_request().unwrap()).collect();
            exec_wo_deps(clireqs, matches.value_of("keyfile"), matches.value_of("url"))?

        }
        Some("r_seed") => {
            let mut clireqs: Vec<CliRequest> = generators::gen_wrapped_rbals().into_iter().map(|x| x.to_cli_request().unwrap()).collect();
            exec_wo_deps(clireqs, matches.value_of("keyfile"), matches.value_of("url"))?

        }
        Some("c_end") => {
            let mut clireqs: Vec<CliRequest> = vec![Wrapper::EndC.to_cli_request().unwrap()];
            exec_wo_deps(clireqs, matches.value_of("keyfile"), matches.value_of("url"))?
        }
        Some("r_end") => {
            let mut clireqs: Vec<CliRequest> = vec![Wrapper::EndR.to_cli_request().unwrap()];
            exec_wo_deps(clireqs, matches.value_of("keyfile"), matches.value_of("url"))?
        }
        Some("v_state") => {
            getstate::get_voterlist_state(matches.value_of("url"))?
        }
        Some("c_result") => {
            getstate::get_cbresult_state(matches.value_of("url"))?
        }
        Some("r_result") => {
            getstate::get_rbresult_state(matches.value_of("url"))?
        }
        Some("c_state") => {
            getstate::get_cballotlist_state(matches.value_of("url"))?
        }
        Some("r_state") => {
            getstate::get_rballotlist_state(matches.value_of("url"))?
        }
        _ => return Err(CliError::ParsedNoneError)
    }
        Ok(())
}


fn main() {
    if let Err(e) = run() {
        println!("{}", e);
        std::process::exit(1);
    }
}
