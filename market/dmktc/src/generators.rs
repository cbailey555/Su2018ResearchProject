use dmktp::order::{ BuyOrder, SellOrder, OrderT };
use dmktp::useracct::UserAccount;
use dmktp::wrapper::Wrapper;
use dmktp::address::Address;
use dmktp::auction::{ Auction, Bid };
use dmktp::sealedbid::{ SealedBid, UnsealedBid, SealedAuction };
use utils::addr_from_keyfile;
use swth_cli_libv2::errors::CliError;


const ACCT0: &'static str = "025a96a6b38a4b852182aca678153779e646a2899efb1ebfff57cef3fffc421b16";
const ACCT1: &'static str = "039c73852149c673747a9e821d9ca2616cdf35dd70a4f8a136ae589290f8e80761";
const ACCT2: &'static str = "03bac8208500d201697375caf1a24a69af46b92d08c39815c543842d6ee6ebb094";
const ACCT3: &'static str = "0357195f96d72a839e40a7265a2beb83a0e1fbed85bc1bdc3d6b9101724b40698f";
const ACCT4: &'static str = "03523fed45b81e41656755fc3902071ec955dd189999d0a341a21d240ab5e2e97d";
const ACCT5: &'static str = "02f7feb50cc3de505f2d26b44c3de0c1b353a8e376fbabfabedbe05098feae00f3";
const ACCT6: &'static str = "03b1969e9f363a4a471e4a400c4a3c967b2711fb55b857de9f73bea924c3773897";
const ACCT7: &'static str = "03bca56ce9d9b9418fc4201d05026314b099de0589510cb91270a75ea24d03873f";
const ACCT8: &'static str = "0218fe2c174e6d34abed40d2720505fbdf3288ea824aefc010e66521adc2eb739c";
const ACCT9: &'static str = "026fa9b06fe0e7b77f5b4c2021608a525eb5d1dd0d0fb9041ec0583ccab9b4ca1b";
const ACCT10: &'static str = "02fd42bc8eecccf31e7465051142b2719c2cb76c57b07dbcc70055539265518ff4";
const ACCT11: &'static str = "022109f0aab6ac52d1b5916ccf41daffe880b66048a157ae0c9446c84fac0d90a9";
const ACCT12: &'static str = "02a7a4fcfd500521fa2f9a1856a7bc5f8938616e358edcdec620e6906224a71f59";
const ACCT13: &'static str = "02263ce748170b74f7cc0449eb2560da6e44587985e1a35f558e78ecc1f0a8fb51";
const ACCT14: &'static str = "03841962e411bcc670e056029d463481ec54ac7cf563da8ffdfc8703529418ea4a";
const ACCT15: &'static str = "0232c59dfd92334d42890874904ffd861a8303f08a61b270eb0013248a840d38fa";
const ACCT16: &'static str = "03fca91b84d462913cff4fabbe41f550531d1e5bfd82693ef0f5e4ea949d18091d";
const ACCT17: &'static str = "02950b10eb3c3f18d3a215c8499d1d9839bde653348f4bc9b9a1fd6b781a1142bc";
const ACCT18: &'static str = "02ca3a9a9d73182634ff1773f930d403ce119f4e0d1a66ea6f004ec17a7fc79bf4";
const ACCT19: &'static str = "02f4fef2e5573894df65b00deeecc2f37068216057407a877a56955bc33aa53992";

pub fn all_accts_vec() -> Vec<String> {
    vec![
    String::from(ACCT0),
    String::from(ACCT1),
    String::from(ACCT2),
    String::from(ACCT3),
    String::from(ACCT4),
    String::from(ACCT5),
    String::from(ACCT6),
    String::from(ACCT7),
    String::from(ACCT8),
    String::from(ACCT9),
    String::from(ACCT10),
    String::from(ACCT11),
    String::from(ACCT12),
    String::from(ACCT13),
    String::from(ACCT14),
    String::from(ACCT15),
    String::from(ACCT16),
    String::from(ACCT17),
    String::from(ACCT18),
    String::from(ACCT19),
    ]
}

pub fn all_addrs_vec() -> Vec<Address> {
    all_accts_vec().iter_mut().map(|x| Address::from_pubkey_string(x)).collect()
}


pub fn all_names() -> Vec<String> {
    vec![
    String::from("Alex Ahad"),
    String::from("Bob Bosenbeck"),
    String::from("Charles Carol"),
    String::from("Dave Drubeck"),
    String::from("Erich Elran"),
    String::from("Fwa Feebwo"),
    String::from("Gertrud Geld"),
    String::from("Herbert Hathaway"),
    String::from("Ibert Ilyria"),
    String::from("JJ Johnson"),
    String::from("Klaus Kmini"),
    String::from("Leo Leeroy"),
    String::from("Michael McMee"),
    String::from("Nonon Nbique"),
    String::from("Oro Oboqua"),
    String::from("Peter Pipan"),
    String::from("Quint Quail"),
    String::from("Roger Rigo"),
    String::from("Steven Shay"),
    String::from("Tyler Tumumay")
    ]
}
    
