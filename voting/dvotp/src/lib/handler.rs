#[allow(unused_imports)]
#[allow(dead_code)]

use serde_cbor;
use sawtooth_sdk::messages::processor::TpProcessRequest;
use sawtooth_sdk::processor::handler::{ ApplyError, TransactionContext, TransactionHandler };
use failure::Error;
use errors::LibError;

use voterid::{ VoterId, Pubkey, IdName };
use ballots::{ CBallot, RBallot };
use wrapper::Wrapper;
use voterlist::{ VoterList };
use ballotboxes::{ RBallotBox, CBallotBox };
use getset;
use routes;
//use electionstatus::ElectionStatus;
//use addressing::{ self, pubkey_to_voterlist, pubkey_to_rballotlist, pubkey_to_cballotlist };
use addressing::{ FAMILY_NAME, FAMILY_VERSION, FAMILY_PREFIX };
//use getset::FamilyState;

pub struct FamilyHandler {
    pub family_name: String,
    pub family_versions: Vec<String>,
    pub namespaces: Vec<String>
}

impl FamilyHandler {
    pub fn new() -> Self {
        FamilyHandler {
            family_name: String::from(FAMILY_NAME),
            family_versions: vec![String::from(FAMILY_VERSION)],
            namespaces: vec![String::from(FAMILY_PREFIX)],
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
        context: &mut TransactionContext
    ) -> Result<(), ApplyError> {

//        info!("Entered apply method\n");
//        info!("test output: {:?}\n", addressing::pubkey_to_voterlist(String::from("025a96a6b38a4b852182aca678153779e646a2899efb1ebfff57cef3fffc421b16")));
//        info!("inputs and outputs for this tx: {:?}\n, {:?}\n", request.get_header().get_inputs(), request.get_header().get_outputs());


        let msg_author_pubkey: String = String::from(request.get_header().get_signer_public_key());

        let mut family_state = getset::FamilyState::new(context);

        let deserd_payload: Wrapper = match serde_cbor::from_slice(request.get_payload()) {
            Ok(v) => v,
            Err(e) => return Err(ApplyError::InternalError(format!("Error deserializing payload in processor: {:?}.\n", e)))
        };

        info!("deserialized payload: {:?}\n", deserd_payload);

        match deserd_payload {
            Wrapper::Id(_voterid) => {
                match routes::voterid_route(&mut family_state, _voterid) {
                    Ok(v) => return Ok(()),
                    Err(e) => return Err(ApplyError::InternalError(format!("Error executing voterid match route in handler: {:?}\n", e)))
                }
            },
            Wrapper::Cb((_cballot, _id)) => {
                match routes::cb_route(&mut family_state, _cballot, _id) {
                    Ok(v) => return Ok(()),
                    Err(e) => return Err(ApplyError::InternalError(format!("Error executing cballot match route in handler: {:?}\n", e)))
                }
            },
            Wrapper::Rb((_rballot, _id)) => {
                match routes::rb_route(&mut family_state, _rballot, _id) {
                    Ok(v) => return Ok(()),
                    Err(e) => return Err(ApplyError::InternalError(format!("Error executing rballot match route in handler: {:?}\n", e)))
                }
            },
            Wrapper::InitC => {
                return Ok(())
            },
            Wrapper::InitR => {
                return Ok(())
            },
            Wrapper::EndC => {
                match routes::cend_route(&mut family_state) {
                    Ok(v) => return Ok(()),
                    Err(e) => return Err(ApplyError::InternalError(format!("Error executing end continuous ballot election match route in handler: {:?}\n", e)))
                }
            },
            Wrapper::EndR => {
                match routes::rend_route(&mut family_state) {
                    Ok(v) => return Ok(()),
                    Err(e) => return Err(ApplyError::InternalError(format!("Error executing end range ballot election match route in handler: {:?}\n", e)))
                }

            }

        }


    }
}