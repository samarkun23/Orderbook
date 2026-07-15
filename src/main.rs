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

use orderbook::{Order, OrderBook, Side};
use std::time::{Duration, Instant};

fn main() {
    let mut book = OrderBook::new(100_000);
    let mut trades = Vec::with_capacity(1000);

    // Warm up
    for i in 0..1000 {
        let order = Order {
            id: i,
            price: 100 + (i % 10),
            qty: 10,
            side: Side::Sell,
            prev: None,
            next: None,
        };
        book.limit_order(order, &mut trades);
        println!("{:?}", order);
    }
    trades.clear();

    // Benchmark a single matching order
    let start = Instant::now();
    let taker = Order {
        id: 999_999,
        price: 110,
        qty: 500,
        side: Side::Buy,
        prev: None,
        next: None,
    };
    book.limit_order(taker, &mut trades);
    println!("{:?}", taker);
    let duration = start.elapsed();

    println!("Matched {} trades in {:?}", trades.len(), duration);
    println!("Latency per trade: {:?}", duration / trades.len() as u32);
}
