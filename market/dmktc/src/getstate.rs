use hyper;
use hyper::Method;
use hyper::client::{Client, Request};
use futures::{future, Future};
use futures::Stream;
use tokio_core;
use base64::decode;

use serde_cbor::{ from_slice };
use serde_json::{ self, Value as JsonValue };



use dmktp::orderbook::OrderBook;
use dmktp::balancebook::BalanceBook;
use dmktp::auction::{ AuctionList, Auction };
use dmktp::addressing::{ BALANCEBOOK, ORDERBOOK, AUCTIONLIST, SEALEDAUCTIONLIST, CBALLOTBOXRESULT };
use dmktp::sealedbid::{ SealedAuctionList, SealedAuction, SealedBid, UnsealedBid };
use swth_cli_libv2::errors::CliError;

const CAPXTEN: u64 = 10_000_000_000;

const LOCALHOST: &'static str = "http://127.0.0.1:8008";
const STATE_QUERY_ROUTE: &'static str = "/state?address=";


pub fn decode_and_fmt_orderbook(x: &JsonValue) -> Result<(), CliError> {
    let base64_decoded = x.get(String::from("data"))?
                                           .as_str()?;
    let cbor_string = decode(base64_decoded)?;
    let as_value: OrderBook = from_slice(&cbor_string[0..])?;
    println!("Order book state: {}\n", as_value);

    Ok(())
}

pub fn decode_and_fmt_cbresult(x: &JsonValue) -> Result<(), CliError> {
    let base64_decoded = x.get(String::from("data"))?
                                           .as_str()?;
    let cbor_string = decode(base64_decoded)?;
    let as_value: u64 = from_slice(&cbor_string[0..])?;
    let imposed_cap: u64 = CAPXTEN * as_value / 100;
    println!("cballot result is: {:?}\n, indicating consensus on reduction of output by {:?}\n percent, down to a total of {} output units.", as_value, as_value, imposed_cap);

    Ok(())
}

pub fn decode_and_fmt_balancebook(x: &JsonValue) -> Result<(), CliError> {
    let base64_decoded = x.get(String::from("data"))?
                                           .as_str()?;

    let cbor_string = decode(base64_decoded)?;
    let as_value: BalanceBook = from_slice(&cbor_string[0..])?;
    println!("balancebook state: {}", as_value);
    Ok(())
}

pub fn decode_and_fmt_auctionlist(x: &JsonValue) -> Result<(), CliError> {
    let base64_decoded = x.get(String::from("data"))?
                                           .as_str()?;

    let cbor_string = decode(base64_decoded)?;
    let as_value: AuctionList = from_slice(&cbor_string[0..])?;
    println!("all auctions state: {:?}", as_value);
    Ok(())
}

pub fn decode_and_fmt_sealedauctionlist(x: &JsonValue) -> Result<(), CliError> {
    let base64_decoded = x.get(String::from("data"))?
                                           .as_str()?;

    let cbor_string = decode(base64_decoded)?;
    let as_value: SealedAuctionList = from_slice(&cbor_string[0..])?;
    println!("all sealed bid auctions state: {}", as_value);
    Ok(())
}



pub fn init_get_state_orderbook(_url: Option<&str>) -> Result<(), CliError> {
    let req_url_string: String = match _url {
        Some(non_local) => format!("{}{}{}", non_local, STATE_QUERY_ROUTE, ORDERBOOK),
        None => format!("{}{}{}", LOCALHOST, STATE_QUERY_ROUTE, ORDERBOOK),
    };

    get_state(req_url_string, 0)
}

pub fn init_get_state_balancebook(_url: Option<&str>) -> Result<(), CliError> {
    let req_url_string: String = match _url {
        Some(non_local) => format!("{}{}{}", non_local, STATE_QUERY_ROUTE, BALANCEBOOK),
        None => format!("{}{}{}", LOCALHOST, STATE_QUERY_ROUTE, BALANCEBOOK),
    };

    get_state(req_url_string, 1)
}

