use std::collections::BinaryHeap;
use std::fmt;

use balancebook::BalanceBook;
use errors::LibError;
use failure::Error;
use order::{BuyOrder, OrderT, SellOrder};
use useracct::UserAccount;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OrderBook {
    pub buy_orders: BinaryHeap<BuyOrder>,
    pub sell_orders: BinaryHeap<SellOrder>,
    pub nonce: u64,
}

impl From<OrderBook> for String {
    fn from(_orderbook: OrderBook) -> String {
        format!("{}", _orderbook)
    }
}

impl fmt::Display for OrderBook {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        //       let chained = self.buy_orders.iter().chain(self.sell_orders.iter()).try_for_each(|x| write!(f, "\n{}\n", x))
        self.buy_orders
            .iter()
            .try_for_each(|x| write!(f, "\n{}\n", x))?;
        self.sell_orders
            .iter()
            .try_for_each(|x| write!(f, "\n{}\n", x))?;
        write!(f, "\nNonce: {}\n", self.nonce)
    }
}

impl OrderBook {
    pub fn new() -> Self {
        let buy: BinaryHeap<BuyOrder> = BinaryHeap::new();
        let sell: BinaryHeap<SellOrder> = BinaryHeap::new();
        OrderBook {
            buy_orders: buy,
            sell_orders: sell,
            nonce: 0,
        }
    }

    pub fn is_buy_empty(&self) -> bool {
        self.buy_orders.is_empty()
    }

    pub fn is_sell_empty(&self) -> bool {
        self.sell_orders.is_empty()
    }

    pub fn buy_cardinality(&self) -> usize {
        self.buy_orders.len()
    }

    pub fn sell_cardinality(&self) -> usize {
        self.sell_orders.len()
    }

    pub fn get_nonce(&self) -> u64 {
        self.nonce
    }

    pub fn inc_nonce(&mut self) {
        self.nonce += 1;
    }

    pub fn hotswap(&mut self, _incoming: OrderBook) {
        self.buy_orders = _incoming.buy_orders;
        self.sell_orders = _incoming.sell_orders;
    }

    pub fn clear_all(&mut self) {
        self.buy_orders.clear();
        self.sell_orders.clear();
    }

    pub fn insert_buy_order(
        &mut self,
        _balance_book: &mut BalanceBook,
        mut _order: BuyOrder,
        _update_nonce: bool,
    ) -> Result<(), Error> {
        let order_cash_amt: u64 = _order.get_qty() * _order.get_price();
        _balance_book.debit_cash(&_order.get_addr_ref(), order_cash_amt)?;
        _balance_book.credit_hold_cash(&_order.get_addr_ref(), order_cash_amt)?;
        if (_update_nonce == true) {
            _order.set_nonce(self.get_nonce());
            self.inc_nonce();
        };
        self.buy_orders.push(_order);
        Ok(())
    }

    pub fn insert_sell_order(
        &mut self,
        _balance_book: &mut BalanceBook,
        mut _order: SellOrder,
        _update_nonce: bool,
    ) -> Result<(), Error> {
        _balance_book.debit_assets(&_order.get_addr_ref(), _order.get_qty())?;
        _balance_book.credit_hold_assets(&_order.get_addr_ref(), _order.get_qty())?;
        if (_update_nonce == true) {
            _order.set_nonce(self.get_nonce());
            self.inc_nonce();
        };
        self.sell_orders.push(_order);
        Ok(())
    }

    // This function will only be called if 'fill_buy' has determined that
    // the buy order can be filled; IE there exists a sell order at or below the buyer's price
    pub fn fill_or_insert_buy(
        &mut self,
        _balance_book: &mut BalanceBook,
        _buy_order: BuyOrder,
    ) -> Result<Option<BuyOrder>, Error> {
        let lowest_sell = self.sell_orders.pop().expect("Somehow popped a 'none' in 'fill or insert buy' method which should have already checked for that.");
        let lowest_sell_qty = lowest_sell.get_qty();
        let buyer_addr_ref = _buy_order.get_addr_ref();
        let seller_addr_ref = lowest_sell.get_addr_ref();

        match _buy_order.get_qty() >= lowest_sell.get_qty() {
            true => {
                let deal_amt = lowest_sell.get_price() * lowest_sell.get_qty();
                let if_buy_price = _buy_order.get_price() * lowest_sell.get_qty();
                let ref_from_buyer_hold_cash = if_buy_price - deal_amt;
                _balance_book.credit_cash(seller_addr_ref, deal_amt)?;
                _balance_book.debit_cash(buyer_addr_ref, deal_amt)?;
                _balance_book.credit_assets(buyer_addr_ref, lowest_sell_qty)?;
                _balance_book.debit_hold_assets(seller_addr_ref, lowest_sell_qty)?;
                let mut decd_buy_order = _buy_order.clone();
                decd_buy_order.dec_qty_by(lowest_sell_qty);
                Ok(Some(decd_buy_order))
            }
            false => {
                let deal_amt = lowest_sell.get_price() * _buy_order.get_qty();
                _balance_book.credit_cash(seller_addr_ref, deal_amt)?;
                _balance_book.debit_cash(buyer_addr_ref, deal_amt)?;
                _balance_book.credit_assets(buyer_addr_ref, _buy_order.get_qty())?;
                _balance_book.debit_hold_assets(seller_addr_ref, _buy_order.get_qty())?;
                let mut decd_sell_order = lowest_sell.clone();
                decd_sell_order.dec_qty_by(_buy_order.get_qty());
                self.sell_orders.push(decd_sell_order);
                Ok(None)
            }
        }
    }

