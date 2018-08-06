use address::Address;
use addressing;
use balancebook::BalanceBook;
use errors::LibError;
use failure::Error;
use std::collections::BTreeMap;
use useracct::UserAccount;

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct AuctionList {
    pub contents: BTreeMap<u64, Auction>,
    pub total_auctioned: u64,
}

#[derive(Serialize, Deserialize, Debug, Hash, Clone, PartialEq)]
pub struct Auction {
    serial: u64,
    description: String,
    is_open: bool,
    auction_amt: u64,
    high_bidder: Address,
    high_bid: Option<Bid>,
    end_date: u64,
    history: Vec<Bid>,
}

#[derive(Serialize, Deserialize, Debug, Hash, Clone, PartialEq)]
pub struct Bid {
    pub addr: Address,
    pub auction_serial: u64,
    pub bid_amt: u64,
}

impl AuctionList {
    pub fn new() -> Self {
        AuctionList {
            contents: BTreeMap::new(),
            total_auctioned: 0,
        }
    }

    pub fn add_auction(&mut self, _a: Auction) {
        self.contents.insert(_a.serial, _a);
    }
}

impl Bid {
    pub fn from_vals(_addr: Address, _serial: u64, _amt: u64) -> Self {
        Bid {
            addr: _addr,
            auction_serial: _serial,
            bid_amt: _amt,
        }
    }
}

impl AuctionList {
    pub fn show_history(&self, _auction_serial: u64) -> Result<Vec<Bid>, Error> {
        let target_auction = match self.contents.get(&_auction_serial) {
            Some(v) => v,
            None => {
                return Err(Error::from(LibError::CustomError {
                    contents: String::from("No auction found with that serial number."),
                }))
            }
        };

        Ok(target_auction.history.clone())
    }

    pub fn get_total(&self) -> u64 {
        self.total_auctioned
    }

    pub fn show_high_bid(&self, _auction_serial: u64) -> Result<Option<Bid>, Error> {
        let target_auction = match self.contents.get(&_auction_serial) {
            Some(v) => v,
            None => {
                return Err(Error::from(LibError::CustomError {
                    contents: String::from("No auction found with that serial number."),
                }))
            }
        };

        match target_auction.high_bid.clone() {
            Some(current_high) => Ok(Some(current_high)),
            None => Ok(None),
        }
    }

    pub fn place_bid(&mut self, _balance_book: &mut BalanceBook, _bid: Bid) -> Result<(), Error> {
        let target_auction = match self.contents.get_mut(&_bid.auction_serial) {
            Some(v) => v,
            None => {
                return Err(Error::from(LibError::CustomError {
                    contents: String::from("No auction found with that serial number."),
                }))
            }
        };
        match target_auction.is_open {
            false => Err(Error::from(LibError::CustomError {
                contents: String::from("Auction is no longer open for bidding"),
            })),
            // auction still open
            true => {
                let bidder_liquid_cash: u64 = match _balance_book.get_by_addr(&_bid.addr) {
                    Some(acc) => acc.cash,
                    None => {
                        return Err(Error::from(LibError::CustomError {
                            contents: String::from("User could not be found."),
                        }))
                    }
                };

                let current_high_bid: u64 = match target_auction.high_bid {
                    Some(ref b) => b.bid_amt.clone(),
                    _ => 0,
                };

                match (bidder_liquid_cash >= _bid.bid_amt, _bid.bid_amt > current_high_bid) {
                    (false, false) => Err(Error::from(LibError::CustomError{ contents: String::from("Cannot bid more than you have in your account.")})),
                    (true, false) =>  Err(Error::from(LibError::CustomError{ contents: String::from("Your bid is less than the current highest bid.")})),
                    (false, true) => Err(Error::from(LibError::CustomError{ contents: String::from("Your bid would be the highest bid, but you can't bid more than you have in your account")})),
                    (true, true) => {
                        if let Some(bid) = target_auction.high_bid.clone() { target_auction.history.push(bid.clone()) };
                        let prev_high_bidder = target_auction.high_bidder.clone();
                        //change new high bidder's balances
                        _balance_book.debit_cash(&_bid.addr, _bid.bid_amt)?;
                        _balance_book.credit_hold_cash(&_bid.addr, _bid.bid_amt)?;
                        // credit previous high bidder's money
                        // Only execute this block is old bid was non-zero.
                        if current_high_bid > 0 {
                            _balance_book.credit_cash(&prev_high_bidder, current_high_bid)?;
                            _balance_book.debit_hold_cash(&prev_high_bidder, current_high_bid)?;
                        } else {
                            ();
                        }
                        target_auction.high_bidder = _bid.addr.clone();
                        target_auction.high_bid = Some(_bid);
                        Ok(())
                    }
                }
            } // end open auction behavior
        }
    }

    pub fn end_auction(
        &mut self,
        _balance_book: &mut BalanceBook,
        _serial: u64,
    ) -> Result<(), Error> {
        let target_auction = match self.contents.get_mut(&_serial) {
            Some(v) => v,
            None => {
                return Err(Error::from(LibError::CustomError {
                    contents: String::from("No auction found with that serial number."),
                }))
            }
        };
        target_auction.is_open = false;
        if let Some(ref mut high_bid) = target_auction.high_bid {
            _balance_book.debit_hold_cash(&target_auction.high_bidder, high_bid.bid_amt)?;
            _balance_book.credit_assets(&target_auction.high_bidder, target_auction.auction_amt)?;
            self.total_auctioned += target_auction.auction_amt;
            Ok(())
        } else {
            Ok(())
        }
    }
}

impl Auction {
    // get address/caller identity from payload signer public key
    pub fn new(
        _serial: u64,
        _description: String,
        _start_open: bool,
        _amt: u64,
        _end_date: u64,
    ) -> Self {
        Auction {
            serial: _serial,
            description: _description,
            is_open: _start_open,
            auction_amt: _amt,
            high_bidder: Address::from_pubkey_string(&String::from(addressing::ADMIN)),
            high_bid: None,
            end_date: _end_date,
            history: Vec::new(),
        }
    }

    pub fn end_auction(&mut self, _balance_book: &mut BalanceBook) -> Result<(), Error> {
        self.is_open = false;
        if let Some(ref mut high_bid) = self.high_bid {
            _balance_book.debit_hold_cash(&self.high_bidder, high_bid.bid_amt)?;
            _balance_book.credit_assets(&self.high_bidder, self.auction_amt)?;
            Ok(())
        } else {
            Ok(())
        }
    }
}
