use address::Address;
use auction::{Auction, Bid};
use useracct::UserAccount;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum AdminMsg {
    AdminCredit(BalanceChange),
    AdminDebit(BalanceChange),
    NewUser(UserAccount),
    EndAuction(String /* Auction address */),
    ClearOrderBook,
    Step,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct BalanceChange {
    pub addr: Address,
    pub cash: (u64, u64),
    pub assets: (u64, u64),
}

impl BalanceChange {
    pub fn from_vals(_addr: Address, _cash: (u64, u64), _assets: (u64, u64)) -> Self {
        BalanceChange {
            addr: _addr,
            cash: _cash,
            assets: _assets,
        }
    }
}