    pub fn fill_or_insert_sell(
        &mut self,
        _balance_book: &mut BalanceBook,
        _sell_order: SellOrder,
    ) -> Result<Option<SellOrder>, Error> {
        let highest_buy = self.buy_orders.pop().expect("Somehow popped a 'None' off the buy order bin heap; previous function call should have validated that there's something there");
        let highest_buy_qty = highest_buy.get_qty();
        let buyer_addr_ref = highest_buy.get_addr_ref();
        let seller_addr_ref = _sell_order.get_addr_ref();

        match _sell_order.get_qty() >= highest_buy.get_qty() {
            true => {
                let deal_amt = _sell_order.get_price() * highest_buy_qty;
                let if_buy_price = highest_buy.get_price() * highest_buy_qty;
                let ref_from_buyer_hold_cash = if_buy_price - deal_amt;
                _balance_book.credit_cash(seller_addr_ref, deal_amt)?;
                _balance_book.debit_hold_cash(buyer_addr_ref, if_buy_price)?;
                _balance_book.credit_cash(buyer_addr_ref, ref_from_buyer_hold_cash)?;
                _balance_book.credit_assets(buyer_addr_ref, highest_buy_qty)?;
                _balance_book.debit_assets(seller_addr_ref, highest_buy_qty)?;
                let mut decd_sell_order = _sell_order.clone();
                decd_sell_order.dec_qty_by(highest_buy_qty);
                Ok(Some(decd_sell_order))
            }
            false => {
                let deal_amt = _sell_order.get_price() * _sell_order.get_qty();
                _balance_book.credit_cash(seller_addr_ref, deal_amt)?;
                _balance_book.debit_hold_cash(buyer_addr_ref, deal_amt)?;
                _balance_book.credit_assets(buyer_addr_ref, _sell_order.get_qty())?;
                _balance_book.debit_assets(seller_addr_ref, _sell_order.get_qty())?;
                let mut decd_buy_order = highest_buy.clone();
                decd_buy_order.dec_qty_by(_sell_order.get_qty());
                self.buy_orders.push(decd_buy_order);
                Ok(None)
            }
        }
    }
}

pub fn fill_buy(
    _order_book: &mut OrderBook,
    _balance_book: &mut BalanceBook,
    mut _order: BuyOrder,
) -> Result<(), Error> {
    let buyer_liq_cash: u64 = match _balance_book.get_by_addr(&_order.get_addr()) {
        Some(v) => v.cash,
        None => {
            return Err(Error::from(LibError::CustomError {
                contents: String::from("User could not be found."),
            }))
        }
    };

    if buyer_liq_cash < (_order.get_qty() * _order.get_price()) {
        return Err(Error::from(LibError::CustomError { contents: format!("Cannot place buy order for more assets than you currently have. Tried to buy: {}, have cash: {}\n", _order.get_qty(), buyer_liq_cash)}));
    };
    // else
    let peeked_price = match _order_book.sell_orders.peek() {
        Some(v) => v.get_price(),
        _ => {
            let order_cash_amt: u64 = _order.get_qty() * _order.get_price();
            _balance_book.debit_cash(&_order.get_addr_ref(), order_cash_amt)?;
            _balance_book.credit_hold_cash(&_order.get_addr_ref(), order_cash_amt)?;
            _order.set_nonce(_order_book.get_nonce());
            _order_book.nonce += 1;
            _order_book.buy_orders.push(_order);
            return Ok(());
        }
    };

    if _order.get_price() >= peeked_price {
        match _order_book.fill_or_insert_buy(_balance_book, _order) {
            Ok(Some(rem_buy)) => {
                if rem_buy.get_qty() == 0 {
                    return Ok(());
                };
                fill_buy(_order_book, _balance_book, rem_buy)
            }
            _ => Ok(()),
        }
    } else {
        _order_book.insert_buy_order(_balance_book, _order, true)
    }
}

pub fn fill_sell(
    _order_book: &mut OrderBook,
    _balance_book: &mut BalanceBook,
    mut _order: SellOrder,
) -> Result<(), Error> {
    let seller_liq_assets: u64 = match _balance_book.get_by_addr(&_order.get_addr()) {
        Some(v) => v.assets,
        None => {
            return Err(Error::from(LibError::CustomError {
                contents: String::from("User could not be found."),
            }))
        }
    };

    if seller_liq_assets < _order.get_qty() {
        return Err(Error::from(LibError::CustomError { contents: format!("Cannot place sell order for more assets than you currently have. Have: {}, tried to sell: {}\n", seller_liq_assets, _order.get_qty())}));
    };
    // else
    /*
    let peeked_price = match _order_book.buy_orders.peek() {
        Some(v) => v.get_price(),
        _ => return _order_book.insert_sell_order(_balance_book, _order, true)
    };
    */
    let peeked_price = match _order_book.buy_orders.peek() {
        Some(v) => v.get_price(),
        _ => {
            _balance_book.debit_assets(&_order.get_addr_ref(), _order.get_qty())?;
            _balance_book.credit_hold_assets(&_order.get_addr_ref(), _order.get_qty())?;
            _order.set_nonce(_order_book.get_nonce());
            _order_book.nonce += 1;
            _order_book.sell_orders.push(_order);
            return Ok(());
        }
    };

    if _order.get_price() <= peeked_price {
        match _order_book.fill_or_insert_sell(_balance_book, _order) {
            Ok(Some(rem_sell)) => {
                if rem_sell.get_qty() == 0 {
                    return Ok(());
                };
                fill_sell(_order_book, _balance_book, rem_sell)
            }
            _ => Ok(()),
        }
    } else {
        _order_book.insert_sell_order(_balance_book, _order, true)
    }
}
