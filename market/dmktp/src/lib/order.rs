use std::cmp::Ordering;
use std::fmt;
use std::u64::MAX;
use swth_cli_libv2::clireq::{CliRequest, ClientRequest};

use address::Address;
use balancebook::BalanceBook;
use failure::Error;
use orderbook::{self, fill_buy, fill_sell, OrderBook};
use serde_cbor;
use wrapper::Wrapper;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct BuyOrder {
    pub addr: Address,
    pub price: u64,
    pub qty: u64,
    pub nonce: u64,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct SellOrder {
    pub addr: Address,
    pub price: u64,
    pub qty: u64,
    pub nonce: u64,
}

pub trait OrderT {
    fn from_vals(_addr: Address, _price: u64, _qty: u64, _nonce: u64) -> Self;
    fn get_addr(&self) -> Address;
    fn get_addr_ref(&self) -> &Address;
    fn get_qty(&self) -> u64;
    fn get_price(&self) -> u64;
    fn get_nonce(&self) -> u64;
    fn set_nonce(&mut self, _nonce: u64);
    fn dec_qty_by(&mut self, _dec_qty: u64);
    fn wrap(self) -> Wrapper;
    fn execute(self, _bb: &mut BalanceBook, _ob: &mut OrderBook) -> Result<(), Error>;
}

impl OrderT for BuyOrder {
    fn from_vals(_addr: Address, _price: u64, _qty: u64, _nonce: u64) -> Self {
        BuyOrder {
            addr: _addr,
            price: _price,
            qty: _qty,
            nonce: _nonce,
        }
    }
    fn get_addr(&self) -> Address {
        self.addr.clone()
    }
    fn get_addr_ref(&self) -> &Address {
        &self.addr
    }
    fn get_qty(&self) -> u64 {
        self.qty
    }
    fn get_price(&self) -> u64 {
        self.price
    }
    fn get_nonce(&self) -> u64 {
        self.nonce
    }

    fn dec_qty_by(&mut self, _dec_qty: u64) {
        self.qty -= _dec_qty;
    }

    fn set_nonce(&mut self, _nonce: u64) {
        self.nonce = _nonce;
    }

    fn wrap(self) -> Wrapper {
        Wrapper::Bo(self)
    }

    fn execute(self, _bb: &mut BalanceBook, _ob: &mut OrderBook) -> Result<(), Error> {
        fill_buy(_ob, _bb, self)
    }
}

impl OrderT for SellOrder {
    fn from_vals(_addr: Address, _price: u64, _qty: u64, _nonce: u64) -> Self {
        SellOrder {
            addr: _addr,
            price: _price,
            qty: _qty,
            nonce: _nonce,
        }
    }

    fn get_addr(&self) -> Address {
        self.addr.clone()
    }
    fn get_addr_ref(&self) -> &Address {
        &self.addr
    }
    fn get_qty(&self) -> u64 {
        self.qty
    }
    fn get_price(&self) -> u64 {
        self.price
    }
    fn get_nonce(&self) -> u64 {
        self.nonce
    }

    fn dec_qty_by(&mut self, _dec_qty: u64) {
        self.qty -= _dec_qty;
    }

    fn set_nonce(&mut self, _nonce: u64) {
        self.nonce = _nonce;
    }

    fn wrap(self) -> Wrapper {
        Wrapper::So(self)
    }

    fn execute(self, _bb: &mut BalanceBook, _ob: &mut OrderBook) -> Result<(), Error> {
        fill_sell(_ob, _bb, self)
    }
}

// Implementation of ord/eq for orders to allow for judgment
// based on nonce when prices are equal.

impl Eq for BuyOrder {}
impl Eq for SellOrder {}

impl PartialOrd for BuyOrder {
    fn partial_cmp(&self, other: &BuyOrder) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialOrd for SellOrder {
    fn partial_cmp(&self, other: &SellOrder) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// BinaryHeap ordering
impl Ord for BuyOrder {
    fn cmp(&self, other: &BuyOrder) -> Ordering {
        match self.price == other.price {
            false => match self.price > other.price {
                true => Ordering::Greater,
                false => Ordering::Less,
            },
            true => match self.nonce < other.nonce {
                true => Ordering::Greater,
                false => Ordering::Less,
            },
        }
    }
}

impl Ord for SellOrder {
    fn cmp(&self, other: &SellOrder) -> Ordering {
        match self.price == other.price {
            false => match self.price < other.price {
                true => Ordering::Greater,
                false => Ordering::Less,
            },
            true => match self.nonce < other.nonce {
                true => Ordering::Greater,
                false => Ordering::Less,
            },
        }
    }
}

//impl Ord for BuyOrder {
//    fn cmp(&self, other: &BuyOrder) -> Ordering {
//        match self.price == other.price {
//            false => match self.price > other.price {
//                true => Ordering::Less,
//                false => Ordering::Greater
//            },
//            true => match self.nonce < other.nonce {
//                true => Ordering::Less,
//                false => Ordering::Greater
//            }
//        }
//    }
//}
//
//impl Ord for SellOrder {
//    fn cmp(&self, other: &SellOrder) -> Ordering {
//        match self.price == other.price {
//            false => match self.price < other.price {
//                true => Ordering::Less,
//                false => Ordering::Greater
//            },
//            true => match self.nonce < other.nonce {
//                true => Ordering::Less,
//                false => Ordering::Greater
//            }
//        }
//    }
//}

impl fmt::Display for BuyOrder {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Buy Order\nOrigin address: {}\nUnit Price: {}\nQuantity: {}\nNonce: {}\n",
            self.addr, self.price, self.qty, self.nonce
        )
    }
}

impl fmt::Display for SellOrder {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "SellOrder\nOrigin address: {}\nUnit Price: {}\nQuantity: {}\nNonce: {}\n",
            self.addr, self.price, self.qty, self.nonce
        )
    }
}
