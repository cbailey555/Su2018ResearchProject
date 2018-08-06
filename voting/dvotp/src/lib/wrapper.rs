use serde_cbor;
use swth_cli_libv2::clireq::{ CliRequest, ClientRequest, FamilyMeta };
use errors::LibError;
use failure::Error;

use voterid::{ VoterId };
use ballots::{ CBallot, RBallot };
use addressing::{ self, ELECTIONSTATEBOOLS, FAMILY_NAME, FAMILY_PREFIX, FAMILY_VERSION };
//use adminmessage::{ InitElection, EndElection, PublishCResult, PublishRResult };


#[derive(Serialize, Deserialize, Debug, PartialEq, PartialOrd, Hash, Clone)]
pub enum Wrapper {
    Id(VoterId),
    Cb((CBallot, VoterId)),
    Rb((RBallot, VoterId)),
    InitC,
    InitR,
    EndC,
    EndR,
}

impl ClientRequest for Wrapper {
    fn to_cli_request(&self) -> Result<CliRequest, Error> {
        let family_meta = FamilyMeta::from(FAMILY_NAME, FAMILY_PREFIX, FAMILY_VERSION);
        match self {
            Wrapper::Id(_voterid) => {
            Ok(CliRequest {
                cbor_payload: serde_cbor::to_vec(&self)?,
                family_meta: FamilyMeta::from(FAMILY_NAME, FAMILY_PREFIX, FAMILY_VERSION),
                input_addrs: vec![String::from(addressing::VOTERLIST)],
                output_addrs: vec![String::from(addressing::VOTERLIST)]
            })
            },
            Wrapper::Cb((_cbal, _voterid)) => {
                Ok(CliRequest {
                    cbor_payload: serde_cbor::to_vec(&self)?,
                    family_meta: FamilyMeta::from(FAMILY_NAME, FAMILY_PREFIX, FAMILY_VERSION),
                    input_addrs: vec![String::from(addressing::VOTERLIST), String::from(addressing::CBALLOTBOX)],
                    output_addrs: vec![String::from(addressing::VOTERLIST), String::from(addressing::CBALLOTBOX)],
                })
            },
            Wrapper::Rb((_rbal, _voterid)) => {
                Ok(CliRequest {
                    cbor_payload: serde_cbor::to_vec(&self)?,
                    family_meta: FamilyMeta::from(FAMILY_NAME, FAMILY_PREFIX, FAMILY_VERSION),
                    input_addrs: vec![String::from(addressing::VOTERLIST), String::from(addressing::RBALLOTBOX)],
                    output_addrs: vec![String::from(addressing::VOTERLIST), String::from(addressing::RBALLOTBOX)],
                })
            },
            Wrapper::InitC => {
                Ok(CliRequest {
                    cbor_payload: serde_cbor::to_vec(&self)?,
                    family_meta: FamilyMeta::from(FAMILY_NAME, FAMILY_PREFIX, FAMILY_VERSION),
                    input_addrs: vec![String::from(ELECTIONSTATEBOOLS)],
                    output_addrs: vec![String::from(ELECTIONSTATEBOOLS)]
                })
            },
            Wrapper::InitR => {
                Ok(CliRequest {
                    cbor_payload: serde_cbor::to_vec(&self)?,
                    family_meta: FamilyMeta::from(FAMILY_NAME, FAMILY_PREFIX, FAMILY_VERSION),
                    input_addrs: vec![String::from(ELECTIONSTATEBOOLS)],
                    output_addrs: vec![String::from(ELECTIONSTATEBOOLS)]
                })
            },
            Wrapper::EndC => {
                Ok(CliRequest {
                    cbor_payload: serde_cbor::to_vec(&self)?,
                    family_meta: FamilyMeta::from(FAMILY_NAME, FAMILY_PREFIX, FAMILY_VERSION),
                    input_addrs: vec![String::from(addressing::CBALLOTBOX), String::from(addressing::CBALLOTBOXRESULT), String::from(addressing::ELECTIONSTATEBOOLS)],
                    output_addrs: vec![String::from(addressing::CBALLOTBOX), String::from(addressing::CBALLOTBOXRESULT), String::from(addressing::ELECTIONSTATEBOOLS)],
                })
            },
            Wrapper::EndR => {
                Ok(CliRequest {
                    cbor_payload: serde_cbor::to_vec(&self)?,
                    family_meta: FamilyMeta::from(FAMILY_NAME, FAMILY_PREFIX, FAMILY_VERSION),
                    input_addrs: vec![String::from(addressing::RBALLOTBOX), String::from(addressing::RBALLOTBOXRESULT), String::from(addressing::ELECTIONSTATEBOOLS)],
                    output_addrs: vec![String::from(addressing::RBALLOTBOX), String::from(addressing::RBALLOTBOXRESULT), String::from(addressing::ELECTIONSTATEBOOLS)],
                })
            },
        }
    }
}

