use std::collections::HashMap;

use failure::Error;
use errors::LibError;

use sawtooth_sdk::processor::handler::{ ApplyError, TransactionContext };
use serde_cbor;

use voterid::{ VoterId, Pubkey, IdName };
use ballots::{ CBallot, RBallot };
use wrapper::Wrapper;
use voterlist::{ VoterList };
use ballotboxes::{ CBallotBox, RBallotBox };
use electionstatus::ElectionStatus;
use getset::FamilyState;
use addressing::ELECTIONSTATEBOOLS;

use addressing;

// FIXME: These should all use composition with and_or, or_else style combinators
pub fn voterid_route(_familystate: &mut FamilyState, _id: VoterId) -> Result<(), ApplyError> {
    let mut fetched_voterlist: VoterList = _familystate.get_voterlist()?;
    fetched_voterlist.insert_voter(_id);
    _familystate.set_voterlist(fetched_voterlist)
} 

pub fn cb_route(_familystate: &mut FamilyState, _cb: CBallot, _id: VoterId) -> Result<(), ApplyError> {
    let mut fetched_cballotbox: CBallotBox =  _familystate.get_cballotbox()?;
//    fetched_cballotbox.just_insert(_cb, &_id.get_pubkey())?;
    match fetched_cballotbox.just_insert(_cb, &_id.get_pubkey()) {
        Ok(v) => (),
        Err(e) => return Err(ApplyError::InternalError(format!("Error inserting cballot in route handler: {:?}\n", e)))
    }

    info!("passed cbroute error match");

    _familystate.set_cballotbox(fetched_cballotbox)
}

pub fn rb_route(_familystate: &mut FamilyState, _rb: RBallot, _id: VoterId) -> Result<(), ApplyError> {
    let mut fetched_rballotbox: RBallotBox =  _familystate.get_rballotbox()?;
    match fetched_rballotbox.just_insert(_rb, &_id.get_pubkey()) {
         Ok(v) => (),
         Err(e) => return Err(ApplyError::InternalError(format!("Error inserting cballot in route handler: {:?}\n", e)))
    }

    info!("passed rb_route error match");

    _familystate.set_rballotbox(fetched_rballotbox)
}

pub fn cend_route(_familystate: &mut FamilyState) -> Result<(), ApplyError> {
    let mut fetched_election_status: ElectionStatus = _familystate.get_election_status()?;
    let mut fetched_cballotbox: CBallotBox = _familystate.get_cballotbox()?;

    fetched_election_status.set_cstatus(false);
    let cresult = fetched_cballotbox.get_result();
    _familystate.set_cresult(cresult)
}

pub fn rend_route(_familystate: &mut FamilyState) -> Result<(), ApplyError> {
    let mut fetched_election_status: ElectionStatus = _familystate.get_election_status()?;
    let mut fetched_rballotbox: RBallotBox = _familystate.get_rballotbox()?;
    info!("fetched rballotbox: {:?}\n", fetched_rballotbox);
    fetched_election_status.set_rstatus(false);
    let rresults = fetched_rballotbox.get_result();
    info!("obtained rb result just before setting: {:?}\n", rresults);
    _familystate.set_rresult(rresults)
}

