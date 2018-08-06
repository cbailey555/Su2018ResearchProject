use failure::Error;
use sawtooth_sdk::messages::transaction::Transaction;
use serde_cbor;
use std::sync::{Mutex, MutexGuard};

use addressing::{
    self, AUCTIONLIST, BALANCEBOOK, CBALLOTBOXRESULT, FAMILY_NAME, FAMILY_PREFIX, FAMILY_VERSION,
    ORDERBOOK, SEALEDAUCTIONLIST,
};
use adminmsg::AdminMsg;
use auction::{Auction, Bid};
use balancebook::BalanceBook;
use errors::LibError;
use getset::FamilyState;
use order::{BuyOrder, OrderT, SellOrder};
use orderbook::OrderBook;
use sealedbid::{SealedAuction, SealedBid, UnsealedBid};
use swth_cli_libv2::clireq::{CliRequest, ClientRequest, FamilyMeta};
use swth_cli_libv2::errors::CliError;
use useracct::UserAccount;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum Wrapper {
    Ua(UserAccount),
    Bo(BuyOrder),
    So(SellOrder),
    Am(AdminMsg),
    Na(Auction),
    Ns(SealedAuction),
    Sb(SealedBid),
    Ub(UnsealedBid),
    Es(u64),
    Ab(Bid),
    Ea(u64),
    Co,
    Cb,
    Ca,
    Cs,
    //    AuctionBid(Bid),
    //    BuyOrderVec(Vec<BuyOrder>),
    //    SellOrderVec(Vec<SellOrder>),
    //    UserAccountVec(Vec<UserAccount>),
}

impl ClientRequest for Wrapper {
    fn to_cli_request(&self) -> Result<CliRequest, Error> {
        let serialized = serde_cbor::to_vec(&self)?;
        match self {
            Wrapper::Ua(_acct) => Ok(CliRequest {
                cbor_payload: serialized,
                family_meta: FamilyMeta::from(FAMILY_NAME, FAMILY_PREFIX, FAMILY_VERSION),
                input_addrs: vec![String::from(ORDERBOOK), String::from(BALANCEBOOK)],
                output_addrs: vec![String::from(ORDERBOOK), String::from(BALANCEBOOK)],
            }),
            Wrapper::Bo(_buyorder) => Ok(CliRequest {
                cbor_payload: serialized,
                family_meta: FamilyMeta::from(FAMILY_NAME, FAMILY_PREFIX, FAMILY_VERSION),
                input_addrs: vec![String::from(BALANCEBOOK), String::from(ORDERBOOK)],
                output_addrs: vec![String::from(BALANCEBOOK), String::from(ORDERBOOK)],
            }),
            Wrapper::So(_sellorder) => Ok(CliRequest {
                cbor_payload: serialized,
                family_meta: FamilyMeta::from(FAMILY_NAME, FAMILY_PREFIX, FAMILY_VERSION),
                input_addrs: vec![String::from(BALANCEBOOK), String::from(ORDERBOOK)],
                output_addrs: vec![String::from(BALANCEBOOK), String::from(ORDERBOOK)],
            }),
            Wrapper::Co => Ok(CliRequest {
                cbor_payload: serialized,
                family_meta: FamilyMeta::from(FAMILY_NAME, FAMILY_PREFIX, FAMILY_VERSION),
                input_addrs: vec![String::from(ORDERBOOK)],
                output_addrs: vec![String::from(ORDERBOOK)],
            }),
            Wrapper::Cb => Ok(CliRequest {
                cbor_payload: serialized,
                family_meta: FamilyMeta::from(FAMILY_NAME, FAMILY_PREFIX, FAMILY_VERSION),
                input_addrs: vec![String::from(BALANCEBOOK)],
                output_addrs: vec![String::from(BALANCEBOOK)],
            }),
            Wrapper::Na(_auction) => Ok(CliRequest {
                cbor_payload: serialized,
                family_meta: FamilyMeta::from(FAMILY_NAME, FAMILY_PREFIX, FAMILY_VERSION),
                input_addrs: vec![String::from(AUCTIONLIST), String::from(BALANCEBOOK)],
                output_addrs: vec![String::from(AUCTIONLIST), String::from(BALANCEBOOK)],
            }),
            Wrapper::Ab(_bid) => Ok(CliRequest {
                cbor_payload: serialized,
                family_meta: FamilyMeta::from(FAMILY_NAME, FAMILY_PREFIX, FAMILY_VERSION),
                input_addrs: vec![String::from(AUCTIONLIST), String::from(BALANCEBOOK)],
                output_addrs: vec![String::from(AUCTIONLIST), String::from(BALANCEBOOK)],
            }),
            Wrapper::Ea(_serial) => Ok(CliRequest {
                cbor_payload: serialized,
                family_meta: FamilyMeta::from(FAMILY_NAME, FAMILY_PREFIX, FAMILY_VERSION),
                input_addrs: vec![String::from(AUCTIONLIST), String::from(BALANCEBOOK)],
                output_addrs: vec![String::from(AUCTIONLIST), String::from(BALANCEBOOK)],
            }),
            Wrapper::Ns(_sealedauction) => Ok(CliRequest {
                cbor_payload: serialized,
                family_meta: FamilyMeta::from(FAMILY_NAME, FAMILY_PREFIX, FAMILY_VERSION),
                input_addrs: vec![
                    String::from(SEALEDAUCTIONLIST),
                    String::from(CBALLOTBOXRESULT),
                ],
                output_addrs: vec![String::from(SEALEDAUCTIONLIST)],
            }),
            Wrapper::Sb(_sealedbid) => Ok(CliRequest {
                cbor_payload: serialized,
                family_meta: FamilyMeta::from(FAMILY_NAME, FAMILY_PREFIX, FAMILY_VERSION),
                input_addrs: vec![String::from(SEALEDAUCTIONLIST)],
                output_addrs: vec![String::from(SEALEDAUCTIONLIST)],
            }),
            Wrapper::Ub(_unsealedbid) => Ok(CliRequest {
                cbor_payload: serialized,
                family_meta: FamilyMeta::from(FAMILY_NAME, FAMILY_PREFIX, FAMILY_VERSION),
                input_addrs: vec![String::from(SEALEDAUCTIONLIST)],
                output_addrs: vec![String::from(SEALEDAUCTIONLIST)],
            }),
            Wrapper::Es(_serial) => Ok(CliRequest {
                cbor_payload: serialized,
                family_meta: FamilyMeta::from(FAMILY_NAME, FAMILY_PREFIX, FAMILY_VERSION),
                input_addrs: vec![String::from(SEALEDAUCTIONLIST), String::from(BALANCEBOOK)],
                output_addrs: vec![String::from(SEALEDAUCTIONLIST), String::from(BALANCEBOOK)],
            }),
            Wrapper::Cs => Ok(CliRequest {
                cbor_payload: serialized,
                family_meta: FamilyMeta::from(FAMILY_NAME, FAMILY_PREFIX, FAMILY_VERSION),
                input_addrs: vec![String::from(SEALEDAUCTIONLIST)],
                output_addrs: vec![String::from(SEALEDAUCTIONLIST)],
            }),

            _ => Ok(CliRequest {
                cbor_payload: serialized,
                family_meta: FamilyMeta::from(FAMILY_NAME, FAMILY_PREFIX, FAMILY_VERSION),
                input_addrs: vec![],
                output_addrs: vec![],
            }),
        }
    }
}
