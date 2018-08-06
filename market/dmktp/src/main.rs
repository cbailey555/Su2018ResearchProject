#![feature(try_trait)]
#![feature(custom_attribute)]
#[macro_use]
extern crate clap;
#[macro_use]
extern crate log;
#[macro_use]
extern crate failure_derive;
#[macro_use]
extern crate serde_derive;
extern crate dmktp;
extern crate failure;
extern crate log4rs;
extern crate sawtooth_sdk;
extern crate serde;
extern crate serde_cbor;

use dmktp::handler;
use dmktp::handler::FamilyHandler;
use log::LogLevelFilter;
use log4rs::append::console::ConsoleAppender;
use log4rs::config::{Appender, Config, Root};
use log4rs::encode::pattern::PatternEncoder;
use std::process;

use sawtooth_sdk::processor::TransactionProcessor;

fn main() {
    let matches = clap_app!(intkey =>
        (version: crate_version!())
        (about: "Bounded Commodies Market v 0.0.2a")
        (@arg connect: -C --connect +takes_value
         "connection endpoint for validator")
        (@arg verbose: -v --verbose +multiple
         "increase output verbosity"))
        .get_matches();

    let endpoint = matches
        .value_of("connect")
        .unwrap_or("tcp://localhost:4004");

    let console_log_level;
    match matches.occurrences_of("verbose") {
        0 => console_log_level = LogLevelFilter::Warn,
        1 => console_log_level = LogLevelFilter::Info,
        2 => console_log_level = LogLevelFilter::Debug,
        3 | _ => console_log_level = LogLevelFilter::Trace,
    }

    let stdout = ConsoleAppender::builder()
        .encoder(Box::new(PatternEncoder::new(
            "{h({l:5.5})} | {({M}:{L}):20.20} | {m}{n}",
        )))
        .build();

    let config = match Config::builder()
        .appender(Appender::builder().build("stdout", Box::new(stdout)))
        .build(Root::builder().appender("stdout").build(console_log_level))
    {
        Ok(x) => x,
        Err(_) => process::exit(1),
    };

    match log4rs::init_config(config) {
        Ok(_) => (),
        Err(_) => process::exit(1),
    }
    // ##########################################

    let handler = handler::FamilyHandler::new();
    let mut processor = TransactionProcessor::new(endpoint);

    info!("Console logging level: {}", console_log_level);

    processor.add_handler(&handler);
    processor.start();
}
