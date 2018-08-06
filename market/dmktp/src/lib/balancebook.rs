use address::Address;
use errors::LibError::{
    self, IntOverflowError, IntUnderflowError, NExistKeyError, UserExistsError,
};
use failure::Error;
use std::collections::BTreeMap;
use std::fmt;
use useracct::UserAccount;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct BalanceBook {
    pub balance_book: BTreeMap<String, UserAccount>,
}

impl fmt::Display for BalanceBook {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.balance_book
            .iter()
            .try_for_each(|x| write!(f, "\n{}\n", x.1))
    }
}

impl BalanceBook {
    pub fn new() -> Self {
        BalanceBook {
            balance_book: BTreeMap::new(),
        }
    }

    pub fn new_from(_b: BTreeMap<String, UserAccount>) -> Self {
        BalanceBook { balance_book: _b }
    }

    pub fn hotswap(&mut self, _incoming: BalanceBook) {
        self.balance_book = _incoming.balance_book;
    }

    pub fn insert_new_user(&mut self, _useraccount: UserAccount) -> Result<(), Error> {
        match self.balance_book.contains_key(_useraccount.addr_str()) {
            true => {
                return Err(Error::from(UserExistsError {
                    user: _useraccount.addr.to_string(),
                }))
            }
            false => {
                self.balance_book
                    .insert(_useraccount.addr.to_string(), _useraccount);
                Ok(())
            }
        }
    }

    pub fn insert_seeded(&mut self, _useraccount: UserAccount) -> Result<(), Error> {
        match self.balance_book.contains_key(_useraccount.addr_str()) {
            true => {
                return Err(Error::from(UserExistsError {
                    user: _useraccount.addr.to_string(),
                }))
            }
            false => {
                self.balance_book
                    .insert(_useraccount.addr.to_string(), _useraccount);
                Ok(())
            }
        }
    }

    pub fn remove(&mut self, _useraccount: UserAccount) {
        self.balance_book.remove(_useraccount.addr_str());
    }

    // Fine to taken & return a reference since this is only for viewing.
    pub fn get_useraccount(&self, _useraccount: UserAccount) -> Option<&UserAccount> {
        self.balance_book.get(_useraccount.addr_str())
    }

    pub fn get_by_addr(&self, _addr: &Address) -> Option<&UserAccount> {
        self.balance_book.get(_addr.to_string_ref())
    }

    // Will mostly be called with an &Address; from orderbook (orders contain Addresses)
    pub fn get_acct_mut(&mut self, _addr: &Address) -> Result<&mut UserAccount, LibError> {
        match self.balance_book.get_mut(_addr.to_string_ref()) {
            Some(v) => Ok(v),
            None => {
                return Err(NExistKeyError {
                    contents: format!("{}", _addr.to_string()),
                    structure: String::from("User balance book"),
                })
            }
        }
    }

    //
    //
    //
    //
    //
    //

    pub fn credit_cash(&mut self, _addr: &Address, _amt: u64) -> Result<(), Error> {
        match self.balance_book.get_mut(&_addr.contents) {
            Some(val) => {
                if let Some(checked_sum) = val.cash.checked_add(_amt) {
                    val.cash = checked_sum;
                    return Ok(());
                } else {
                    return Err(Error::from(IntOverflowError {
                        origin: String::from("credit cash balance method"),
                        fst: val.cash as usize,
                        snd: _amt as usize,
                        intsize: String::from("u64"),
                    }));
                }
            }
            None => {
                return Err(Error::from(NExistKeyError {
                    contents: format!("{}", _addr.to_string()),
                    structure: String::from("User balance book >> get_mut for cash credit"),
                }))
            }
        }
    }