pub fn init_get_state_auctionlist(_url: Option<&str>) -> Result<(), CliError> {
    let req_url_string: String = match _url {
        Some(non_local) => format!("{}{}{}", non_local, STATE_QUERY_ROUTE, AUCTIONLIST),
        None => format!("{}{}{}", LOCALHOST, STATE_QUERY_ROUTE, AUCTIONLIST),
    };

    get_state(req_url_string, 2)
}

pub fn init_get_state_sealedauctionlist(_url: Option<&str>) -> Result<(), CliError> {
    let req_url_string: String = match _url {
        Some(non_local) => format!("{}{}{}", non_local, STATE_QUERY_ROUTE, SEALEDAUCTIONLIST),
        None => format!("{}{}{}", LOCALHOST, STATE_QUERY_ROUTE, SEALEDAUCTIONLIST),
    };

    get_state(req_url_string, 3)
}

pub fn init_get_cbresult_state(_url: Option<&str>) -> Result<(), CliError> {
    let route = String::from(CBALLOTBOXRESULT);

    let req_url_string: String = match _url {
        Some(non_local) => format!("{}{}{}", non_local, STATE_QUERY_ROUTE, route),
        None => format!("{}{}{}", LOCALHOST, STATE_QUERY_ROUTE, route),
    };

    get_state(req_url_string, 4)
}


pub fn get_state(_url: String, type_code: usize) -> Result<(), CliError> {


    let hyper_uri = match _url.parse::<hyper::Uri>() {
        Ok(uri) => uri,
        Err(e) => return Err(CliError::SubmissionError{ error_details: (format!("Invalid get URL: {}: {}", e, _url)) }),
    };
    //

    match hyper_uri.scheme() {
        Some(scheme) => {
            if scheme != "http" {
                return Err(CliError::SubmissionError{
                    error_details: (format!(
                    "Unsupported scheme ({}) in URL: {}",
                    scheme, _url
                ))
                });
            }
        }
        None => {
            return Err(CliError::SubmissionError {
                error_details: (format!("No scheme in URL: {}", _url))
            });
        }
    }

    let mut core = tokio_core::reactor::Core::new()?;
    let handle = core.handle();
    let client = Client::configure().build(&handle);

    let req = Request::new(Method::Get, hyper_uri);
    let work = client.request(req).and_then(|res| {
        res.body()
            .fold(Vec::new(), |mut v, chunk| {
                v.extend(&chunk[..]);
                future::ok::<_, hyper::Error>(v)
            })
            .and_then(move |chunks| {
                let body = String::from_utf8(chunks).unwrap();
                future::ok(body)
            })
    });

    let body = core.run(work)?;
    let response_as_serde_value: JsonValue = match serde_json::from_str(&body) {
        Ok(v) => v,
        Err(e) => return Err(CliError::CustomError { contents: format!("reponse_as_serde_value in getstate hyper portion parsed nothing")})
    };


    let mut data_vec_iter = response_as_serde_value.get(String::from("data"))?
                                                       .as_array()?
                                                       .iter();
    
    match type_code {
        0 => match data_vec_iter.len() {
            0 => return Err(CliError::CustomError { contents: String::from("No orderbook exists yet!")}),
            _ => data_vec_iter.try_for_each(|x| decode_and_fmt_orderbook(x))?
        },
        1 => match data_vec_iter.len() {
            0 => return Err(CliError::CustomError { contents: String::from("No balancebook exists yet!")}),
            _ => data_vec_iter.try_for_each(|x| decode_and_fmt_balancebook(x))?
        },
        2 => match data_vec_iter.len() {
            0 => return Err(CliError::CustomError { contents: String::from("No auctions exist yet!")}),
            _ => data_vec_iter.try_for_each(|x| decode_and_fmt_auctionlist(x))?

        }
        3 => match data_vec_iter.len() {
            0 => return Err(CliError::CustomError { contents: String::from("No sealed bid auctions exist yet!")}),
            _ => data_vec_iter.try_for_each(|x| decode_and_fmt_sealedauctionlist(x))?
        },
        4 => match data_vec_iter.len() {
            0 => return Err(CliError::CustomError { contents: String::from("No CRange voting result has been posted yet!")}),
            _ => data_vec_iter.try_for_each(|x| decode_and_fmt_cbresult(x))?
        }
        _ => unreachable!()
    };

    
    Ok(())
}