use std::hint::black_box;

use criterion::{Criterion, criterion_group, criterion_main,BatchSize};
use orderbook::{Order, OrderBook, Side};


fn create_order(id:u64) -> Order {
    Order {
        id: black_box(id),
        price: black_box(100), //price: 100,
        qty: black_box(10), //qty: 10,
        side: Side::Buy,
        prev: None,
        next: None
    }
}


fn bench_single_insert_empty_book(c: &mut Criterion) {
    c.bench_function("bench_single_limit_order_empty_book", |b| {
        b.iter_batched(
            || {
                (
                    OrderBook::new(100_000),
                    Vec::new(),
                    create_order(1)
                )
            },
            | (mut book, mut trades, order)| {
                book.limit_order(order, &mut trades);
            },
            BatchSize::SmallInput
        );
    });
}

fn bench_insert_into_growing_orderbook_by_per_iteration(c: &mut Criterion) {
    
    c.bench_function("insert_order in growing orderbook ", |b| {
        b.iter_batched(
            || {
                let mut book = OrderBook::new(100_000);
                let mut trades = Vec::new();

                for id in 0..10000{
                    book.limit_order(create_order(id), &mut trades);
                }

                (book,trades)
            },
            |(mut book, mut trades)| {
                book.limit_order(create_order(10001), &mut trades);
            },
            BatchSize::SmallInput
        );
        
    });
}

criterion_group!(benches, bench_insert_into_growing_orderbook_by_per_iteration, bench_single_insert_empty_book);
criterion_main!(benches);
