// finally new version with some opimizations
//! Key Optimizations:
//! 1. Zero-Allocation Hot Path: Uses `Slab` (pre-allocated pool) for orders.
//! 2. O(1) Operations: Price levels are doubly-linked lists of indices.
//!    - Insert: O(log P) to find price level, O(1) to append.
//!    - Match: O(1) to pop front.
//!    - Cancel: O(1) via `id_map` + linked list removal.
//! 3. Cache Locality: Indices (usize) instead of pointers/references.
//! 4. Fast Hashing: `FxHashMap` for O(1) order lookups.
//! 5. Minimal Branching: `#[inline(always)]` and streamlined logic.

use rustc_hash::FxHashMap;
use slab::Slab;
use std::collections::BTreeMap;
use std::time::Instant;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Side {
    Buy,
    Sell,
}

#[derive(Debug, Clone, Copy)]
pub struct Order {
    pub id: u64,
    pub price: u64,
    pub qty: u64,
    pub side: Side,
    // Intrusive-style doubly linked list pointers (indices into Slab)
    pub prev: Option<usize>,
    pub next: Option<usize>,
}

#[derive(Debug, Default)]
struct PriceLevel {
    head: Option<usize>,
    tail: Option<usize>,
}

pub struct OrderBook {
    bids: BTreeMap<u64, PriceLevel>, 
    asks: BTreeMap<u64, PriceLevel>, 
    orders: Slab<Order>,             
    id_map: FxHashMap<u64, usize>,   // Faster hashing for HFT
}

#[derive(Debug, Clone, Copy)]
pub struct Trade {
    pub maker_id: u64,
    pub taker_id: u64,
    pub price: u64,
    pub qty: u64,
}

impl OrderBook {
    pub fn new(capacity: usize) -> Self {
        Self {
            bids: BTreeMap::new(),
            asks: BTreeMap::new(),
            orders: Slab::with_capacity(capacity),
            id_map: FxHashMap::with_capacity_and_hasher(capacity, Default::default()),
        }
    }

    #[inline]
    pub fn limit_order(&mut self, taker_order: Order, trades: &mut Vec<Trade>) {
        match taker_order.side {
            Side::Buy => self.match_buy(taker_order, trades),
            Side::Sell => self.match_sell(taker_order, trades),
        }
    }

    fn match_buy(&mut self, mut taker: Order, trades: &mut Vec<Trade>) {
        while taker.qty > 0 {
            let (&best_ask_price, _) = match self.asks.iter().next() {
                Some(entry) => entry,
                None => break,
            };

            if best_ask_price > taker.price {
                break;
            }

            self.fill_at_price(best_ask_price, &mut taker, trades, Side::Sell);
        }

        if taker.qty > 0 {
            self.insert_into_book(taker, Side::Buy);
        }
    }

    fn match_sell(&mut self, mut taker: Order, trades: &mut Vec<Trade>) {
        while taker.qty > 0 {
            let (&best_bid_price, _) = match self.bids.iter().next_back() {
                Some(entry) => entry,
                None => break,
            };

            if best_bid_price < taker.price {
                break;
            }

            self.fill_at_price(best_bid_price, &mut taker, trades, Side::Buy);
        }

        if taker.qty > 0 {
            self.insert_into_book(taker, Side::Sell);
        }
    }

    #[inline(always)]
    fn fill_at_price(&mut self, price: u64, taker: &mut Order, trades: &mut Vec<Trade>, maker_side: Side) {
        let book_side = if maker_side == Side::Buy { &mut self.bids } else { &mut self.asks };
        
        let level = match book_side.get_mut(&price) {
            Some(l) => l,
            None => return,
        };
        
        while let Some(maker_idx) = level.head {
            if taker.qty == 0 { break; }

            let maker = self.orders.get_mut(maker_idx).expect("Slab index must exist");
            let trade_qty = std::cmp::min(taker.qty, maker.qty);

            trades.push(Trade {
                maker_id: maker.id,
                taker_id: taker.id,
                price,
                qty: trade_qty,
            });

            taker.qty -= trade_qty;
            maker.qty -= trade_qty;

            if maker.qty == 0 {
                let next_idx = maker.next;
                let maker_id = maker.id;
                
                level.head = next_idx;
                if let Some(next) = next_idx {
                    self.orders.get_mut(next).unwrap().prev = None;
                } else {
                    level.tail = None;
                }

                self.orders.remove(maker_idx);
                self.id_map.remove(&maker_id);
            } else {
                break;
            }
        }

        if level.head.is_none() {
            book_side.remove(&price);
        }
    }

    #[inline(always)]
    fn insert_into_book(&mut self, mut order: Order, side: Side) {
        let price = order.price;
        let book_side = if side == Side::Buy { &mut self.bids } else { &mut self.asks };
        
        let level = book_side.entry(price).or_insert_with(PriceLevel::default);
        
        order.prev = level.tail;
        order.next = None;
        let order_id = order.id;
        let slab_idx = self.orders.insert(order);
        
        if let Some(tail_idx) = level.tail {
            self.orders.get_mut(tail_idx).unwrap().next = Some(slab_idx);
        } else {
            level.head = Some(slab_idx);
        }
        level.tail = Some(slab_idx);
        
        self.id_map.insert(order_id, slab_idx);
    }

    #[inline]
    pub fn cancel_order(&mut self, order_id: u64) -> bool {
        let slab_idx = match self.id_map.remove(&order_id) {
            Some(idx) => idx,
            None => return false,
        };

        let order = self.orders.remove(slab_idx);
        let book_side = if order.side == Side::Buy { &mut self.bids } else { &mut self.asks };
        
        let level = match book_side.get_mut(&order.price) {
            Some(l) => l,
            None => return true, // Should not happen if id_map is consistent
        };
        
        if let Some(prev) = order.prev {
            self.orders.get_mut(prev).unwrap().next = order.next;
        } else {
            level.head = order.next;
        }

        if let Some(next) = order.next {
            self.orders.get_mut(next).unwrap().prev = order.prev;
        } else {
            level.tail = order.prev;
        }

        if level.head.is_none() {
            book_side.remove(&order.price);
        }

        true
    }
}

fn main() {
    let mut book = OrderBook::new(100_000);
    let mut trades = Vec::with_capacity(1000);

    // Warm up
    for i in 0..1000 {
        let order = Order { id: i, price: 100 + (i % 10), qty: 10, side: Side::Sell, prev: None, next: None };
        book.limit_order(order, &mut trades);
        println!("{:?}", order);
    }
    trades.clear();

    // Benchmark a single matching order
    let start = Instant::now();
    let taker = Order { id: 999_999, price: 110, qty: 500, side: Side::Buy, prev: None, next: None };
    book.limit_order(taker, &mut trades);
    println!("{:?}", taker);
    let duration = start.elapsed();

    println!("Matched {} trades in {:?}", trades.len(), duration);
    println!("Latency per trade: {:?}", duration / trades.len() as u32);
}
