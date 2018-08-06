use std::collections::BTreeMap;
use errors::LibError;

use voterlist::{ VoterList };
use ballots::{ CBallot, RBallot };
use voterid::{ Pubkey };

// Maps public keys to Ballots. Public keys are strings
// for the sake of serialization.


#[derive(Serialize, Deserialize, Debug)]
pub struct RBallotBox {
    contents: BTreeMap<String, RBallot>,
    tally: Vec<u32>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CBallotBox {
    contents: BTreeMap<String, CBallot>,
    tally: u64,
    cardinality: u64,
}

impl CBallotBox {
    pub fn new() -> Self {
        CBallotBox { 
            contents: BTreeMap::new(),
            tally: 0,
            cardinality: 0,
            }
    }

    pub fn check_and_insert(&mut self, _voterlist: &VoterList, _pubkey: &Pubkey, _ballot: CBallot) -> Result<(), LibError> {
        match _voterlist.check_registration(&_pubkey) {
            true => {
                self.tally += _ballot.contents;
                self.cardinality += 1;
                self.contents.insert(_pubkey.to_string(), _ballot);
                Ok(())
            }
            false => Err(LibError::CustomError{ contents: format!("voter with public key {} is not currently registered to vote in this election\n", _pubkey.to_string())})
        }

    }

    pub fn just_insert(&mut self, _ballot: CBallot, _pubkey: &Pubkey) -> Result<(), LibError> {
        self.tally += _ballot.contents;
        self.cardinality += 1;
        self.contents.insert(_pubkey.to_string(), _ballot);
        Ok(())
    } 

    pub fn get_result(&self) -> u64 {
        self.tally / self.cardinality
    }


}

impl RBallotBox {
    pub fn new() -> Self {
        RBallotBox { 
            contents: BTreeMap::new(),
            tally: vec![0; 5]
             }
    }

    pub fn check_and_insert(&mut self, _voterlist: &VoterList, _pubkey: &Pubkey, _ballot: RBallot) -> Result<(), LibError> {
        match _voterlist.check_registration(&_pubkey) {
            true => {
                self.tally = self.tally.iter()
                                 .zip(_ballot.contents.iter())
                                 .map(|x| x.0 + x.1)
                                 .collect::<Vec<u32>>();
                self.contents.insert(_pubkey.to_string(), _ballot);
                Ok(())
            }
            false => Err(LibError::CustomError{ contents: format!("voter with public key {} is not currently registered to vote in this election\n", _pubkey.to_string())})
        }
    }

    pub fn just_insert(&mut self, _ballot: RBallot, _pubkey: &Pubkey) -> Result<(), LibError> {
        self.tally = self.tally.iter().zip(_ballot.contents.iter()).map(|x| x.0 + x.1).collect::<Vec<u32>>();
        self.contents.insert(_pubkey.to_string(), _ballot);
        Ok(())
    }

    pub fn get_result(&self) -> Vec<u32> {
        self.tally.clone()
    }

    pub fn fold_result(&self) -> u32 {
        let results = self.tally.clone();
        results.into_iter().fold(0, |acc, x| if x > acc { x } else { acc } )

    }


}

