use address::Address;
use addressing;
use balancebook::BalanceBook;
use crypto::digest::Digest;
use crypto::sha2::Sha512;
use errors::LibError;
use failure::Error;
use std::cmp::Ordering;
use std::collections::BTreeMap;
use std::fmt;
use useracct::UserAccount;

// actual bid that goes in just needs to be a hash digest.
// The reveal thing is the struct.

// Needs serial for routing. No addr keeps anonymous.

// Need the salt to prevent collision attacks which would otherwise
// be able to discern the bidder/price given powerful enough hardware.

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct SealedBid {
    pub serial: u64,
    pub digest: String,
}

impl SealedBid {
    pub fn from_vals(_serial: u64, _digest: String) -> Self {
        SealedBid {
            serial: _serial,
            digest: _digest,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, PartialOrd, Clone)]
pub struct UnsealedBid {
    pub address: Address,
    pub serial: u64,
    pub price: u64,
    pub salt: String,
}

impl fmt::Display for UnsealedBid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "address: {} \n", self.address.to_string_ref())?;
        write!(f, "serial: {} \n", self.serial)?;
        write!(f, "price: {} \n", self.price)
    }
}

impl UnsealedBid {
    pub fn from_vals(_addr: Address, _serial: u64, _price: u64, _salt: String) -> Self {
        UnsealedBid {
            address: _addr,
            serial: _serial,
            price: _price,
            salt: _salt,
        }
    }

    pub fn to_sealed_bid(&self) -> SealedBid {
        SealedBid {
            serial: self.serial,
            digest: self.get_self_hash(),
        }
    }

    pub fn stringify(&self) -> String {
        format!("{}{}{}{}", self.address, self.serial, self.price, self.salt)
    }

    pub fn check_hash(&self, _sealed: &String) -> bool {
        let mut hasher = Sha512::new();
        hasher.input(&self.stringify().as_bytes());
        let hash_result_string = hasher.result_str();
        if &hash_result_string == _sealed {
            true
        } else {
            false
        }
    }

    pub fn get_self_hash(&self) -> String {
        let mut hasher = Sha512::new();
        hasher.input(&self.stringify().as_bytes());
        hasher.result_str()
    }
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct SealedAuctionList {
    pub contents: BTreeMap<u64, SealedAuction>,
    pub total_auctioned: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct SealedAuction {
    pub serial: u64,
    pub description: String,
    pub is_open: bool,
    pub auction_amt: u64,
    pub leader: (Option<Address>, u64),
    pub second_price: Option<u64>,
    pub bid_pool: BTreeMap<String, Option<UnsealedBid>>,
    pub price_pool: BTreeMap<u64, ()>,
    pub end_date: u64,
}

impl fmt::Display for SealedAuction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let leader0 = match self.leader.0.clone() {
            None => format!("None"),
            Some(v) => format!("{}", v),
        };
        write!(f, "serial: {} \n", self.serial)?;
        write!(f, "description: {} \n", self.description)?;
        write!(f, "is open: {} \n", self.is_open)?;
        write!(f, "auction qty: {} \n", self.auction_amt)?;
        write!(f, "leading bid: {}, {} \n", leader0, self.leader.1)?;
        write!(f, "second price: {:?}\n", self.second_price)?;
        write!(f, "bid pool: {:#?}\n", self.bid_pool)?;
        write!(f, "end_date: {}\n", self.end_date)
    }
}

impl fmt::Display for SealedAuctionList {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.contents
            .values()
            .try_for_each(|x| write!(f, "\n{}\n", x))
    }
}

impl SealedAuctionList {
    pub fn new() -> Self {
        SealedAuctionList {
            contents: BTreeMap::new(),
            total_auctioned: 0,
        }
    }

    pub fn add_sealed_auction(&mut self, _a: SealedAuction) {
        self.contents.insert(_a.serial, _a);
    }

    pub fn get_total_auctioned(&self) -> u64 {
        self.total_auctioned
    }

    pub fn submit_sealed_bid(&mut self, _sealed: SealedBid) -> Result<(), Error> {
        match self.contents.get_mut(&_sealed.serial) {
            None => {
                return Err(Error::from(LibError::CustomError {
                    contents: String::from("No auction found with that serial number."),
                }))
            }
            Some(v) => v.bid_pool.insert(_sealed.digest, None),
        };

        Ok(())
    }

    pub fn submit_unsealed_bid(&mut self, _unsealed: UnsealedBid) -> Result<(), Error> {
        match self.contents.get_mut(&_unsealed.serial) {
            None => {
                return Err(Error::from(LibError::CustomError {
                    contents: String::from("No auction found with that serial number."),
                }))
            }
            Some(v) => v.insert_unsealed_and_check(_unsealed)?,
        };

        Ok(())
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

        target_auction.end_auction(_balance_book)?;
        Ok(())
    }
}

impl SealedAuction {
    // get address/caller identity from payload signer public key
    pub fn new(
        _serial: u64,
        _description: String,
        _start_open: bool,
        _amt: u64,
        _end_date: u64,
    ) -> Self {
        SealedAuction {
            serial: _serial,
            description: _description,
            is_open: _start_open,
            auction_amt: _amt,
            leader: (None, 0),
            second_price: None,
            bid_pool: BTreeMap::new(),
            price_pool: BTreeMap::new(),
            end_date: _end_date,
        }
    }

    pub fn insert_unsealed_and_check(&mut self, _unsealed: UnsealedBid) -> Result<(), Error> {
        let hash_of_unsealed: String = _unsealed.clone().get_self_hash();
        match self.bid_pool.contains_key(&hash_of_unsealed) {
            false => {
                return Err(Error::from(LibError::CustomError {
                    contents: format!(
                        "There was no sealed bid found which corresopnds to your unsealed bid."
                    ),
                }))
            }
            true => {
                let current_leader = self.leader.clone();
                if _unsealed.price > current_leader.1 {
                    self.leader = (Some(_unsealed.clone().address), _unsealed.price)
                };
                self.price_pool.insert(_unsealed.price, ());
                self.bid_pool.insert(hash_of_unsealed, Some(_unsealed));
            }
        }

        return Ok(());
    }

    pub fn end_auction(&mut self, _balance_book: &mut BalanceBook) -> Result<(), Error> {
        self.is_open = false;
        let second_price = match self.price_pool.len() {
            0 => {
                return Err(Error::from(LibError::CustomError {
                    contents: format!("No one entered the auction! Closing with no payout."),
                }))
            }
            1 => self.price_pool.keys().next().expect(
                "got none in end_auction method. This expect call should have been qualified.",
            ),
            _ => {
                let mut key_iter = self.price_pool.keys();
                key_iter.next_back();
                key_iter.next_back().expect(
                    "got none in end_auction method. This expect call should have been qualified.",
                )
            }
        };

        self.second_price = Some(second_price.clone());

        let winning_addr = self.leader.0.clone().expect("Got 'none' in end auction -> define winning addr prior to balancebook changes. This unwrap should have been checked");
        _balance_book.debit_cash(&winning_addr, *second_price)?;
        _balance_book.credit_assets(&winning_addr, self.auction_amt)?;

        Ok(())
    }
}

impl Eq for SealedBid {}

impl PartialOrd for SealedBid {
    fn partial_cmp(&self, other: &SealedBid) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for SealedBid {
    fn cmp(&self, other: &SealedBid) -> Ordering {
        match self.digest == other.digest {
            true => Ordering::Equal,
            false => match self.digest < other.digest {
                true => Ordering::Less,
                false => Ordering::Greater,
            },
        }
    }
}
