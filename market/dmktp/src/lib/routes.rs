use sawtooth_sdk::processor::handler::{ApplyError, TransactionContext};

use auction::{Auction, AuctionList, Bid};
use balancebook::BalanceBook;
use getset::FamilyState;
use order::{BuyOrder, OrderT, SellOrder};
use orderbook::OrderBook;
use sealedbid::{SealedAuction, SealedAuctionList, SealedBid, UnsealedBid};
use useracct::UserAccount;

pub const CAPXTEN: u64 = 10_000_000_000;

pub fn acct_route(_fstate: &mut FamilyState, _acct: UserAccount) -> Result<(), ApplyError> {
    let mut bb = _fstate.get_balancebook()?;
    match bb.insert_seeded(_acct) {
        Ok(v) => (),
        Err(e) => {
            return Err(ApplyError::InternalError(format!(
                "Error inserting user account into balanceboook in acc_route: {:?}\n",
                e
            )))
        }
    }
    _fstate.set_balancebook(bb)
}

pub fn bo_route(_fstate: &mut FamilyState, _buyorder: BuyOrder) -> Result<(), ApplyError> {
    let mut bb = _fstate.get_balancebook()?;
    let mut ob = _fstate.get_orderbook()?;
    match _buyorder.execute(&mut bb, &mut ob) {
        Ok(v) => (),
        Err(e) => {
            return Err(ApplyError::InternalError(format!(
                "Error calling 'execute' on buy order in routes -> bo_route: {:?}\n",
                e
            )))
        }
    }

    _fstate.set_balancebook(bb)?;
    _fstate.set_orderbook(ob)
}

pub fn so_route(_fstate: &mut FamilyState, _sellorder: SellOrder) -> Result<(), ApplyError> {
    let mut bb = _fstate.get_balancebook()?;
    let mut ob = _fstate.get_orderbook()?;
    match _sellorder.execute(&mut bb, &mut ob) {
        Ok(v) => (),
        Err(e) => {
            return Err(ApplyError::InternalError(format!(
                "Error calling 'execute' on sell order in routes -> so_route: {:?}\n",
                e
            )))
        }
    }

    _fstate.set_balancebook(bb)?;
    _fstate.set_orderbook(ob)
}

pub fn co_route(_fstate: &mut FamilyState) -> Result<(), ApplyError> {
    _fstate.set_orderbook(OrderBook::new())
}

pub fn cb_route(_fstate: &mut FamilyState) -> Result<(), ApplyError> {
    _fstate.set_balancebook(BalanceBook::new())
}

pub fn ca_route(_fstate: &mut FamilyState) -> Result<(), ApplyError> {
    _fstate.set_auctionlist(AuctionList::new())
}

pub fn na_route(_fstate: &mut FamilyState, _auction: Auction) -> Result<(), ApplyError> {
    let mut alist: AuctionList = _fstate.get_auctionlist()?;
    alist.add_auction(_auction);
    _fstate.set_auctionlist(alist)
}

//pub fn bid_route_compose(_fstate: &mut FamilyState, _bid: Bid) -> Result<(), ApplyError {
//    _fstate.get_auctionlist().and_then()
//}

pub fn ea_route(_fstate: &mut FamilyState, _serial: u64) -> Result<(), ApplyError> {
    let mut bb: BalanceBook = _fstate.get_balancebook()?;
    let mut alist: AuctionList = _fstate.get_auctionlist()?;
    match alist.end_auction(&mut bb, _serial) {
        Ok(v) => (),
        Err(e) => return Err(ApplyError::InternalError(format!("Unable to find auction with serial number corersponding to bid's serial number in bid route: {:?}\n", e)))
    }

    _fstate.set_auctionlist(alist)?;
    _fstate.set_balancebook(bb)
}

pub fn ab_route(_fstate: &mut FamilyState, _bid: Bid) -> Result<(), ApplyError> {
    let mut bb: BalanceBook = _fstate.get_balancebook()?;
    let mut alist: AuctionList = _fstate.get_auctionlist()?;
    match alist.place_bid(&mut bb, _bid) {
        Ok(v) => (),
        Err(e) => return Err(ApplyError::InternalError(format!("Unable to find auction with serial number corersponding to bid's serial number in bid route: {:?}\n", e)))
    }

    _fstate.set_auctionlist(alist)?;
    _fstate.set_balancebook(bb)
}

pub fn ns_route(
    _fstate: &mut FamilyState,
    _sealedauction: SealedAuction,
) -> Result<(), ApplyError> {
    let cbresult: u64 = _fstate.get_cballotresult()?;
    let imposed: u64 = (CAPXTEN * cbresult) / 100;
    let imposed_cap: (u64, u64) = (cbresult, imposed);
    info!("imposed cap is: {:?}\n", imposed_cap);
    let auction_amt = _sealedauction.auction_amt;
    info!("auction amt: {:?}\n", auction_amt);
    if auction_amt > imposed_cap.1 {
        return Err(ApplyError::InternalError(format!(
            "Cannot initiate auction for more credits than allotted by vote!"
        )));
    } else {
        info!("auction amoutn is okay. amount: {:?}\n Cap: {:?}\n", auction_amt, imposed_cap.1);
        let mut slist: SealedAuctionList = _fstate.get_sealed_auctionlist()?;
        slist.add_sealed_auction(_sealedauction);
        _fstate.set_sealed_auctionlist(slist)
    }
}

pub fn sb_route(_fstate: &mut FamilyState, _sealedbid: SealedBid) -> Result<(), ApplyError> {
    let mut slist: SealedAuctionList = _fstate.get_sealed_auctionlist()?;
    match slist.submit_sealed_bid(_sealedbid) {
        Ok(v) => (),
        Err(e) => {
            return Err(ApplyError::InternalError(format!(
                "error inserting sealed bid: {:?}\n",
                e
            )))
        }
    };
    _fstate.set_sealed_auctionlist(slist)
}

pub fn ub_route(_fstate: &mut FamilyState, _unsealedbid: UnsealedBid) -> Result<(), ApplyError> {
    let mut slist: SealedAuctionList = _fstate.get_sealed_auctionlist()?;
    match slist.submit_unsealed_bid(_unsealedbid) {
        Ok(v) => (),
        Err(e) => {
            return Err(ApplyError::InternalError(format!(
                "error inserting unsealed bid: {:?}\n",
                e
            )))
        }
    };
    _fstate.set_sealed_auctionlist(slist)
}

pub fn es_route(_fstate: &mut FamilyState, _serial: u64) -> Result<(), ApplyError> {
    let mut slist: SealedAuctionList = _fstate.get_sealed_auctionlist()?;
    let mut bb: BalanceBook = _fstate.get_balancebook()?;
    match slist.end_auction(&mut bb, _serial) {
        Ok(v) => (),
        Err(e) => {
            return Err(ApplyError::InternalError(format!(
                "error ending sealed bid auction: {:?}\n",
                e
            )))
        }
    }

    _fstate.set_balancebook(bb)?;
    _fstate.set_sealed_auctionlist(slist)
}

pub fn cs_route(_fstate: &mut FamilyState) -> Result<(), ApplyError> {
    _fstate.set_sealed_auctionlist(SealedAuctionList::new())
}
