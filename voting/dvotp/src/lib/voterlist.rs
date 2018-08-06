use std::collections::BTreeMap;
use errors::LibError;

use voterid::{ VoterId, IdName, Pubkey };

// This mod is the master list of registerd voters
// Implemented as a HashMap. Voters present in the HashMap
// are registered, voters not present are unregistered and ineligible
// to vote. The reason for using a heavier mapping of String -> VoterId
// rather than IE String -> bool is so that we can validate public keys.

// String is used as map key in order for Serde to derive 
// a deserialization scheme.

//Voterlist will exist only as part of processor.
// Does not require wrapper type impl.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct VoterList {
    contents: BTreeMap<String, VoterId>
}

impl VoterList {
    pub fn new() -> Self {
        VoterList { contents: BTreeMap::new() }
    }

    pub fn insert_voter(&mut self, _voterid: VoterId) -> Option<VoterId> {
        self.contents.insert(_voterid.pubkey.to_string(), _voterid)
    }

    pub fn check_registration(&self, _pubkey: &Pubkey) -> bool {
        match self.contents.get(&_pubkey.to_string()) {
            Some(v) => true,
            _ => false,
        }
    }

}


#[cfg(test)]
mod voterlisttests {
    use super::*;

    #[test]
    fn add_works() {
        let mut a_voterlist: VoterList = VoterList::new();


        let n: String = String::from("Bob Bosenbeck");
        let m: String = String::from("Casey Caseman");
        let p: String = String::from("03b1969e9f363a4a471e4a400c4a3c967b2711fb55b857de9f73bea924c3773897");
        let q: String = String::from("025a96a6b38a4b852182aca678153779e646a2899efb1ebfff57cef3fffc421b16");

        let a_voterid: VoterId = VoterId::from_vals(n, p).expect("Couldn't create voterid in voterid mod test");
        let cloned_voterid = a_voterid.clone();

        let b_voterid: VoterId = VoterId::from_vals(m, q).expect("Failed to create voterid b in voterid mod unit tests");
        let cloned_voteridb = b_voterid.clone();


        a_voterlist.insert_voter(a_voterid);

        let a_present: bool = a_voterlist.check_registration(&cloned_voterid.get_pubkey());
        let b_present: bool = a_voterlist.check_registration(&cloned_voteridb.get_pubkey());

        assert_eq!(a_present, true);
        assert_eq!(b_present, false);



    }

}



