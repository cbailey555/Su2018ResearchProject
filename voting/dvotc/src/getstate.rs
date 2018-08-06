use hyper;
use hyper::Method;
use hyper::client::{Client, Request};
use futures::{future, Future};
use futures::Stream;
use tokio_core;
use base64::decode;

use serde_json::{ self, Value as JsonValue };
use serde_cbor::{ from_slice };

use swth_cli_libv2::errors::CliError;
use dvotp::voterlist::{ VoterList };
use dvotp::ballotboxes::{ CBallotBox, RBallotBox };
use dvotp::addressing;

const CAPXTEN: u64 = 10_000_000_000;

const LOCALHOST: &'static str = "http://127.0.0.1:8008";
const STATE_QUERY_ROUTE: &'static str = "/state?address=";


pub fn decode_and_fmt_voterlist(x: &JsonValue) -> Result<(), CliError> {
    let base64_decoded = x.get(String::from("data"))?
                                           .as_str()?;
    let cbor_string = decode(base64_decoded)?;
    let as_value: VoterList = from_slice(&cbor_string[0..])?;
    println!("voterlist state is: {:?}\n", as_value);
    Ok(())
}

pub fn decode_and_fmt_cballotlist(x: &JsonValue) -> Result<(), CliError> {
    let base64_decoded = x.get(String::from("data"))?
                                           .as_str()?;
    let cbor_string = decode(base64_decoded)?;
    let as_value: CBallotBox = from_slice(&cbor_string[0..])?;
    println!("continuous range ballot box state is: {:?}\n", as_value);

    Ok(())
}


pub fn decode_and_fmt_rballotlist(x: &JsonValue) -> Result<(), CliError> {
    let base64_decoded = x.get(String::from("data"))?
                                           .as_str()?;
    let cbor_string = decode(base64_decoded)?;
    let as_value: RBallotBox = from_slice(&cbor_string[0..])?;
    println!("range ballot box state is: {:?}\n", as_value);

    Ok(())
}

pub fn decode_and_fmt_cbresult(x: &JsonValue) -> Result<(), CliError> {
    let base64_decoded = x.get(String::from("data"))?
                                           .as_str()?;
    let cbor_string = decode(base64_decoded)?;
    let as_value: u64 = from_slice(&cbor_string[0..])?;
    let imposed_cap: u64 = CAPXTEN * as_value / 100;
    println!("cballot result is: {:?}\n, indicating a consensus reduction of output by {:?}\n percent, down to a total of {} output units.", as_value, as_value, imposed_cap);

    Ok(())
}

pub fn decode_and_fmt_rbresult(x: &JsonValue) -> Result<(), CliError> {
    let base64_decoded = x.get(String::from("data"))?
                                           .as_str()?;
    let cbor_string = decode(base64_decoded)?;
    let as_value: Vec<u64> = from_slice(&cbor_string[0..])?;
    let folded: u64 = as_value.clone().into_iter().fold(0, |acc, x| if x > acc { x } else { acc } );
    let finder = as_value.clone().into_iter().position(|x| x == folded).unwrap();
    println!("rballotlist state is: {:?}. \nThe winning proposal with the highest number of points is proposition {}, with {:?} points.\n", as_value, (finder + 1), folded);

    Ok(())
}

// *************
// *************

pub fn get_voterlist_state(_url: Option<&str>) -> Result<(), CliError> {
    let route_addr = addressing::VOTERLIST;

    let req_url_string: String = match _url {
        Some(non_local) => format!("{}{}{}", non_local, STATE_QUERY_ROUTE, route_addr),
        None => format!("{}{}{}", LOCALHOST, STATE_QUERY_ROUTE, route_addr),
    };

    get_state(req_url_string, 0)
}

pub fn get_cballotlist_state(_url: Option<&str>) -> Result<(), CliError> {
//    let route_addr = addressing::pubkey_to_cballotbox(_pubkey).expect("Failed to create string from addressing::pubkey_to_cballotbox");
    let route_addr = addressing::CBALLOTBOX;

    let req_url_string: String = match _url {
        Some(non_local) => format!("{}{}{}", non_local, STATE_QUERY_ROUTE, route_addr),
        None => format!("{}{}{}", LOCALHOST, STATE_QUERY_ROUTE, route_addr),
    };

    get_state(req_url_string, 1)
}

pub fn get_rballotlist_state(_url: Option<&str>) -> Result<(), CliError> {
//    let route_addr = addressing::pubkey_to_rballotbox(_pubkey).expect("Failed to create string from addressing::pubkey_to_rballotbox");
    let route_addr = addressing::RBALLOTBOX;
;

    let req_url_string: String = match _url {
        Some(non_local) => format!("{}{}{}", non_local, STATE_QUERY_ROUTE, route_addr),
        None => format!("{}{}{}", LOCALHOST, STATE_QUERY_ROUTE, route_addr),
    };

    get_state(req_url_string, 2)
}

pub fn get_cbresult_state(_url: Option<&str>) -> Result<(), CliError> {
    let route = String::from(addressing::CBALLOTBOXRESULT);

    let req_url_string: String = match _url {
        Some(non_local) => format!("{}{}{}", non_local, STATE_QUERY_ROUTE, route),
        None => format!("{}{}{}", LOCALHOST, STATE_QUERY_ROUTE, route),
    };

    get_state(req_url_string, 3)
}

pub fn get_rbresult_state(_url: Option<&str>) -> Result<(), CliError> {
    let route = String::from(addressing::RBALLOTBOXRESULT);

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
    let response_as_serde_value: JsonValue = serde_json::from_str(&body)?;


    let mut data_vec_iter = response_as_serde_value.get(String::from("data"))?
                                                       .as_array()?
                                                       .iter();

    
    match type_code {
        0 => match data_vec_iter.len() {
            0 => return Err(CliError::CustomError { contents: String::from("No voterlist exists yet!")}),
            _ => data_vec_iter.try_for_each(|x| decode_and_fmt_voterlist(x))?
        },
        1 => match data_vec_iter.len() {
            0 => return Err(CliError::CustomError { contents: String::from("No cballotlist exists yet!")}),
            _ => data_vec_iter.try_for_each(|x| decode_and_fmt_cballotlist(x))?
        },
        2 => match data_vec_iter.len() {
            0 => return Err(CliError::CustomError { contents: String::from("No rballotlist exists yet!")}),
            _ => data_vec_iter.try_for_each(|x| decode_and_fmt_rballotlist(x))?

        },
        3 => match data_vec_iter.len() {
            0 => return Err(CliError::CustomError { contents: String::from("No CRange voting result has been posted yet!")}),
            _ => data_vec_iter.try_for_each(|x| decode_and_fmt_cbresult(x))?
        },
        4 => match data_vec_iter.len() {
            0 => return Err(CliError::CustomError { contents: String::from("No range voting result has been posted yet!")}),
            _ => data_vec_iter.try_for_each(|x| decode_and_fmt_rbresult(x))?
        },
        _ => unreachable!()
    };

    
    Ok(())
}