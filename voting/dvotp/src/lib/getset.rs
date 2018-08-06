#[allow(unused_imports)]
#[allow(dead_code)]

use std::collections::HashMap;
use sawtooth_sdk::processor::handler::{ TransactionContext, ContextError, ApplyError };
use failure::Error;

use serde_cbor;

use voterlist::{ VoterList };
use ballotboxes::{ CBallotBox, RBallotBox };
use electionstatus::ElectionStatus;
use voterid::VoterId;

use errors::LibError;
use addressing;

const FAMILY_NAME: &'static str = "VOTING";
const FAMILY_VERSION: &'static str = "1.0";
const FAMILY_PREFIX: &'static str = "594666";


pub struct FamilyState<'a> {
    context: &'a mut TransactionContext,
}

impl <'a> FamilyState<'a> {

    pub fn new(context: &'a mut TransactionContext) -> FamilyState {
        FamilyState {
            context: context
        }
    }

    pub fn get_voterlist(&mut self) -> Result<VoterList, ApplyError> {
        let ser_voterlist_response = self.context.get_state(addressing::VOTERLIST)?;
        match ser_voterlist_response {
            Some(contents) => {
                let deser_voterlist: VoterList = serde_cbor::from_slice(&contents[0..])?;
                Ok(deser_voterlist)
            },
            None => {
                Ok(VoterList::new())
            }
        }
    }


    pub fn get_election_status(&mut self) -> Result<ElectionStatus, ApplyError> {
        let ser_election_status_response = self.context.get_state(addressing::ELECTIONSTATEBOOLS)?;
        match ser_election_status_response {
            Some(contents) => {
                let deser_election_status: ElectionStatus = serde_cbor::from_slice(&contents[0..])?;
                Ok(deser_election_status)
            },
            None => {
                Ok(ElectionStatus::new())
            }
        }
    }

    pub fn set_election_status(&mut self, _electionstatus: ElectionStatus) -> Result<(), ApplyError> {
        let serd_status = serde_cbor::to_vec(&_electionstatus)?;
        match self.context.set_state(addressing::ELECTIONSTATEBOOLS, &serd_status) {
            Ok(v) => return Ok(()),
            Err(e) => return Err(ApplyError::InternalError(format!("Error executing set_voteridlist in getset: {:?}\n", e)))
        }
    }

    pub fn set_voterlist(&mut self, _voterlist: VoterList) -> Result<(), ApplyError> {
        let serd_voterlist = serde_cbor::to_vec(&_voterlist)?;
        match self.context.set_state(addressing::VOTERLIST, &serd_voterlist) {
            Ok(v) => return Ok(()),
            Err(e) => return Err(ApplyError::InternalError(format!("Error executing set_voteridlist in getset: {:?}\n", e)))
        }
    }

    pub fn get_cballotbox(&mut self) -> Result<CBallotBox, ApplyError> {
        let ser_cblist_response = self.context.get_state(addressing::CBALLOTBOX)?;
        match ser_cblist_response {
            Some(contents) => {
                let deser_cbbox: CBallotBox = serde_cbor::from_slice(&contents[0..])?;
                Ok(deser_cbbox)
            },
            None => Ok(CBallotBox::new())
        }
    }

    pub fn set_cballotbox(&mut self, _cblist: CBallotBox) -> Result<(), ApplyError> {
        let serd_cballotbox = serde_cbor::to_vec(&_cblist)?;
        match self.context.set_state(addressing::CBALLOTBOX, &serd_cballotbox) {
            Ok(v) => return Ok(()),
            Err(e) => return Err(ApplyError::InternalError(format!("Error executing set_cballotbox in getset: {:?}\n", e)))
        }
    }

    pub fn get_rballotbox(&mut self) -> Result<RBallotBox, ApplyError> {
        let ser_rblist_response = self.context.get_state(addressing::RBALLOTBOX)?;
        match ser_rblist_response {
            Some(contents) => {
                let deserd_rbbox: RBallotBox = serde_cbor::from_slice(&contents[0..])?;
                Ok(deserd_rbbox)
            },
            None => Ok(RBallotBox::new())
            }
    }

    pub fn set_rballotbox(&mut self, _rblist: RBallotBox) -> Result<(), ApplyError> {
        let serd_rbbox = serde_cbor::to_vec(&_rblist)?;
        match self.context.set_state(addressing::RBALLOTBOX, &serd_rbbox) {
            Ok(v) => return Ok(()),
            Err(e) => return Err(ApplyError::InternalError(format!("Error executing set_rballotbox in getset: {:?}\n", e)))
        }

    }

    pub fn get_cballotresult(&mut self) -> Result<Option<u64>, ApplyError> {
        let ser_cblist_response = self.context.get_state(addressing::CBALLOTBOX)?;
        match ser_cblist_response {
            Some(contents) => {
                let cballotresult: CBallotBox = serde_cbor::from_slice(&contents[0..])?;
                Ok(Some(cballotresult.get_result()))
            },
            None => Ok(None)
        }
    }


    pub fn get_rballotresult(&mut self, rblist_addr: &String) -> Result<Option<Vec<u32>>, ApplyError> {
        let ser_rblist_response = self.context.get_state(&rblist_addr)?;
        match ser_rblist_response {
            Some(contents) => {
                let rballotresult: RBallotBox = serde_cbor::from_slice(&contents[0..])?;
                Ok(Some(rballotresult.get_result()))
            },
//            None => return Err(ApplyError::InvalidTransaction(format!("Attempted to get election results, but there was no ballotlist")))
            None => Ok(None)
        }
    }




// ALl changes in state for orderbook AND balancebook will be done within the apply method using
// associated methods, so all the getters and setters have to do is get/deserialize, and
// set/serialize.


    pub fn set_cresult(&mut self, _result: u64) -> Result<(), ApplyError> {
        let result_serd = serde_cbor::to_vec(&_result)?;
        match self.context.set_state(addressing::CBALLOTBOXRESULT, &result_serd) {
            Ok(v) => return Ok(()),
            Err(e) => return Err(ApplyError::InternalError(format!("Error executing set_cresult in getset: {:?}\n", e)))
        }

    }

    pub fn set_rresult(&mut self, _result: Vec<u32>) -> Result<(), ApplyError> {
        let result_serd = serde_cbor::to_vec(&_result)?;
        info!("rresult serd: {:?}\n", result_serd);
        match self.context.set_state(addressing::RBALLOTBOXRESULT, &result_serd) {
            Ok(v) => return Ok(()),
            Err(e) => return Err(ApplyError::InternalError(format!("Error executing set_rresult in getset: {:?}\n", e)))
        }

    }

}
