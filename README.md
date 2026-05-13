# LinearMap Performance Benchmarks

A high-performance benchmark suite evaluating custom, contiguous `LinearMap` implementations against the standard library `HashMap` in Rust. 

These tests are designed to measure extreme low-latency execution for telemetry ingestion (specifically firewall logs), emphasizing the advantages of cache locality, deterministic memory management, and flat memory layouts over traditional hashing abstractions.

## Benchmark Reports

- Full benchmark analysis: [BENCH-RESULT.MD](./BENCH-RESULT.MD)
- Interactive Criterion HTML Report: [criterion/report/index.html](https://fct-lokesh.github.io/maps-benchmark/criterion/report/index.html)
## Overview

The benchmark tracks the evolution of a custom `LinearMap` built from scratch across repeted benchmark-runs, testing both Insert and Find operations.

*   **`HashMap`**: The Rust standard library baseline.
*   **`LinearMap` (v1)**: The initial stable contiguous layout.
*   **`LinearMapV2`**: Iteration optimized for the fastest possible lookup paths.
*   **`LinearMapV3`**: Iteration optimized for maximum insertion throughput.

Testing is executed against two distinct workload profiles:
1.  **`firewall_log`**: Standard dataset (~40 fields).
2.  **`firewall_log_expanded`**: Heavy dataset (~80 fields) used to stress-test cache-line boundaries and memory alignment.

## Key Findings

Extensive profiling via Criterion has demonstrated that standard hashing mechanisms degrade under expanded schemas due to cache misses and scattered memory allocations. 

*   **Lookups**: `LinearMapV2` consistently provides the lowest latency and fastest lookup performance across all datasets, heavily outperforming the `HashMap`.
*   **Insertions**: `LinearMapV3` achieves the best insertion throughput, mitigating the write-penalties typically associated with contiguous arrays.
*   **Degradation**: The standard `HashMap` suffers a severe insertion regression (averaging ~12.91 µs) on the expanded dataset, validating the move to custom arena-based contiguous layouts for structured telemetry workloads.

*(For a full breakdown of the Epoch progression and microsecond-level metrics, see `BENCH-RESULT.MD`).*

## Project Structure

```text
.
├── benches/
│   └── map_bench.rs        # Criterion benchmark definitions and runner
├── criterion/              # Pre-generated HTML/SVG statistical reports
├── src/
│   ├── maps/               # Data structure implementations
│   │   ├── hm.rs           # Standard HashMap wrapper
│   │   ├── lmv1.rs         # Original LinearMap
│   │   ├── lmv2.rs         # LinearMapV2
│   │   └── lmv3.rs         # LinearMapV3
│   ├── data.rs             # Log dataset generation and mock payloads
│   ├── lib.rs              
│   └── main.rs             
├── BENCH-RESULT.MD         # Aggregate analysis and performance deltas
└── Cargo.toml
```


## 🛠️ Getting Started

### 1. Install Rust
This project requires the Rust compiler and `cargo` package manager. If you don't have Rust installed, you can easily install it via `rustup`.

Open your terminal and run the following command:
```bash
curl --proto '=https' --tlsv1.2 -sSf [https://sh.rustup.rs](https://sh.rustup.rs) | sh
```

### 2. Clone the Repository
git clone [https://github.com/fct-lokesh/maps-benchmark.git]

### 3.Run the Benchmarks
```
cargo bench
```

### 4. View the Reports
```
open criterion/report/index.html
```
(Note: You can also view the historical performance graphs directly in this repository by navigating to the criterion/ folder via GitHub).
