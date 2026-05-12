# Data Structure Performance Benchmark Report: Epoch Progression

This document tracks the performance metrics (Insert and Find operations) of various data structures (`LinearMap` variants and `HashMap`) across four epochs, using two distinct datasets (`firewall_log(~40 fields)` and `firewall_log_expanded(~80 fields)`).

---

## Performance Difference: Epoch 1 (Baseline) vs Epoch 4 (Latest)

This section highlights the net performance change from the initial baseline (Epoch 1) to the latest recorded state (Epoch 4). 
*(Note: Negative values indicate a decrease in time, meaning **performance improvement**. Positive values indicate an increase in time, meaning **performance regression**).*

### Dataset: `firewall_log`
| Structure | Insert (E1 -> E4) | Net Insert Diff | Find (E1 -> E4) | Net Find Diff | Overall Trend (E1 to E4) |
| :--- | :--- | :--- | :--- | :--- | :--- |
| **LinearMap** | 4.03 µs -> 3.43 µs | **-0.60 µs** | 2.05 µs -> 1.95 µs | **-0.10 µs** | Improved |
| **LinearMapV2** | 2.55 µs -> 2.79 µs | **+0.24 µs** | 1.41 µs -> 1.46 µs | **+0.05 µs** | Regressed |
| **LinearMapV3** | 2.70 µs -> 2.41 µs | **-0.29 µs** | 2.34 µs -> 1.91 µs | **-0.43 µs** | Improved |
| **HashMap** | 6.35 µs -> 6.37 µs | **+0.02 µs** | 2.41 µs -> 2.40 µs | **-0.01 µs** | Stagnant |

### Dataset: `firewall_log_expanded`
| Structure | Insert (E1 -> E4) | Net Insert Diff | Find (E1 -> E4) | Net Find Diff | Overall Trend (E1 to E4) |
| :--- | :--- | :--- | :--- | :--- | :--- |
| **LinearMap** | 8.70 µs -> 9.11 µs | **+0.41 µs** | 5.97 µs -> 6.08 µs | **+0.11 µs** | Regressed |
| **LinearMapV2** | 5.42 µs -> 5.26 µs | **-0.16 µs** | 4.38 µs -> 4.34 µs | **-0.04 µs** | Improved |
| **LinearMapV3** | 5.05 µs -> 4.71 µs | **-0.34 µs** | 6.27 µs -> 6.15 µs | **-0.12 µs** | Improved |
| **HashMap** | 12.41 µs -> 14.28 µs | **+1.87 µs** | 4.61 µs -> 4.94 µs | **+0.33 µs** | Severely Regressed |

---

## Detailed Epoch Logs

### Epoch 1 
#### Dataset: `firewall_log`
| Structure | Insert | Find |
| :--- | :--- | :--- |
| **LinearMap** | 4.03 µs | 2.05 µs |
| **LinearMapV2** | 2.55 µs | 1.41 µs |
| **LinearMapV3** | 2.70 µs | 2.34 µs |
| **HashMap** | 6.35 µs | 2.41 µs |

#### Dataset: `firewall_log_expanded`
| Structure | Insert | Find |
| :--- | :--- | :--- |
| **LinearMap** | 8.70 µs | 5.97 µs |
| **LinearMapV2** | 5.42 µs | 4.38 µs |
| **LinearMapV3** | 5.05 µs | 6.27 µs |
| **HashMap** | 12.41 µs | 4.61 µs |

---

### Epoch 2

#### Dataset: `firewall_log`
| Structure | Insert | Find | Trend |
| :--- | :--- | :--- | :--- |
| **LinearMap** | 3.43 µs | 2.01 µs | Improved |
| **LinearMapV2** | 2.56 µs | 1.35 µs | Stable/improved |
| **LinearMapV3** | 2.38 µs | 2.12 µs | Lookup regressed |
| **HashMap** | 5.89 µs | 2.32 µs | Lookup regressed |

#### Dataset: `firewall_log_expanded`
| Structure | Insert | Find | Trend |
| :--- | :--- | :--- | :--- |
| **LinearMap** | 7.98 µs | 6.28 µs | Insert regressed |
| **LinearMapV2** | 4.98 µs | 4.01 µs | Improved |
| **LinearMapV3** | 4.67 µs | 5.96 µs | Lookup regressed |
| **HashMap** | 12.88 µs | 4.65 µs | Lookup improved |

---

### Epoch 3 

#### Dataset: `firewall_log`
| Structure | Insert | Find | Trend |
| :--- | :--- | :--- | :--- |
| **LinearMap** | 3.42 µs | 2.22 µs | Insert improved |
| **LinearMapV2** | 2.59 µs | 1.41 µs | Stable |
| **LinearMapV3** | 2.42 µs | 2.07 µs | Lookup improved |
| **HashMap** | 5.97 µs | 2.09 µs | Improved |

#### Dataset: `firewall_log_expanded`
| Structure | Insert | Find | Trend |
| :--- | :--- | :--- | :--- |
| **LinearMap** | 8.17 µs | 5.91 µs | Insert improved |
| **LinearMapV2** | 5.44 µs | 4.33 µs | Stable |
| **LinearMapV3** | 4.75 µs | 5.57 µs | Improved |
| **HashMap** | 12.07 µs | 5.03 µs | Lookup regressed |

---

### Epoch 4

#### Dataset: `firewall_log`
| Structure | Insert | Find | Trend |
| :--- | :--- | :--- | :--- |
| **LinearMap** | 3.43 µs | 1.95 µs | Stable |
| **LinearMapV2** | 2.79 µs | 1.46 µs | Regressed |
| **LinearMapV3** | 2.41 µs | 1.91 µs | Lookup improved |
| **HashMap** | 6.37 µs | 2.40 µs | Insert regressed |

#### Dataset: `firewall_log_expanded`
| Structure | Insert | Find | Trend |
| :--- | :--- | :--- | :--- |
| **LinearMap** | 9.11 µs | 6.08 µs | Stable |
| **LinearMapV2** | 5.26 µs | 4.34 µs | Regressed |
| **LinearMapV3** | 4.71 µs | 6.15 µs | Lookup regressed |
| **HashMap** | 14.28 µs | 4.94 µs | Fully regressed |

---

## Aggregate Mean Performance (`firewall_log`)

| Structure | Avg Insert | Avg Find |
|---|---:|---:|
| LinearMap | 3.58 µs | 2.06 µs |
| LinearMapV2 | 2.62 µs | 1.41 µs |
| LinearMapV3 | 2.48 µs | 2.11 µs |
| HashMap | 6.15 µs | 2.31 µs |

## Aggregate Mean Performance (`firewall_log_expanded`)

| Structure | Avg Insert | Avg Find |
|---|---:|---:|
| LinearMap | 8.49 µs | 6.06 µs |
| LinearMapV2 | 5.28 µs | 4.27 µs |
| LinearMapV3 | 4.80 µs | 5.99 µs |
| HashMap | 12.91 µs | 4.81 µs |

---

# Key Findings

- `LinearMapV2` consistently provided the fastest lookup performance across all epochs.
- `LinearMapV3` achieved the best insertion throughput but exhibited unstable lookup behavior.
- `HashMap` insertion performance degraded significantly on expanded datasets.
- Expanded schemas amplified memory-layout and cache-locality effects across all implementations.
- Arena-based contiguous layouts demonstrated strong advantages for structured telemetry workloads.

## Interactive Criterion Report

View the generated Criterion HTML benchmark report here:

- [criterion/report/index.html](./criterion/report/index.html)