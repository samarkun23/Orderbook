use std::{hint::black_box};
use criterion::{Criterion,criterion_group,criterion_main,BatchSize};
use orderbook::{Order, OrderBook, Side};

fn create_order(id:u64,price:u64,qty:u64,side:Side) -> Order {
    Order {
        id: black_box(id),
        price: black_box(price), //price: 100,
        qty: black_box(qty), //qty: 10,
        side: side,
        prev: None,
        next: None
    }
}


fn bench_matching(c: &mut Criterion) {
    let mut group = c.benchmark_group("matching");

    group.bench_function("single match", |b| {
        b.iter_batched(
            || {
                let mut book = OrderBook::new(100);
                let mut trades = Vec::new();

                let maker = create_order(1,100,10, Side::Sell);
                book.limit_order(maker, &mut trades);

                (book, trades)

            },
            | (mut book, mut trades)| {
                let taker = create_order(2,100,10, Side::Buy);

                book.limit_order(taker, &mut trades);
            },
            BatchSize::SmallInput,
        );
    });

    group.bench_function("partial_match", |b| {
        b.iter_batched(
            || {
                let mut book = OrderBook::new(100);
                let mut trades = Vec::new();

                let maker = create_order(1,100,100, Side::Sell);
                book.limit_order(maker, &mut trades);

                (book,trades)
            },
            | (mut book, mut trades) | {
                let taker = create_order(2, 100, 20, Side::Buy);

                book.limit_order(taker, &mut trades);
            },
            BatchSize::SmallInput,
        );
    });

    group.bench_function("multiple_match", |b|{
        b.iter_batched(
            || {
                let mut book = OrderBook::new(100);
                let mut trades = Vec::new();

                for id in 1..=10{
                    let maker = create_order(id, 100,10, Side::Sell);
                    book.limit_order(maker, &mut trades);
                }

                (book,trades)
            },
            |(mut book,mut trades)|{
                let taker = create_order(100, 100, 100, Side::Buy);

                book.limit_order(taker, &mut trades);
            },
            BatchSize::SmallInput
        );
    });


    group.finish();

}

criterion_group!(benches, bench_matching);
criterion_main!(benches);
