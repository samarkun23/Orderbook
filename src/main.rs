use ::std::collections::{BTreeMap, VecDeque};

#[derive(Debug)]
enum Side {
    Buy,
    Sell,
}

#[derive(Debug)]
struct Order {
    id: u64,
    price: u64,
    qty: u64,
    side: Side,
}

#[derive(Debug)]
struct OrderBook {
    bids: BTreeMap<u64, VecDeque<Order>>,
    asks: BTreeMap<u64, VecDeque<Order>>,
}

#[derive(Debug)]
struct MatchingEngine {
    orderbook: OrderBook,
}

impl MatchingEngine {
    fn process_order(&mut self, mut order: Order) {
        match order.side {
            Side::Buy => {
                while order.qty > 0 {
                    let best_asks = self.orderbook.asks.iter().next(); // this gives a best price means .iter().next() gives smallest price 
                    // so this is telling if best_ask have some value extract it so in the best_ask it's like this (u64, &VecDeque<Order>) so we need u64 value means price so that it is simple verson of this
                    // if best_ask.is_some() {
                    //     let (price, queue) = best_ask.unwrap();
                    // }

                    // if best_asks.is_none() {
                    //     // if the best ask is empty then we break
                    //     break;
                    // } // we comment this bec else break that also correct .

                    if let Some((&price, _)) = best_asks {
                        let should_remove;

                        if price > order.price {
                            break;
                        }

                        {
                            // match the order

                            let order_at_place = self.orderbook.asks.get_mut(&price).unwrap(); // get tha order queue

                            // let front_order = order_at_place.front_mut().unwrap(); // picking the front order

                            // this is the right way or you can also do that
                            if let Some(front_order) = order_at_place.front_mut() {
                                // matching the order
                                let trade_qty = std::cmp::min(order.qty, front_order.qty);
                                println!("Trade executed: {} @ {}", trade_qty, price);

                                order.qty -= trade_qty;
                                front_order.qty -= trade_qty;

                                if front_order.qty == 0 {
                                    order_at_place.pop_front();
                                }
                            }

                            should_remove = order_at_place.is_empty(); // this return a false or true so there is catch bro understand it . if it's return false than it's false
                        }
                        if should_remove {
                            self.orderbook.asks.remove(&price);
                        }
                    } else {
                        break;
                    }
                }
                // the left quantity goes here
                if order.qty > 0 {
                    self.orderbook
                        .bids
                        .entry(order.price)
                        .or_insert_with(VecDeque::new)
                        .push_back(order);
                }
            }
            Side::Sell => {
                while order.qty > 0 {
                    let best_bid = self.orderbook.bids.iter().next_back(); // nextback is highest price 
                    // if best_bid.is_none(){
                    //     break;
                    // }

                    if let Some((&price, _)) = best_bid {
                        let should_remove: bool;
                        if price < order.price {
                            break;
                        }

                        {
                            let order_at_place = self.orderbook.bids.get_mut(&price).unwrap(); // get the order queue

                            // let front_order = order_at_place.front_mut().unwrap(); // picking the front order
                            if let Some(front_order) = order_at_place.front_mut() {
                                // matching the order
                                let trade_qty = std::cmp::min(order.qty, front_order.qty);
                                println!("Trade executed: {} @ {}", trade_qty, price);
                                order.qty -= trade_qty;
                                front_order.qty -= trade_qty;

                                if front_order.qty == 0 {
                                    order_at_place.pop_front();
                                }
                            }

                            should_remove = order_at_place.is_empty();
                        }

                        if should_remove {
                            self.orderbook.bids.remove(&price);
                        }
                    } else {
                        break;
                    }
                }

                if order.qty > 0 {
                    self.orderbook
                        .asks
                        .entry(order.price)
                        .or_insert_with(VecDeque::new)
                        .push_back(order);
                }
            }
        }
    }
}

fn main() {
    let mut engine = MatchingEngine {
        orderbook: OrderBook {
            bids: BTreeMap::new(),
            asks: BTreeMap::new(),
        },
    };

    // add a sell order
    engine.process_order(Order {
        id: 1,
        price: 100,
        qty: 5,
        side: Side::Sell,
    });
    engine.process_order(Order {
        id: 2,
        price: 100,
        qty: 5,
        side: Side::Sell,
    });
    engine.process_order(Order {
        id: 3,
        price: 100,
        qty: 6,
        side: Side::Buy,
    });     

    println!("Bids: {:?}", engine.orderbook.bids);
    println!("Asks: {:?}", engine.orderbook.asks);
}
