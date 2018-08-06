extern crate crypto;
extern crate dmktp;
extern crate failure;

use crypto::digest::Digest;
use crypto::sha2::Sha512;
use dmktp::address::Address;
use dmktp::addressing;
use dmktp::balancebook::BalanceBook;
use dmktp::errors::LibError;
use dmktp::sealedbid::{SealedAuction, SealedAuctionList, SealedBid, UnsealedBid};
use dmktp::useracct::UserAccount;
use failure::Error;
use std::cmp::Ordering;
use std::collections::BTreeMap;

mod common;

pub fn mk_useracct1() -> UserAccount {
    UserAccount::new_seeded(
        String::from("Alex"),
        String::from("1111111111111111111111111111111111111111"),
        1_000_000,
        1_000_000,
    ).unwrap()
}
pub fn mk_useracct2() -> UserAccount {
    UserAccount::new_seeded(
        String::from("Bob"),
        String::from("2222222222222222222222222222222222222222"),
        1_000_000,
        1_000_000,
    ).unwrap()
}
pub fn mk_useracct3() -> UserAccount {
    UserAccount::new_seeded(
        String::from("Charles"),
        String::from("3333333333333333333333333333333333333333"),
        1_000_000,
        1_000_000,
    ).unwrap()
}
pub fn mk_useracct4() -> UserAccount {
    UserAccount::new_seeded(
        String::from("Dave"),
        String::from("4444444444444444444444444444444444444444"),
        1_000_000,
        1_000_000,
    ).unwrap()
}

pub fn mk_addr1() -> Address {
    Address::from_string(String::from("1111111111111111111111111111111111111111")).unwrap()
}
pub fn mk_addr2() -> Address {
    Address::from_string(String::from("2222222222222222222222222222222222222222")).unwrap()
}
pub fn mk_addr3() -> Address {
    Address::from_string(String::from("3333333333333333333333333333333333333333")).unwrap()
}

#[test]
fn auction_t1() {
    let mut bb: BalanceBook = BalanceBook::new();
    bb.insert_seeded(mk_useracct1());
    bb.insert_seeded(mk_useracct2());
    bb.insert_seeded(mk_useracct3());
    bb.insert_seeded(mk_useracct4());

    let mut auctionlist = SealedAuctionList::new();

    let auction1: SealedAuction = SealedAuction::new(1, format!("test one"), true, 1_000, 3999);
    auctionlist.add_sealed_auction(auction1);

    let mut bid1: UnsealedBid = UnsealedBid::from_vals(
        mk_addr1(),
        1,
        100,
        String::from("a24a69af46b92d08c39815c543842d6ee6ebb094"),
    );
    let mut bid2: UnsealedBid = UnsealedBid::from_vals(
        mk_addr2(),
        1,
        200,
        String::from("4a3c967b2711fb55b857de9f73bea924c3773897"),
    );
    let mut bid3: UnsealedBid = UnsealedBid::from_vals(
        mk_addr3(),
        1,
        500,
        String::from("0505fbdf3288ea824aefc010e66521adc2eb739c"),
    );

    let mut sbid1 = bid1.to_sealed_bid();
    let mut sbid2 = bid2.to_sealed_bid();
    let mut sbid3 = bid3.to_sealed_bid();

    auctionlist.submit_sealed_bid(sbid1).unwrap();
    auctionlist.submit_sealed_bid(sbid2).unwrap();
    auctionlist.submit_sealed_bid(sbid3).unwrap();

    auctionlist.submit_unsealed_bid(bid1).unwrap();
    auctionlist.submit_unsealed_bid(bid2).unwrap();
    auctionlist.submit_unsealed_bid(bid3).unwrap();

    auctionlist.end_auction(&mut bb, 1).unwrap();

    println!("auctionlist: {}\n", auctionlist);
    println!("balancebook: {}\n", bb);

    assert_eq!(1, 1);
}