pub fn all_useraccts() -> Vec<UserAccount> {
    let keys = all_addrs_vec();
    let names = all_names();
    let zipmap = names.into_iter().zip(keys).map(|x| UserAccount::from_vals(x.0, x.1.to_string()).unwrap());
    let collected: Vec<UserAccount> = zipmap.collect();

    collected
}

pub fn all_wrapped_useraccts() -> Vec<Wrapper> {
    let keys = all_addrs_vec();
    let names = all_names();
    let zipmap = names.into_iter().zip(keys).map(|x| Wrapper::Ua(UserAccount::from_vals(x.0, x.1.to_string()).unwrap()));
    let collected: Vec<Wrapper> = zipmap.collect();

    collected
}

pub fn all_seeded_wrapped() -> Vec<Wrapper> {
    let keys = all_addrs_vec();
    let names = all_names();
    names.into_iter().zip(keys).map(|x| Wrapper::Ua(UserAccount::new_seeded(x.0, x.1.to_string(), 500_000_000, 500_000_000).unwrap())).collect::<Vec<Wrapper>>()
}

pub fn gensell1() -> Vec<Wrapper> {
    let mut buffer: Vec<Wrapper> = Vec::new();
    let all_addrs = all_addrs_vec();

    for i in 1..21 {
        buffer.push(Wrapper::So(SellOrder::from_vals(all_addrs[1].clone(), i, i, i)))
    }

    buffer
}

pub fn gensell2() -> Vec<Wrapper> {
    let mut buffer: Vec<Wrapper> = Vec::new();
    let all_addrs = all_addrs_vec();

    for i in 21..41 {
        buffer.push(Wrapper::So(SellOrder::from_vals(all_addrs[1].clone(), i, i, i)))
    }

    buffer
}

pub fn gensell3() -> Vec<Wrapper> {
    let mut buffer: Vec<Wrapper> = Vec::new();
    let all_addrs = all_addrs_vec();

    for i in 41..61 {
        buffer.push(Wrapper::So(SellOrder::from_vals(all_addrs[1].clone(), i, i, i)))
    }

    buffer
}

pub fn genbuy1() -> Vec<Wrapper> {
    let mut buffer: Vec<Wrapper> = Vec::new();
    let all_addrs = all_addrs_vec();

    for i in 1..21 {
        buffer.push(Wrapper::Bo(BuyOrder::from_vals(all_addrs[2].clone(), i, i, i)))
    }

    buffer
}

pub fn genbuy2() -> Vec<Wrapper> {
    let mut buffer: Vec<Wrapper> = Vec::new();
    let all_addrs = all_addrs_vec();

    for i in 21..41 {
        buffer.push(Wrapper::Bo(BuyOrder::from_vals(all_addrs[2].clone(), i, i, i)))
    }

    buffer
}


pub fn genbuy3() -> Vec<Wrapper> {
    let mut buffer: Vec<Wrapper> = Vec::new();
    let all_addrs = all_addrs_vec();

    for i in 41..61 {
        buffer.push(Wrapper::Bo(BuyOrder::from_vals(all_addrs[2].clone(), i, i, i)))
    }

    buffer
}

pub fn genoddbuy1() -> Vec<Wrapper> {
    let mut buffer: Vec<Wrapper> = Vec::new();
    let all_addrs = all_addrs_vec();

    for i in 30..50 {
        buffer.push(Wrapper::Bo(BuyOrder::from_vals(all_addrs[2].clone(), i, i, i)))
    }

    buffer
}

pub fn genoddsell1() -> Vec<Wrapper> {
    let mut buffer: Vec<Wrapper> = Vec::new();
    let all_addrs = all_addrs_vec();

    for i in 12..32 {
        buffer.push(Wrapper::So(SellOrder::from_vals(all_addrs[1].clone(), i, i, i)))
    }

    buffer
}

pub fn gen_auction() -> Vec<Wrapper> {
    let auction: Auction = Auction::new(1, format!("Initial auction"), true, 1_000_000, 3_000_000_000);
    vec![Wrapper::Na(auction)]
}

pub fn gen_sealed_auction1() -> Wrapper {
    let auction: SealedAuction = SealedAuction::new(1, String::from("demo sealed price 2nd bid auction #01"), true, 1_000_000, 1533453523);
    Wrapper::Ns(auction)
}

pub fn gen_sealed_auction2() -> Wrapper {
    let auction: SealedAuction = SealedAuction::new(2, String::from("demo sealed price 2nd bid auction #02"), true, 2_000_000, 1533453523);
    Wrapper::Ns(auction)
}

