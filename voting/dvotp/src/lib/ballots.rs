use errors::LibError;
use wrapper::Wrapper;
use swth_cli_libv2::clireq::{ CliRequest, ClientRequest, FamilyMeta };
use failure::Error;
use serde_cbor;
use voterid::VoterId;
use addressing::{ self, FAMILY_NAME, FAMILY_PREFIX, FAMILY_VERSION };

#[derive(Serialize, Deserialize, Debug, PartialEq, PartialOrd, Eq, Hash, Clone)]
// Continuous vote ballot; holds a single numerical value as preference.
// Shouldn't actually be necessary to hold more information since the
// Validators and actual blockchain messages have verifiable signatures.

pub struct CBallot {
    pub contents: u64
}

impl CBallot {
    pub fn from_prefs(x: u64) -> Self {
        CBallot { contents: x }
    }

    pub fn to_wrapper(self, _voterid: VoterId) -> Wrapper {
        Wrapper::Cb((self, _voterid))
    }

}

#[derive(Serialize, Deserialize, Debug, PartialEq, PartialOrd, Eq, Hash, Clone)]
// Range voting ballot; contains vector of scored preferences from 
// 1 - 5, 5 being the most preferred option.
pub struct RBallot {
    pub contents: Vec<u32>
}

impl RBallot {
    pub fn from_prefs(_x: Vec<u32>) -> Self {
        RBallot {
            contents: _x
        }
    }

    pub fn to_wrapper(self, _voterid: VoterId) -> Wrapper {
        Wrapper::Rb((self, _voterid))
    }
}