    pub fn credit_assets(&mut self, _addr: &Address, _amt: u64) -> Result<(), Error> {
        match self.balance_book.get_mut(_addr.to_string_ref()) {
            Some(val) => {
                if let Some(checked_sum) = val.assets.checked_add(_amt) {
                    val.assets = checked_sum;
                    return Ok(());
                } else {
                    return Err(Error::from(IntOverflowError {
                        origin: String::from("credit asset balance method"),
                        fst: val.assets as usize,
                        snd: _amt as usize,
                        intsize: String::from("u64"),
                    }));
                }
            }
            None => {
                return Err(Error::from(NExistKeyError {
                    contents: format!("{}", _addr.to_string()),
                    structure: String::from("User balance book >> get mut for asset credit"),
                }))
            }
        }
    }
    pub fn credit_hold_cash(&mut self, _addr: &Address, _amt: u64) -> Result<(), Error> {
        match self.balance_book.get_mut(_addr.to_string_ref()) {
            Some(val) => {
                if let Some(checked_sum) = val.hold_cash.checked_add(_amt) {
                    val.hold_cash = checked_sum;
                    return Ok(());
                } else {
                    return Err(Error::from(IntOverflowError {
                        origin: String::from("credit hold_cash balance method"),
                        fst: val.hold_cash as usize,
                        snd: _amt as usize,
                        intsize: String::from("u64"),
                    }));
                }
            }
            None => {
                return Err(Error::from(NExistKeyError {
                    contents: format!("{}", _addr.to_string()),
                    structure: String::from("User balance book"),
                }))
            }
        }
    }
    pub fn credit_hold_assets(&mut self, _addr: &Address, _amt: u64) -> Result<(), Error> {
        match self.balance_book.get_mut(_addr.to_string_ref()) {
            Some(val) => {
                if let Some(checked_sum) = val.hold_assets.checked_add(_amt) {
                    val.hold_assets = checked_sum;
                    Ok(())
                } else {
                    return Err(Error::from(IntOverflowError {
                        origin: String::from("credit asset balance method"),
                        fst: val.hold_assets as usize,
                        snd: _amt as usize,
                        intsize: String::from("u64"),
                    }));
                }
            }
            None => {
                return Err(Error::from(NExistKeyError {
                    contents: format!("{}", _addr.to_string()),
                    structure: String::from("User balance book"),
                }))
            }
        }
    }

    pub fn debit_cash(&mut self, _addr: &Address, _amt: u64) -> Result<(), Error> {
        match self.balance_book.get_mut(_addr.to_string_ref()) {
            Some(val) => {
                if let Some(checked_diff) = val.cash.checked_sub(_amt) {
                    val.cash = checked_diff;
                    Ok(())
                } else {
                    return Err(Error::from(IntUnderflowError {
                        origin: String::from("debit cash balance method"),
                        fst: val.cash as usize,
                        snd: _amt as usize,
                        intsize: String::from("u64"),
                    }));
                }
            }
            None => {
                return Err(Error::from(NExistKeyError {
                    contents: format!("{}", _addr.to_string()),
                    structure: String::from("User balance book"),
                }))
            }
        }
    }
    pub fn debit_assets(&mut self, _addr: &Address, _amt: u64) -> Result<(), Error> {
        match self.balance_book.get_mut(_addr.to_string_ref()) {
            Some(val) => {
                if let Some(checked_diff) = val.assets.checked_sub(_amt) {
                    val.assets = checked_diff;
                    Ok(())
                } else {
                    return Err(Error::from(IntUnderflowError {
                        origin: String::from("debit asset balance method"),
                        fst: val.assets as usize,
                        snd: _amt as usize,
                        intsize: String::from("u64"),
                    }));
                }
            }
            None => {
                return Err(Error::from(NExistKeyError {
                    contents: format!("{}", _addr.to_string()),
                    structure: String::from("User balance book"),
                }))
            }
        }
    }
    pub fn debit_hold_cash(&mut self, _addr: &Address, _amt: u64) -> Result<(), Error> {
        match self.balance_book.get_mut(_addr.to_string_ref()) {
            Some(val) => {
                if let Some(checked_diff) = val.hold_cash.checked_sub(_amt) {
                    val.hold_cash = checked_diff;
                    Ok(())
                } else {
                    return Err(Error::from(IntUnderflowError {
                        origin: String::from("debit hold_cash balance method"),
                        fst: val.hold_cash as usize,
                        snd: _amt as usize,
                        intsize: String::from("u64"),
                    }));
                }
            }
            None => {
                return Err(Error::from(NExistKeyError {
                    contents: format!("{}", _addr.to_string()),
                    structure: String::from("User balance book"),
                }))
            }
        }
    }
    pub fn debit_hold_assets(&mut self, _addr: &Address, _amt: u64) -> Result<(), Error> {
        match self.balance_book.get_mut(_addr.to_string_ref()) {
            Some(val) => {
                if let Some(checked_diff) = val.hold_assets.checked_sub(_amt) {
                    val.hold_assets = checked_diff;
                    Ok(())
                } else {
                    return Err(Error::from(IntUnderflowError {
                        origin: String::from("debit hold_assets balance method"),
                        fst: val.hold_cash as usize,
                        snd: _amt as usize,
                        intsize: String::from("u64"),
                    }));
                }
            }
            None => {
                return Err(Error::from(NExistKeyError {
                    contents: format!("{}", _addr.to_string()),
                    structure: String::from("User balance book"),
                }))
            }
        }
    }
}
