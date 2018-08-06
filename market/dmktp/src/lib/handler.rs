use sawtooth_sdk::messages::processor::TpProcessRequest;
use sawtooth_sdk::processor::handler::{ApplyError, TransactionContext, TransactionHandler};
#[allow(unused_imports)]
#[allow(dead_code)]
use std::sync::{Mutex, MutexGuard};

use serde_cbor;

use address::Address;
use addressing;
use balancebook::BalanceBook;
use order::{BuyOrder, OrderT, SellOrder};
use orderbook::OrderBook;
use useracct::UserAccount;
use wrapper::Wrapper;

use failure::Error;

use getset;
use routes;
use validation;

pub struct FamilyHandler {
    pub family_name: String,
    pub family_versions: Vec<String>,
    pub namespaces: Vec<String>,
}

impl FamilyHandler {
    pub fn new() -> Self {
        FamilyHandler {
            family_name: String::from(addressing::FAMILY_NAME),
            family_versions: vec![String::from(addressing::FAMILY_VERSION)],
            namespaces: vec![
                String::from(addressing::FAMILY_PREFIX),
                String::from(addressing::VOTING_PREFIX),
            ],
        }
    }
}

impl TransactionHandler for FamilyHandler {
    fn family_name(&self) -> String {
        self.family_name.clone()
    }

    fn family_versions(&self) -> Vec<String> {
        self.family_versions.clone()
    }

    fn namespaces(&self) -> Vec<String> {
        self.namespaces.clone()
    }

    // ONLY create and insert a DatedPrice value if an order cannot clear immediately upon
    // receipt
    fn apply(
        &self,
        request: &TpProcessRequest,
        context: &mut TransactionContext,
    ) -> Result<(), ApplyError> {
        info!("Entered apply method");

        let msg_author_pubkey: String = String::from(request.get_header().get_signer_public_key());
        let msg_author_addr: Address = Address::from_pubkey_string(&msg_author_pubkey);

        let mut family_state = getset::FamilyState::new(context);

        let deserd_payload: Wrapper = match serde_cbor::from_slice(request.get_payload()) {
            Ok(v) => v,
            Err(e) => {
                return Err(ApplyError::InternalError(format!(
                    "error deserializing payload in handler"
                )))
            }
        };

        let exec_result = match deserd_payload {
            Wrapper::Ua(_acct) => {
                match routes::acct_route(&mut family_state, _acct) {
                    Ok(v) =>  /*return*/ Ok(()),
                    Err(e) => /*return*/ Err(ApplyError::InternalError(format!("Error executing add useraccount match route in handler: {:?}\n", e))),
                }
            }
            Wrapper::Bo(_buyorder) => {
                match routes::bo_route(&mut family_state, _buyorder) {
                    Ok(v) =>  /*return*/  Ok(()),
                    Err(e) => /*return*/ Err(ApplyError::InternalError(format!("Error executing buyorder match route in handler: {:?}\n", e))),
                }
            }
            Wrapper::So(_sellorder) => {
                match routes::so_route(&mut family_state, _sellorder) {
                    Ok(v) =>  /*return*/  Ok(()),
                    Err(e) => /*return*/ Err(ApplyError::InternalError(format!("Error executing sellorder match route in handler: {:?}\n", e))),
                }
            }
            Wrapper::Na(_newauction) => {
                match routes::na_route(&mut family_state, _newauction) {
                    Ok(v) =>  /*return*/  Ok(()),
                    Err(e) => /*return*/ Err(ApplyError::InternalError(format!("Error executing new_auction match route in handler: {:?}\n", e))),
                }
            }
            Wrapper::Ab(_bid) => {
                match routes::ab_route(&mut family_state, _bid) {
                    Ok(v) =>  /*return*/  Ok(()),
                    Err(e) => /*return*/ Err(ApplyError::InternalError(format!("Error executing bid match route in handler: {:?}\n", e))),
                }
            }
            Wrapper::Ea(_serial) => {
                match routes::ea_route(&mut family_state, _serial) {
                    Ok(v) =>  /*return*/  Ok(()),
                    Err(e) => /*return*/ Err(ApplyError::InternalError(format!("Error executing end auction match route in handler: {:?}\n", e))),
                }
            }
            Wrapper::Co => {
                match routes::co_route(&mut family_state) {
                    Ok(v) =>  /*return*/  Ok(()),
                    Err(e) => /*return*/ Err(ApplyError::InternalError(format!("Error executing 'clear orderbook' route handler in handler.rs: {:?}\n", e)))
                }
            }
            Wrapper::Cb => {
                match routes::cb_route(&mut family_state) {
                    Ok(v) =>  /*return*/  Ok(()),
                    Err(e) => /*return*/ Err(ApplyError::InternalError(format!("Error executing 'clear balancebook' route handler in handler.rs: {:?}\n", e)))
                }
            }
            Wrapper::Ca => {
                match routes::ca_route(&mut family_state) {
                    Ok(v) =>  /*return*/  Ok(()),
                    Err(e) => /*return*/ Err(ApplyError::InternalError(format!("Error executing 'clear auctionlist' route handler in handler.rs: {:?}\n", e)))
                }
            }
            Wrapper::Ns(_sealedauction) => {
                match routes::ns_route(&mut family_state, _sealedauction) {
                    Ok(v) =>  /*return*/  Ok(()),
                    Err(e) => /*return*/ Err(ApplyError::InternalError(format!("Error executing 'new sealed auction' route handler in handler.rs: {:?}\n", e)))
                }
            }
            Wrapper::Sb(_sealedbid) => {
                match routes::sb_route(&mut family_state, _sealedbid) {
                    Ok(v) =>  /*return*/  Ok(()),
                    Err(e) => /*return*/ Err(ApplyError::InternalError(format!("Error executing 'sealed bid' route handler in handler.rs: {:?}\n", e)))
                }
            }
            Wrapper::Ub(_unsealedbid) => {
                match routes::ub_route(&mut family_state, _unsealedbid) {
                    Ok(v) =>  /*return*/  Ok(()),
                    Err(e) => /*return*/ Err(ApplyError::InternalError(format!("Error executing 'unsealed bid' route handler in handler.rs: {:?}\n", e)))
                }
            }
            Wrapper::Es(_serial) => {
                match routes::es_route(&mut family_state, _serial) {
                    Ok(v) =>  /*return*/  Ok(()),
                    Err(e) => /*return*/ Err(ApplyError::InternalError(format!("Error executing 'unsealed bid' route handler in handler.rs: {:?}\n", e)))
                }
            }
            Wrapper::Cs => {
                match routes::cs_route(&mut family_state) {
                    Ok(v) =>  /*return*/  Ok(()),
                    Err(e) => /*return*/ Err(ApplyError::InternalError(format!("Error executing 'clear sealed auctionlist' route handler in handler.rs: {:?}\n", e)))
                }
            }
            _ => unreachable!(),
        };

        match exec_result {
            Ok(v) => return Ok(()),
            Err(e) => {
                info!("transaction processing experienced error: {:?}\n", e);
                return Ok(());
            }
        }
    }
}
