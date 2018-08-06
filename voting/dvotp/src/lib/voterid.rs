use errors::LibError;
use wrapper::Wrapper;
use swth_cli_libv2::clireq::{ CliRequest, ClientRequest, FamilyMeta };
use failure::Error;
use serde_cbor;
use addressing;


#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, PartialOrd, Hash, Eq)]
pub struct VoterId {
    pub name: IdName,
    pub pubkey: Pubkey,
}

impl VoterId {
    pub fn from_vals(_name: String, _pubkey: String) -> Result<Self, LibError> {
        let name: IdName = IdName::from_string(&_name)?;
        let pubkey: Pubkey = Pubkey::from_string(&_pubkey)?;
        Ok(VoterId {
            name: name,
            pubkey: pubkey,
        })
    }

    pub fn get_pubkey(&self) -> Pubkey {
        self.pubkey.clone()
    }

    pub fn get_pubkey_as_string(&self) -> String {
        self.pubkey.clone().to_string()
    }

    pub fn to_wrapper(self) -> Wrapper {
        Wrapper::Id(self)
    }

// This is only ergonomic if the structures they'll end up in get passed to all incoming txs
//    pub fn processor_exec(self) -> Result<(), Error> {
//        
//    }
}


#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, PartialOrd, Hash, Eq)]
pub struct IdName {
   pub contents: String
}


#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, PartialOrd, Hash, Eq)]
pub struct Pubkey {
    pub contents: String
}

impl Pubkey {
    pub fn from_string(_a: &String) -> Result<Self, LibError> {
        match (_a.is_ascii() && _a.len() == 66) {
            true => Ok(Pubkey { contents: _a.clone() }), 
            false => Err(LibError::CustomError{ contents: format!("valid voter public keys must be 66 ascii characters. Received: {}\n", _a)})
        }
    }

    pub fn to_string(&self) -> String {
        self.contents.clone()
    }

}

impl IdName {
    pub fn from_string(_a: &String) -> Result<Self, LibError> {
        match (_a.is_ascii() && _a.len() > 0 && _a.len() < 100) {
            true => Ok(IdName { contents: _a.clone() }),
            false => Err(LibError::CustomError{ contents: format!("valid voter names must be ascii encoded, non-empty, and less than 100 characters in length. Received: {}\n", _a)}) 
        }
    }

    pub fn to_string(&self) -> String {
        self.contents.clone()
    }
}
