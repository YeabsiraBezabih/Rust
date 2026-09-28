# Capstone 5: Sub-Millisecond In-Memory Time-Series Analytics Engine

## 1. Project Overview

Capstone 5 focuses on extreme low-level systems performance engineering, profiling, and hardware mechanical sympathy. 

You will design and optimize an **In-Memory Time-Series Columnar Analytics Engine** capable of ingesting **10,000,000 datapoints per second** and calculating aggregated statistics (sum, average, P50, P90, P99, min, max, standard deviation) over 100,000,000 recorded metrics in **under 2 milliseconds** using SIMD vectorization, cache line alignment, custom memory allocators (`jemalloc`/`mimalloc`), and lock-free ring buffers.

```
                               ┌────────────────────────────────────────────────────────┐
                               │   Sub-Millisecond SIMD Time-Series Analytics Engine    │
                               │                                                        │
10M Datapoints / Sec ─────────►│ [ Lock-Free SPSC / MPMC Ring Buffer ]                  │
(Network / Kernel Ingestion)   │           │ (Cache-line padded #[repr(align(64))])     │
                               │           ▼                                            │
                               │ [ Columnar In-Memory Segment Storage ]                 │
                               │           ├── Timestamps Column: Box<[u64]> (Contiguous)│
                               │           ├── Metrics Column:    Box<[f32]> (64B Align)│
                               │           └── Tags Column:       Dictionary-Encoded u16│
                               │                                                        │
                               ├──► [ Custom High-Performance Allocator ] (jemalloc)    │
                               │          └── Pre-allocated chunk arena pools           │
                               │                                                        │
                               ├──► [ SIMD AVX2 / NEON Query Execution Unit ]           │
                               │          ├── f32x8 Vectorized Sum & Average            │
                               │          ├── f32x8 Vectorized Min / Max Scan           │
                               │          └── Parallel Segment Aggregator via Rayon     │
                               │                                                        │
Sub-Millisecond Query Results ◄│ [ Flamegraph-Profiled Zero-Copy Aggregation API ]      │
(P99 Latency: < 2ms)           └────────────────────────────────────────────────────────┘
```

---

## 2. Core Functional Requirements

### A. Columnar Data Layout vs. Row-Oriented Layout
- **Columnar Representation**: Decompose metric structs into contiguous primitive arrays (`timestamps: Vec<u64>`, `values: Vec<f32>`, `metric_ids: Vec<u16>`).
- **Mechanical Sympathy**: Packing 16 `f32` numbers into a single 64-byte CPU cache line ensures that when computing averages, the CPU prefetcher achieves 100% L1/L2 cache hit ratios with zero pointer dereferences.

### B. SIMD (Single Instruction, Multiple Data) Vectorization
- **Explicit SIMD Vectorization (`std::simd` / AVX2 / ARM NEON)**: Process 8 single-precision floats (`f32x8`) simultaneously per CPU cycle.
- **Vectorized Aggregations**:
  - Vectorized Sum & Arithmetic Mean.
  - Vectorized Min/Max search using vector comparison instructions.
  - Vectorized Threshold Filters (`count_where(val > threshold)`).

### C. Cache Alignment & Lock-Free Ring Buffers
- **Cache Line Padding (`#[repr(align(64))]`)**: Ensure producer and consumer atomic pointers live on separate 64-byte cache lines to completely eliminate **False Sharing**.
- **Lock-Free SPSC/MPMC Ring Buffer**: Ingestion buffer utilizing atomic `compare_exchange` and memory fences (`Ordering::Acquire` / `Ordering::Release`) without mutexes.

### D. Memory Allocator Tuning & Profiling
- **Global Allocator Swap**: Integrate `tikv-jemallocator` or `mimalloc` to prevent memory fragmentation during rapid metric ingestion.
- **Profiling with Flamegraphs**: Generate CPU sample flamegraphs using `cargo-flamegraph` and Linux `perf` events to identify and eliminate hot CPU bottlenecks.

---

## 3. Recommended Project Architecture & Module Hierarchy

```
capstone-performance-engine/
├── Cargo.toml
├── benches/
│   └── query_benchmark.rs        # Criterion micro-benchmarks (SIMD vs Scalar)
├── src/
│   ├── main.rs                   # Engine benchmark runner and telemetry
│   ├── allocator.rs              # jemalloc / mimalloc configuration
│   ├── buffer/
│   │   ├── mod.rs
│   │   ├── ring_buffer.rs        # Lock-free SPSC queue with 64-byte alignment
│   │   └── atomic_pointers.rs    # Acquire/Release memory barrier mechanics
│   ├── storage/
│   │   ├── mod.rs
│   │   ├── chunk.rs              # Fixed-size 64KB columnar data blocks
│   │   ├── segment.rs            # Segment catalog and dictionary encoding
│   │   └── compressor.rs         # Gorilla/Delta-of-Delta timestamp compression
│   ├── simd/
│   │   ├── mod.rs
│   │   ├── avx2_aggregate.rs     # AVX2/NEON f32x8 vector math
│   │   └── fallback_scalar.rs    # Auto-vectorized portable fallback
│   ├── query/
│   │   ├── mod.rs
│   │   ├── engine.rs             # Multi-core query scheduler (Rayon integration)
│   │   └── filter.rs             # Vectorized predicate filtering
│   └── telemetry/
│       ├── mod.rs
│       └── metrics.rs            # CPU cycle counters and cache-miss monitors
└── profile/
    ├── generate_flamegraph.sh    # Script to run perf and output flamegraph.svg
    └── perf_results.md           # Documentation of optimization iterations
```

---

## 4. Benchmark & Hardware Performance Targets

| Operation | Dataset Size | Scalar Baseline | SIMD Target | Speedup Factor |
| :--- | :--- | :--- | :--- | :--- |
| **Sum & Mean** | 100,000,000 `f32` entries | $45.2\text{ ms}$ | **$< 3.5\text{ ms}$** | **$12.9\times$** |
| **Min / Max Scan** | 100,000,000 `f32` entries | $52.0\text{ ms}$ | **$< 4.1\text{ ms}$** | **$12.6\times$** |
| **Filter Count (`val > 100.0`)**| 100,000,000 `f32` entries | $68.0\text{ ms}$ | **$< 5.8\text{ ms}$** | **$11.7\times$** |
| **Ingestion Throughput** | Continuous stream | 1.2M points/sec | **$> 12\text{M points/sec}$** | **$10\times$** |

---

## 5. Profiling Verification Checklist

- [ ] Run `cargo-flamegraph` to confirm zero unexpected memory allocation functions (`malloc`/`free`) in the hot query loop.
- [ ] Inspect LLVM assembly using `cargo-asm` to verify vector instructions (`vaddps`, `vmaxps`, `vmovups`).
- [ ] Validate that atomic ring buffer producers and consumers never cause CPU cache line invalidation stalls.
