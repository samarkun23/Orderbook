use std::hint::black_box;
use criterion::{Criterion, criterion_group, criterion_main, BatchSize};
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


fn first_insert_and_cancel_order(c: &mut Criterion){
    c.bench_function("first_insert_and_cancel_order", |b| {
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
                book.cancel_order(order.id);
            },
            BatchSize::SmallInput
        );
    });
}

fn bench_cancel_order(c: &mut Criterion){
    c.bench_function("bench_cancel_order", |b: &mut criterion::Bencher<'_>| {
        b.iter_batched(
            || {
                let mut book = OrderBook::new(100_000);
                let mut trades = Vec::new();
                let order = create_order(1);

                let id = order.id;

                book.limit_order(order, &mut trades);
                
                (book, id)
            },
            | (mut book,id)|{
                book.cancel_order(id);
            },
            BatchSize::SmallInput,
        );
    });
}

criterion_group!(benches, first_insert_and_cancel_order, bench_cancel_order);
criterion_main!(benches);

