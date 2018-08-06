use failure::Error;
use std::fmt;

use address::Address;
use errors::LibError::{self, CustomError, EmptyZeroError, EncodingError};
use wrapper::Wrapper;
//use creditdebit::{ Credit, Debit, BalanceChange };
//use balancebook::{ BalanceBook };
//use serwrapper::SW;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Hash, Eq)]
pub struct UserAccount {
    pub addr: Address,
    pub name: String,
    pub cash: u64,
    pub assets: u64,
    pub hold_cash: u64,
    pub hold_assets: u64,
}

impl fmt::Display for UserAccount {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: {}    cash (liquid): {}\n    assets (liquid): {}\n    cash (held): {}\n    assets(held): {}", 
        self.name, self.addr, self.cash, self.assets, self.hold_cash, self.hold_assets)
    }
}

impl UserAccount {
    pub fn from_vals(_name: String, _addr: String) -> Result<Self, LibError> {
        let address: Address = Address::from_string(_addr)?;

        if _name.len() == 0 {
            //
            return Err(EmptyZeroError {
                field: String::from("User Account Name"),
            });
        //
        } else if _name.len() > 40 {
            //
            return Err(CustomError {
                contents: String::from(
                    "User Account Name (real ID name) must contain fewer than 40 characters",
                ),
            });
        //
        } else if _name.is_ascii() == false {
            //
            return Err(EncodingError {
                field: String::from("User Account Name (real ID name)"),
                encoding: String::from("ASCII"),
                got: _name,
            });
        //
        } else {
            //
            Ok(UserAccount {
                addr: address,
                name: _name,
                cash: 0,
                assets: 0,
                hold_cash: 0,
                hold_assets: 0,
            })
            //
        }
    }

    pub fn new_seeded(
        _name: String,
        _addr: String,
        _seed_cash: u64,
        _seed_assets: u64,
    ) -> Result<Self, LibError> {
        let address: Address = Address::from_string(_addr)?;

        if _name.len() == 0 {
            //
            return Err(EmptyZeroError {
                field: String::from("User Account Name"),
            });
        //
        } else if _name.len() > 40 {
            //
            return Err(CustomError {
                contents: String::from(
                    "User Account Name (real ID name) must contain fewer than 40 characters",
                ),
            });
        //
        } else if _name.is_ascii() == false {
            //
            return Err(EncodingError {
                field: String::from("User Account Name (real ID name)"),
                encoding: String::from("ASCII"),
                got: _name,
            });
        //
        } else {
            //
            Ok(UserAccount {
                addr: address,
                name: _name,
                cash: _seed_cash,
                assets: _seed_assets,
                hold_cash: 0,
                hold_assets: 0,
            })
            //
        }
    }

    pub fn as_wrapper(self) -> Wrapper {
        Wrapper::Ua(self)
    }

    pub fn addr_ref(&self) -> &Address {
        &self.addr
    }

    pub fn addr_clone(&self) -> Address {
        self.addr.clone()
    }

    pub fn addr_str(&self) -> &String {
        self.addr.to_string_ref()
    }

    pub fn addr_string_clone(&self) -> String {
        self.addr.contents.clone()
    }
}
