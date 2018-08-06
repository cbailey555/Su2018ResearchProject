use sawtooth_sdk::processor::handler::{ApplyError, ContextError, TransactionContext};
use std::collections::HashMap;
#[allow(unused_imports)]
#[allow(dead_code)]
use std::fs::File;
use std::io::prelude::*;

use serde_cbor;

use errors::LibError;
use failure::Error;

use addressing;
use auction::{Auction, AuctionList, Bid};
use balancebook::BalanceBook;
use order::{BuyOrder, OrderT, SellOrder};
use orderbook::{fill_buy, fill_sell, OrderBook};
use sealedbid::SealedAuctionList;
use useracct::UserAccount;
use wrapper::Wrapper;

pub struct FamilyState<'a> {
    context: &'a mut TransactionContext,
}

impl<'a> FamilyState<'a> {
    pub fn new(context: &'a mut TransactionContext) -> FamilyState {
        FamilyState { context: context }
    }

    pub fn get_balancebook(&mut self) -> Result<BalanceBook, ApplyError> {
        let ser_balancebook_response = self.context.get_state(addressing::BALANCEBOOK)?;
        match ser_balancebook_response {
            Some(contents) => {
                let deser_balancebook: BalanceBook = match serde_cbor::from_slice(&contents[0..]) {
                    Ok(v) => v,
                    Err(e) => return Err(ApplyError::InternalError(format!("Error deserializing balancebook (serde_cbor::from_slice) in geset: {:?}\n.", e)))
                };
                Ok(deser_balancebook)
            }
            None => Ok(BalanceBook::new()),
        }
    }

    pub fn get_orderbook(&mut self) -> Result<OrderBook, ApplyError> {
        let ser_orderbook_response = self.context.get_state(addressing::ORDERBOOK)?;
        match ser_orderbook_response {
            Some(contents) => {
                let deser_orderbook: OrderBook = match serde_cbor::from_slice(&contents[0..]) {
                    Ok(v) => v,
                    Err(e) => return Err(ApplyError::InternalError(format!(
                        "Error deserializing orderbook (serde_cbor::from_slice) in geset: {:?}\n.",
                        e
                    ))),
                };

                Ok(deser_orderbook)
            }
            None => Ok(OrderBook::new()),
        }
    }

    pub fn get_auctionlist(&mut self) -> Result<AuctionList, ApplyError> {
        let ser_auctionlist_response = self.context.get_state(addressing::AUCTIONLIST)?;
        match ser_auctionlist_response {
            Some(contents) => {
                let deser_auctionlist: AuctionList = match serde_cbor::from_slice(&contents[0..]) {
                    Ok(v) => v,
                    Err(e) => return Err(ApplyError::InternalError(format!(
                        "Error deserializing orderbook (serde_cbor::from_slice) in geset: {:?}\n.",
                        e
                    ))),
                };

                Ok(deser_auctionlist)
            }
            None => Ok(AuctionList::new()),
        }
    }

    pub fn get_sealed_auctionlist(&mut self) -> Result<SealedAuctionList, ApplyError> {
        let ser_auctionlist_response = self.context.get_state(addressing::SEALEDAUCTIONLIST)?;
        match ser_auctionlist_response {
            Some(contents) => {
                let deser_auctionlist: SealedAuctionList = match serde_cbor::from_slice(
                    &contents[0..],
                ) {
                    Ok(v) => v,
                    Err(e) => return Err(ApplyError::InternalError(format!(
                        "Error deserializing orderbook (serde_cbor::from_slice) in geset: {:?}\n.",
                        e
                    ))),
                };

                Ok(deser_auctionlist)
            }
            None => Ok(SealedAuctionList::new()),
        }
    }

    pub fn set_balancebook(&mut self, _balance_book: BalanceBook) -> Result<(), ApplyError> {
        let balancebook_cbor = serde_cbor::to_vec(&_balance_book)?;
        //        info!("location getset: calling set balancebook with: {}\n", _balance_book);
        match self
            .context
            .set_state(addressing::BALANCEBOOK, &balancebook_cbor)
        {
            Ok(_) => Ok(()),
            Err(e) => {
                return Err(ApplyError::InternalError(format!(
                    "failed to execute set_balancebook in getset module: {:?}\n",
                    e
                )))
            }
        }
    }

    pub fn set_orderbook(&mut self, _order_book: OrderBook) -> Result<(), ApplyError> {
        let orderbook_cbor = serde_cbor::to_vec(&_order_book)?;
        match self
            .context
            .set_state(addressing::ORDERBOOK, &orderbook_cbor)
        {
            Ok(_) => Ok(()),
            Err(e) => {
                return Err(ApplyError::InternalError(format!(
                    "failed to execute set_orderbook in getset module: {:?}\n",
                    e
                )))
            }
        }
    }

    pub fn set_auctionlist(&mut self, _auctionlist: AuctionList) -> Result<(), ApplyError> {
        let auctionlist_cbor = serde_cbor::to_vec(&_auctionlist)?;
        match self
            .context
            .set_state(addressing::AUCTIONLIST, &auctionlist_cbor)
        {
            Ok(_) => Ok(()),
            Err(e) => {
                return Err(ApplyError::InternalError(format!(
                    "failed to execute set_auctionlist in getset module: {:?}\n",
                    e
                )))
            }
        }
    }

    pub fn set_sealed_auctionlist(
        &mut self,
        _sealedlist: SealedAuctionList,
    ) -> Result<(), ApplyError> {
        let auctionlist_cbor = serde_cbor::to_vec(&_sealedlist)?;
        match self
            .context
            .set_state(addressing::SEALEDAUCTIONLIST, &auctionlist_cbor)
        {
            Ok(_) => Ok(()),
            Err(e) => {
                return Err(ApplyError::InternalError(format!(
                    "failed to execute set sealed auctionlist in getset module: {:?}\n",
                    e
                )))
            }
        }
    }

    pub fn get_cballotresult(&mut self) -> Result<u64, ApplyError> {
        let ser_cblist_response = self.context.get_state(addressing::CBALLOTBOXRESULT)?;
        match ser_cblist_response {
            Some(contents) => {
                let cballotresult: u64 = serde_cbor::from_slice(&contents[0..])?;
                Ok(cballotresult)
            }
            None => {
                return Err(ApplyError::InternalError(format!(
                    "No election has taken place yet!"
                )))
            }
        }
    }
}
