# 📊 Detailed Benchmark Report

All benchmarks were run using [Criterion.rs](https://github.com/bheisler/criterion.rs) in release mode (`cargo bench`). Plotting backend: `plotters` (gnuplot not available on this machine).

---

## 1. Cancel Order (`benches/cancel.rs`)

| Benchmark | Time (min–max, mean) | Change | Verdict |
|---|---|---|---|
| `first_insert_and_cancel_order` | 11.079 – 11.189 µs (mean 11.136 µs) | -27.96% to -24.41% (mean -26.22%), p = 0.00 | ✅ Improved |
| `bench_cancel_order` | 9.3528 – 9.4287 µs (mean 9.3909 µs) | -28.72% to -17.02% (mean -24.46%), p = 0.00 | ✅ Improved |

Outliers: 1/100 (high severe) for the first benchmark, 5/100 (1 high mild, 4 high severe) for the second.

📈 Graph: [`graphs/cancel/report/index.html`](./graphs/bench_cancel_order/report/index.html)

---

## 2. Insert Order (`benches/insert.rs`)

| Benchmark | Time (min–max, mean) | Change | Verdict |
|---|---|---|---|
| `insert_order in growing orderbook` | 175.13 – 178.72 µs (mean 177.08 µs) | +61046% to +96113% (mean +80155%), p = 0.00 | 🔴 **Regressed** |
| `bench_single_limit_order_empty_book` | 11.027 – 11.113 µs (mean 11.073 µs) | no baseline available | ⚪ New |

⚠️ **Regression flagged.** `insert_order in growing orderbook` jumped by roughly 3 orders of magnitude in relative terms — from a microsecond-scale baseline to ~177 µs mean. This needs to be investigated before merging further changes. Likely suspects:
- `BTreeMap` price-level insertion cost growing with book depth
- Slab reallocation/growth overhead
- A change in benchmark setup (e.g. book size) between runs

Outliers: 10/100, all low severe.

***Insert in empty order book***
📈 Graph: [`graphs/insert_in_empty_orderbook/report/index.html`](./graphs/bench_single_insert_empty_book/report/index.html)

***Insert in growing order book per iteration***
📈 Graph: [`graphs/insert_single_limit_orderbook/report/index.html`](./graphs/insert_order%20in%20growing%20orderbook%20/report/index.html)

---

## 3. Matching (`benches/match.rs`)

| Benchmark | Time (min–max, mean) | Change | p-value | Verdict |
|---|---|---|---|---|
| `matching/single_match` | 361.54 – 430.12 ns (mean 399.85 ns) | -36.06% to +6.16% (mean -17.99%) | 0.13 | ⚪ No significant change |
| `matching/partial_match` | 357.65 – 424.48 ns (mean 395.29 ns) | -39.41% to +2.00% (mean -21.06%) | 0.08 | ⚪ No significant change |
| `matching/multiple_match` | 532.87 – 660.11 ns (mean 602.19 ns) | -33.17% to -3.73% (mean -20.08%) | 0.02 | ✅ Improved |

Outliers: 25/100 for `multiple_match` (24 high mild, 1 high severe).

***Single Match***
📈 Graph: [`graphs/match/report/index.html`](graphs/matching/single_match/index.html)

***Partial Match***
📈 Graph: [`graphs/match/report/index.html`](graphs/matching/partial_match/index.html)

***Multiple Match***
📈 Graph: [`graphs/match/report/index.html`](graphs/matching/multipe_match/index.html)

---

## Summary

- ✅ Cancellation path (`cancel_order`, `insert_and_cancel`) improved **24–26%**.
- ✅ Multi-order matching improved **~20%**.
- ⚪ Single/partial match unchanged — differences fall within noise (p > 0.05).
- 🔴 **`insert_order` in a growing orderbook regressed massively (~+80,000%)** — top priority to profile and fix before the next benchmark cycle.

## All Graphs

Interactive Criterion HTML reports (open in browser):

- [`graphs/cancel/report/index.html`](graphs/bench_cancel_order/report/index.html)
- [`graphs/insert/report/index.html`](graphs/insert_order/report/index.html)
- [`graphs/match/report/index.html`](graphs/matching/partial_match/index.html)