pub fn gen_sealed_auction_ng(amt: u64) -> Wrapper {
    let auction: SealedAuction = SealedAuction::new(3, String::from("demo sealed price 2nd bid auction #03"), true, amt, 1533453523);
    Wrapper::Ns(auction)
}


pub fn gen_unsealed_bid1(_keyfile: Option<&str>) -> Result<Wrapper, CliError> {
    let addr = match addr_from_keyfile(_keyfile) {
        Ok(v) => v,
        Err(e) => return Err(CliError::CustomError { contents: format!("error creating unsealed bid due to keyfile management issues: {}\n", e)})
    };

    let ubid: UnsealedBid = UnsealedBid::from_vals(addr, 1, 1_000, format!("21fa2f9a1856a7bc5f8938616e358edc"));
    Ok(Wrapper::Ub(ubid))
}

pub fn gen_unsealed_bid2(_keyfile: Option<&str>) -> Result<Wrapper, CliError> {
    let addr = match addr_from_keyfile(_keyfile) {
        Ok(v) => v,
        Err(e) => return Err(CliError::CustomError { contents: format!("error creating unsealed bid due to keyfile management issues: {}\n", e)})
    };

    let ubid: UnsealedBid = UnsealedBid::from_vals(addr, 1, 2_000, format!("21fa2f9a1856a7bc5f8938616e358edc"));
    Ok(Wrapper::Ub(ubid))
}

pub fn gen_unsealed_bid3(_keyfile: Option<&str>) -> Result<Wrapper, CliError> {
    let addr = match addr_from_keyfile(_keyfile) {
        Ok(v) => v,
        Err(e) => return Err(CliError::CustomError { contents: format!("error creating unsealed bid due to keyfile management issues: {}\n", e)})
    };

    let ubid: UnsealedBid = UnsealedBid::from_vals(addr, 1, 3_000, format!("21fa2f9a1856a7bc5f8938616e358edc"));
    Ok(Wrapper::Ub(ubid))
}

pub fn gen_sealed_bid1(_keyfile: Option<&str>) -> Result<Wrapper, CliError> {
    let addr = match addr_from_keyfile(_keyfile) {
        Ok(v) => v,
        Err(e) => return Err(CliError::CustomError { contents: format!("error creating unsealed bid due to keyfile management issues: {}\n", e)})
    };

    let ubid: UnsealedBid = UnsealedBid::from_vals(addr, 1, 1_000, format!("21fa2f9a1856a7bc5f8938616e358edc"));
    Ok(Wrapper::Sb(ubid.to_sealed_bid()))
}

pub fn gen_sealed_bid2(_keyfile: Option<&str>) -> Result<Wrapper, CliError> {
    let addr = match addr_from_keyfile(_keyfile) {
        Ok(v) => v,
        Err(e) => return Err(CliError::CustomError { contents: format!("error creating unsealed bid due to keyfile management issues: {}\n", e)})
    };

    let ubid: UnsealedBid = UnsealedBid::from_vals(addr, 1, 2_000, format!("21fa2f9a1856a7bc5f8938616e358edc"));
    Ok(Wrapper::Sb(ubid.to_sealed_bid()))
}

pub fn gen_sealed_bid3(_keyfile: Option<&str>) -> Result<Wrapper, CliError> {
    let addr = match addr_from_keyfile(_keyfile) {
        Ok(v) => v,
        Err(e) => return Err(CliError::CustomError { contents: format!("error creating unsealed bid due to keyfile management issues: {}\n", e)})
    };

    let ubid: UnsealedBid = UnsealedBid::from_vals(addr, 1, 3_000, format!("21fa2f9a1856a7bc5f8938616e358edc"));
    Ok(Wrapper::Sb(ubid.to_sealed_bid()))
}



#[cfg(test)]
pub mod idtests {
    use super::*;

    #[test]
    fn test_alluas() {
        let a = all_useraccts();
//        println!("**uas: {:?}\n", a);
        assert_eq!(1, 1);
    }

    #[test]
    fn test_allwrappeduas() {
        let a = all_seeded_wrapped();
//        println!("**wuas: {:?}\n", a);
        assert_eq!(1, 1);
    }

    #[test]
    fn test_bids() {
       let a = gen_sealed_bid1(Some("acct1"));
       let b = gen_sealed_bid2(Some("acct2"));
       let c = gen_sealed_bid3(Some("acct3"));
       let d = gen_unsealed_bid1(Some("acct1"));
       let e = gen_unsealed_bid2(Some("acct2"));
       let f = gen_unsealed_bid3(Some("acct3"));

       println!("{:?}\n{:?}\n{:?}\n{:?}\n{:?}\n{:?}\n", a, b, c, d, e, f);
    }

    #[test]
    fn test_sealed_auction() {
        let a = gen_sealed_auction1();
        let b = gen_sealed_auction2();
        println!("auction: {:?}\n{:?}\n", a, b);
    }
}