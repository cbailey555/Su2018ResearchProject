#![ignore(unused_mut)]
#![ignore(unused_import)]
#![feature(try_trait)] // Allows ParseNoneError handling via Failure
#[macro_use] extern crate clap;

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
extern crate dmktp;

use clap::{ App, Arg };
use swth_cli_libv2::clireq::{ CliRequest, ClientRequest };
use failure::Error;
use dmktp::order::{ BuyOrder, SellOrder, OrderT };
use dmktp::wrapper::Wrapper;
use dmktp::auction::{ Auction, Bid };
use swth_cli_libv2::errors::CliError;
use swth_cli_libv2::mkbatch::exec_wo_deps;

//pub mod pubkeys;
pub mod generators;
pub mod getstate;
pub mod utils;

fn run() -> Result<(), Error>  {
    let matches = App::new("Intkey CLI using swth_cli_lib")
                       .version(crate_version!())
                       .author(crate_authors!())
                       .about("leave as much functionality as possible to library")

                        .arg(Arg::with_name("verb")
                            .help("Either a utility option, or \'buy\'/\'sell\'"))

                        .arg(Arg::with_name("price")
                            .help("The price you want your buy/sell order to execute at"))

                        .arg(Arg::with_name("quantity")
                            .help("The quantity of your buy/sell order"))
                        
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
        Some("get_orderbook") => {
            getstate::init_get_state_orderbook(matches.value_of("url"))?;
        },
        Some("get_balancebook") => {
            getstate::init_get_state_balancebook(matches.value_of("url"))?;
        },
        Some("get_auctionlist") => {
            getstate::init_get_state_auctionlist(matches.value_of("url"))?;
        },
        Some("get_sealed_auctions") => {
            getstate::init_get_state_sealedauctionlist(matches.value_of("url"))?;
        },
        Some("get_cap") => {
            getstate::init_get_cbresult_state(matches.value_of("url"))?;
        }
        Some("seed_accts") => {
            let mut clireqs: Vec<CliRequest> = generators::all_seeded_wrapped().iter_mut().map(|x| x.to_cli_request().unwrap()).collect();
            exec_wo_deps(clireqs, matches.value_of("keyfile"), matches.value_of("url"))?;
        },
        Some("buy") => {
            let price: u64 = match matches.value_of("price") {
                Some(v) => v.parse::<u64>()?,
                None => return Err(Error::from(CliError::CustomError { contents: format!("Price cannot be left blank!")}))
            };
            let qty: u64 = match matches.value_of("quantity") {
                Some(v) => v.parse::<u64>()?,
                None => return Err(Error::from(CliError::CustomError { contents: format!("Quantity cannot be left blank!")}))
            };
            let addr = utils::addr_from_keyfile(matches.value_of("keyfile"))?;

            let clireqs = vec![Wrapper::Bo(BuyOrder::from_vals(addr, price, qty, 0)).to_cli_request().unwrap()];
            exec_wo_deps(clireqs, matches.value_of("keyfile"), matches.value_of("url"))?;

        }
        Some("sell") => {
            let price: u64 = match matches.value_of("price") {
                Some(v) => v.parse::<u64>()?,
                None => return Err(Error::from(CliError::CustomError { contents: format!("Price cannot be left blank!")}))
            };
            let qty: u64 = match matches.value_of("quantity") {
                Some(v) => v.parse::<u64>()?,
                None => return Err(Error::from(CliError::CustomError { contents: format!("Quantity cannot be left blank!")}))
            };
            let addr = utils::addr_from_keyfile(matches.value_of("keyfile"))?;

            let clireqs = vec![Wrapper::So(SellOrder::from_vals(addr, price, qty, 0)).to_cli_request().unwrap()];
            exec_wo_deps(clireqs, matches.value_of("keyfile"), matches.value_of("url"))?;
        }
        Some("mk_auction") => {
            let mut clireqs: Vec<CliRequest> = generators::gen_auction().into_iter().map(|x| x.to_cli_request().unwrap()).collect();
            exec_wo_deps(clireqs, matches.value_of("keyfile"), matches.value_of("url"))?;
        }
        Some("bid") => {
            let serial: u64 = match matches.value_of("price") {
                Some(v) => v.parse::<u64>()?,
                None => return Err(Error::from(CliError::CustomError { contents: format!("Auction serial no. cannot be left blank!")}))
            };
            let price: u64 = match matches.value_of("quantity") {
                Some(v) => v.parse::<u64>()?,
                None => return Err(Error::from(CliError::CustomError { contents: format!("Bid amount cannot be left blank!")}))
            };

            let bid: Wrapper = utils::mk_bid(matches.value_of("keyfile"), serial, price)?;
            let mut clireqs: Vec<CliRequest> = vec![bid].into_iter().map(|x| x.to_cli_request().unwrap()).collect();

            exec_wo_deps(clireqs, matches.value_of("keyfile"), matches.value_of("url"))?;
        }
        Some("end_auction") => {
            let serial: u64 = match matches.value_of("price") {
                Some(v) => v.parse::<u64>()?,
                None => return Err(Error::from(CliError::CustomError { contents: format!("Auction serial no. cannot be left blank!")}))
            };

            let mut clireqs: Vec<CliRequest> = vec![Wrapper::Ea(serial)].into_iter().map(|x| x.to_cli_request().unwrap()).collect();
            exec_wo_deps(clireqs, matches.value_of("keyfile"), matches.value_of("url"))?;
        }
        Some("sell1") => {
            let mut clireqs: Vec<CliRequest> = generators::gensell1().into_iter().map(|x| x.to_cli_request().unwrap()).collect();
            exec_wo_deps(clireqs, matches.value_of("keyfile"), matches.value_of("url"))?;

        }
        Some("sell2") => {
            let mut clireqs: Vec<CliRequest> = generators::gensell2().into_iter().map(|x| x.to_cli_request().unwrap()).collect();
            exec_wo_deps(clireqs, matches.value_of("keyfile"), matches.value_of("url"))?;
        }
        Some("sell3") => {
            let mut clireqs: Vec<CliRequest> = generators::gensell3().into_iter().map(|x| x.to_cli_request().unwrap()).collect();
            exec_wo_deps(clireqs, matches.value_of("keyfile"), matches.value_of("url"))?;
        }
        Some("buy1") => {
            let mut clireqs: Vec<CliRequest> = generators::genbuy1().into_iter().map(|x| x.to_cli_request().unwrap()).collect();
            exec_wo_deps(clireqs, matches.value_of("keyfile"), matches.value_of("url"))?;
        }
        Some("buy2") => {
            let mut clireqs: Vec<CliRequest> = generators::genbuy2().into_iter().map(|x| x.to_cli_request().unwrap()).collect();
            exec_wo_deps(clireqs, matches.value_of("keyfile"), matches.value_of("url"))?;
        }
        Some("buy3") => {
            let mut clireqs: Vec<CliRequest> = generators::genbuy3().into_iter().map(|x| x.to_cli_request().unwrap()).collect();
            exec_wo_deps(clireqs, matches.value_of("keyfile"), matches.value_of("url"))?;
        }
        Some("oddsell1") => {
            let mut clireqs: Vec<CliRequest> = generators::genoddsell1().into_iter().map(|x| x.to_cli_request().unwrap()).collect();
            exec_wo_deps(clireqs, matches.value_of("keyfile"), matches.value_of("url"))?;
        }
        Some("oddbuy1") => {
            let mut clireqs: Vec<CliRequest> = generators::genoddbuy1().into_iter().map(|x| x.to_cli_request().unwrap()).collect();
            exec_wo_deps(clireqs, matches.value_of("keyfile"), matches.value_of("url"))?;
        }
        Some("mk_sealed_auction1") => {
            let mut clireqs: Vec<CliRequest> = vec![generators::gen_sealed_auction1()].into_iter().map(|x| x.to_cli_request().unwrap()).collect();
            exec_wo_deps(clireqs, matches.value_of("keyfile"), matches.value_of("url"))?;
        }
        Some("mk_sealed_auction2") => {
            let mut clireqs: Vec<CliRequest> = vec![generators::gen_sealed_auction2()].into_iter().map(|x| x.to_cli_request().unwrap()).collect();
            exec_wo_deps(clireqs, matches.value_of("keyfile"), matches.value_of("url"))?;
        }
        Some("mk_sealed_auction") => {
            let auction_amt = match matches.value_of("price") {
                Some(v) => v.parse::<u64>()?,
                None => 100_000_000
            };

            let mut clireqs: Vec<CliRequest> = vec![generators::gen_sealed_auction_ng(auction_amt)].into_iter().map(|x| x.to_cli_request().unwrap()).collect();
            exec_wo_deps(clireqs, matches.value_of("keyfile"), matches.value_of("url"))?;
        }

        Some("show_sealed_auction1") => {
            println!("{:?}\n", generators::gen_sealed_auction1());
        }
        Some("show_sealed_auction2") => {
            println!("{:?}\n", generators::gen_sealed_auction2());
        }

        Some("sealed_bid1_1") => {
            let wrapper: Wrapper = generators::gen_sealed_bid1(matches.value_of("keyfile"))?;
            println!("Submitting sealed bid: {:?}\n", wrapper);
            let mut clireqs: Vec<CliRequest> = vec![wrapper].into_iter().map(|x| x.to_cli_request().unwrap()).collect();

            exec_wo_deps(clireqs, matches.value_of("keyfile"), matches.value_of("url"))?;
        }
        Some("show_sealed_bid1_1") => {
            println!("{:?}\n", generators::gen_sealed_bid1(matches.value_of("keyfile")));
        }
        Some("sealed_bid1_2") => {
            let wrapper: Wrapper = generators::gen_sealed_bid2(matches.value_of("keyfile"))?;
            println!("Submitting sealed bid: {:?}\n", wrapper);
            let mut clireqs: Vec<CliRequest> = vec![wrapper].into_iter().map(|x| x.to_cli_request().unwrap()).collect();

            exec_wo_deps(clireqs, matches.value_of("keyfile"), matches.value_of("url"))?;
        }
        Some("show_sealed_bid1_2") => {
            println!("{:?}\n", generators::gen_sealed_bid2(matches.value_of("keyfile")));
        }
        Some("sealed_bid1_3") => {
            let wrapper: Wrapper = generators::gen_sealed_bid3(matches.value_of("keyfile"))?;
            println!("Submitting sealed bid: {:?}\n", wrapper);
            let mut clireqs: Vec<CliRequest> = vec![wrapper].into_iter().map(|x| x.to_cli_request().unwrap()).collect();

            exec_wo_deps(clireqs, matches.value_of("keyfile"), matches.value_of("url"))?;
        }
        Some("reveal_bid1_1") => {
            let wrapper: Wrapper = generators::gen_unsealed_bid1(matches.value_of("keyfile"))?;
            println!("Submitting revealed bid: {:?}\n", wrapper);
            let mut clireqs: Vec<CliRequest> = vec![wrapper].into_iter().map(|x| x.to_cli_request().unwrap()).collect();

            exec_wo_deps(clireqs, matches.value_of("keyfile"), matches.value_of("url"))?;

        }
        Some("reveal_bid1_2") => {
            let wrapper: Wrapper = generators::gen_unsealed_bid2(matches.value_of("keyfile"))?;
            println!("Submitting revealed bid: {:?}\n", wrapper);
            let mut clireqs: Vec<CliRequest> = vec![wrapper].into_iter().map(|x| x.to_cli_request().unwrap()).collect();

            exec_wo_deps(clireqs, matches.value_of("keyfile"), matches.value_of("url"))?;

        }
        Some("reveal_bid1_3") => {
            let wrapper: Wrapper = generators::gen_unsealed_bid3(matches.value_of("keyfile"))?;
            println!("Submitting revealed bid: {:?}\n", wrapper);
            let mut clireqs: Vec<CliRequest> = vec![wrapper].into_iter().map(|x| x.to_cli_request().unwrap()).collect();

            exec_wo_deps(clireqs, matches.value_of("keyfile"), matches.value_of("url"))?;

        }
        Some("clear_balancebook") => {
            let wrapper: Wrapper = Wrapper::Cb;
            let mut clireqs: Vec<CliRequest> = vec![wrapper].into_iter().map(|x| x.to_cli_request().unwrap()).collect();
            exec_wo_deps(clireqs, matches.value_of("keyfile"), matches.value_of("url"))?;
        }
        Some("clear_orderbook") => {
            let wrapper: Wrapper = Wrapper::Co;
            let mut clireqs: Vec<CliRequest> = vec![wrapper].into_iter().map(|x| x.to_cli_request().unwrap()).collect();
            exec_wo_deps(clireqs, matches.value_of("keyfile"), matches.value_of("url"))?;
        }
        Some("end_sealed_auction") => {
            let serial: u64 = match matches.value_of("price") {
                Some(v) => v.parse::<u64>()?,
                None => return Err(Error::from(CliError::CustomError { contents: format!("Auction serial no. cannot be left blank!")}))
            };

            let mut clireqs: Vec<CliRequest> = vec![Wrapper::Es(serial)].into_iter().map(|x| x.to_cli_request().unwrap()).collect();
            exec_wo_deps(clireqs, matches.value_of("keyfile"), matches.value_of("url"))?;
        }
        Some("clear_sealed_auction_list") => {
            let mut clireqs: Vec<CliRequest> = vec![Wrapper::Cs].into_iter().map(|x| x.to_cli_request().unwrap()).collect();
            exec_wo_deps(clireqs, matches.value_of("keyfile"), matches.value_of("url"))?;
        }


        _ => return Err(Error::from(CliError::ParsedNoneError))

    }
        println!("Finished");


        Ok(())
}


fn main() {
    if let Err(e) = run() {
        println!("{}", e);
        std::process::exit(1);
    }
}
