use dvotp::voterid::VoterId;
use dvotp::wrapper::Wrapper;
use dvotp::ballots::{ CBallot, RBallot };
use utils::pubkey_from_keyfile;
use swth_cli_libv2::errors::CliError;
use failure::Error;

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
    
pub fn all_voterids() -> Vec<VoterId> {
    let mut keys = all_accts_vec();
    let mut names = all_names();
    let mut zipmap = names.into_iter().zip(keys).map(|x| VoterId::from_vals(x.0, x.1).unwrap());
    let collected: Vec<VoterId> = zipmap.collect();

    collected
}

pub fn all_wrapped_ids() -> Vec<Wrapper> {
    let mut keys = all_accts_vec();
    let mut names = all_names();
    let mut zipmap = names.into_iter().zip(keys).map(|x| Wrapper::Id(VoterId::from_vals(x.0, x.1).unwrap()));
    let collected: Vec<Wrapper> = zipmap.collect();

    collected

}

pub fn seed_cballots() -> Vec<CBallot> {
vec![
    CBallot::from_prefs(0),
    CBallot::from_prefs(5),
    CBallot::from_prefs(10),
    CBallot::from_prefs(15),
    CBallot::from_prefs(20),
    CBallot::from_prefs(25),
    CBallot::from_prefs(30),
    CBallot::from_prefs(35),
    CBallot::from_prefs(40),
    CBallot::from_prefs(45),
    CBallot::from_prefs(50),
    CBallot::from_prefs(55),
    CBallot::from_prefs(60),
    CBallot::from_prefs(65),
    CBallot::from_prefs(70),
    CBallot::from_prefs(75),
    CBallot::from_prefs(80),
    CBallot::from_prefs(85),
    CBallot::from_prefs(90),
    CBallot::from_prefs(95),
    ]
}

pub fn seed_rballots() -> Vec<RBallot> {
vec![
    RBallot::from_prefs(vec![2, 1, 3, 4, 5]),
    RBallot::from_prefs(vec![5, 2, 3, 4, 1]),
    RBallot::from_prefs(vec![3, 1, 2, 5, 4]),
    RBallot::from_prefs(vec![1, 2, 5, 4, 1]),
    RBallot::from_prefs(vec![1, 2, 3, 4, 5]),
    RBallot::from_prefs(vec![1, 5, 3, 4, 2]),
    RBallot::from_prefs(vec![4, 2, 3, 1, 5]),
    RBallot::from_prefs(vec![1, 2, 3, 2, 5]),
    RBallot::from_prefs(vec![1, 5, 3, 4, 2]),
    RBallot::from_prefs(vec![3, 2, 1, 4, 5]),
    RBallot::from_prefs(vec![1, 2, 5, 4, 3]),
    RBallot::from_prefs(vec![4, 2, 3, 1, 5]),
    RBallot::from_prefs(vec![1, 5, 3, 4, 2]),
    RBallot::from_prefs(vec![2, 2, 3, 4, 5]),
    RBallot::from_prefs(vec![1, 2, 5, 4, 3]),
    RBallot::from_prefs(vec![4, 3, 2, 1, 5]),
    RBallot::from_prefs(vec![5, 2, 3, 4, 1]),
    RBallot::from_prefs(vec![1, 2, 3, 4, 5]),
    RBallot::from_prefs(vec![4, 5, 3, 1, 2]),
    RBallot::from_prefs(vec![1, 2, 3, 4, 5]),
    ]

}

pub fn gen_wrapped_regs() -> Vec<Wrapper> {
    let mut ids = all_voterids();
    ids.into_iter().map(|x| Wrapper::Id(x)).collect()
}

pub fn gen_wrapped_cbals() -> Vec<Wrapper> {
    let mut ids = all_voterids();
    let mut cbs = seed_cballots();

    let mut cbs: Vec<Wrapper> = cbs.into_iter().zip(ids).map(|x| Wrapper::Cb((x.0, x.1))).collect();

    cbs
}

pub fn gen_wrapped_rbals() -> Vec<Wrapper> {
    let mut ids = all_voterids();
    let mut cbs = seed_rballots();

    let mut cbs: Vec<Wrapper> = cbs.into_iter().zip(ids).map(|x| Wrapper::Rb((x.0, x.1))).collect();

    cbs
}

pub fn gen_default_id(_n: Option<&str>, _k: Option<&str>) -> Result<VoterId, CliError> {
    let name: String = match _n {
        Some(v) => format!("{}", v),
        None => format!("Default McDefault"),
    };
    match pubkey_from_keyfile(_k) {
        Ok(v) => Ok(VoterId::from_vals(name, v).unwrap()),
        Err(e) => return Err(CliError::CustomError { contents: format!("unable to form public key from keyfile in generators!")})
    }
}


pub fn gen_wrapped_seeds() -> Vec<Wrapper> {
    let mut ids = all_voterids();
    let mut ids2 = all_voterids();
    let mut ids3 = all_voterids();
    let mut cbs = seed_cballots();
    let mut rbs = seed_rballots();

    let mut regs: Vec<Wrapper> = ids.into_iter().map(|x| Wrapper::Id(x)).collect();
    let mut cbs: Vec<Wrapper> = cbs.into_iter().zip(ids2).map(|x| Wrapper::Cb((x.0, x.1))).collect();
    let mut rbs: Vec<Wrapper> = rbs.into_iter().zip(ids3).map(|x| Wrapper::Rb((x.0, x.1))).collect();

    regs.append(&mut cbs);
    regs.append(&mut rbs);

    regs
}

#[cfg(tests)]
mod idtests {
    use super::*;

    #[test]
    fn test_allids() {
        let a = all_voterids();
        println!("**ids: {:?}\n", a);
    }
}