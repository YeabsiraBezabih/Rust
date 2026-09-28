# Zero-to-Hero Systems Programming Course in Rust

A comprehensive, self-paced, zero-prerequisite curriculum designed to take a complete beginner from core computing principles to building high-performance, production-grade systems in Rust (custom HTTP servers, distributed systems, high-throughput database engines, Redis & S3 integrations, and WebAssembly).

---

## Pedagogical Philosophy

Every lesson in this course adheres strictly to:
1. **Zero-Prerequisite Progression**: Every concept is explained from first principles before being used.
2. **Mechanical Sympathy**: Constant visualization of the hardware execution model (stack, heap, CPU cache lines, network buffers, syscalls).
3. **Pure Rust Systems Focus**: Idiomatic, modern Rust without conversational filler or unexplained jargon.
4. **Standard 9-Section Structure**: Every lesson file contains Prerequisites, Concept, Why it matters, Analogy, Code Example, Common Mistakes, Exercises, Solutions, and Checkpoint.

---

## Course Roadmap

```
Phase 0: Core Programming Concepts (Foundations)
   │
Phase 1: Rust Fundamentals (Ownership, Types, Traits, Generics)
   │
Phase 2: Data Structures & Algorithms from Scratch (Memory & Mechanical Sympathy)
   │
   ├──► NeetCode 150 Practice Suite (18 Categories, 150 LeetCode Problems in Rust)
   │
Phase 3: Intermediate Rust & Systems Foundations (Lifetimes, Concurrency, Async, Unsafe)
   │
Phase 4: Networking & HTTP from Scratch (TCP Sockets, Zero-Copy Parsing, Async Tokio)
   │
   ├──► 🌟 Capstone 1: High-Performance Custom HTTP/1.1 Server & Reverse Proxy
   │
Phase 5: Modern Rust Web Frameworks: Axum, Actix-web, and Loco.rs
   │
   ├──► 🌟 Capstone 2: Multi-Tenant Enterprise REST & WebSocket API Engine
   │
Phase 6: Database Engineering & SQL in Rust: Zero to Hero (SQLx, Diesel, SeaORM, Migrations)
   │
   ├──► 🌟 Capstone 3: High-Throughput Financial Ledger & SQL Query Engine
   │
Phase 7: High-Performance Caching & Object Storage (Redis / Dragonfly & S3 / MinIO)
   │
   ├──► 🌟 Capstone 4: Distributed Real-Time Media Storage & Queue Pipeline
   │
Phase 8: Systems Performance Optimization & Low-Level Profiling (Flamegraphs, SIMD, Allocators)
   │
   ├──► 🌟 Capstone 5: Sub-Millisecond In-Memory Time-Series Analytics Engine
   │
Phase 9: Distributed Systems in Rust (Raft Consensus, RPC, Gossip, Vector Clocks)
   │
   ├──► 🌟 Capstone 6: Fault-Tolerant Distributed Key-Value Store with Raft Consensus
   │
Phase 10: WebAssembly (WASM) & Edge Computing (Virtual ISA, Interop, DOM/Canvas)
   │
   ├──► 🌟 Capstone 7: Interactive High-Performance WASM Graphics & Edge Computing Engine
   │
Phase 11: Clean Code, Design Patterns, & Software Craftsmanship in Rust (DDD, Hexagonal Architecture)
   │
   ├──► 🌟 Capstone 8: Enterprise Domain-Driven Design (DDD) & Clean Architecture Core Engine
   │
Phase 12: Linux Systems, Server Operations, & OS Internals (FHS, Systemd, SSH, Docker, Syscalls, VFS, epoll, IPC, Namespaces, cgroups, eBPF)
   │
   ├──► 🌟 Capstone 9: Lightweight Linux Container Engine & eBPF Security Monitor
   │
Phase 13: Zed Editor Mastery & High-Velocity Developer Workflows (GPUI, LSP, Vim, Tasks, CRDT, AI)
```

---

## Detailed Curriculum

### Phase 0: Core Programming Concepts
- [x] **Lesson 0.1**: Compiling vs. Running: How Hardware Executes Code (`0.1-compiling-vs-running.md`)
- [x] **Lesson 0.2**: Memory, Variables, and Data Representation (`0.2-variables-and-data-types.md`)
- [x] **Lesson 0.3**: Control Flow: How CPUs Make Decisions (`0.3-control-flow.md`)
- [x] **Lesson 0.4**: Functions, Call Stacks, and Stack Frames (`0.4-functions.md`)
- [x] **Lesson 0.5**: Memory Layout: Stack vs. Heap (`0.5-stack-vs-heap.md`)
- [x] **Lesson 0.6**: Algorithmic Efficiency: Intuition Behind Big-O Notation (`0.6-big-o-intuition.md`)

---

### Phase 1: Rust Fundamentals
- [x] **Lesson 1.1**: Rust Tooling, Cargo, and the Compilation Pipeline (`1.1-rust-tooling-and-cargo.md`)
- [x] **Lesson 1.2**: Variables, Mutability, and Shadowing (`1.2-variables-mutability-shadowing.md`)
- [x] **Lesson 1.3**: Scalar and Compound Types (`1.3-scalar-and-compound-types.md`)
- [x] **Lesson 1.4**: Ownership: Rust’s Core Memory Invariant (`1.4-ownership.md`)
- [x] **Lesson 1.5**: Borrowing and References (`1.5-borrowing-and-references.md`)
- [x] **Lesson 1.6**: Structs, Methods, and Memory Layout (`1.6-structs-and-methods.md`)
- [x] **Lesson 1.7**: Enums and Algebraic Data Types (`1.7-enums-and-algebraic-data-types.md`)
- [x] **Lesson 1.8**: Pattern Matching and Destructuring (`1.8-pattern-matching.md`)
- [x] **Lesson 1.9**: Handling Absence and Failure: `Option<T>` and `Result<T, E>` (`1.9-option-and-result.md`)
- [x] **Lesson 1.10**: Robust Error Propagation (`?` Operator and Custom Errors) (`1.10-error-propagation-and-custom-errors.md`)
- [x] **Lesson 1.11**: Packages, Crates, and Modules (`1.11-packages-crates-modules.md`)
- [x] **Lesson 1.12**: Common Collections: `Vec<T>`, `String`, and `HashMap<K, V>` (`1.12-common-collections.md`)
- [x] **Lesson 1.13**: Traits: Defining Shared Behavior (`1.13-traits-shared-behavior.md`)
- [x] **Lesson 1.14**: Generics: Writing Type-Agnostic, Zero-Cost Abstractions (`1.14-generics-zero-cost.md`)
- [x] **Lesson 1.15**: Automated Testing: Unit Tests, Integration Tests, and TDD (`1.15-automated-testing.md`)

---

### Phase 2: Data Structures & Algorithms from Scratch
- [x] **Lesson 2.1**: Contiguous Memory: Dynamic Arrays (`CustomVec`) and Amortized Analysis (`2.1-contiguous-memory-dynamic-arrays.md`)
- [x] **Lesson 2.2**: Node-Based Structures: Singly Linked Lists in Safe Rust (`2.2-linked-lists-in-rust.md`)
- [x] **Lesson 2.3**: Stacks and Queues (LIFO & FIFO Mechanics) (`2.3-stacks-and-queues.md`)
- [x] **Lesson 2.4**: Recursion & Divide-and-Conquer (`2.4-recursion-and-divide-and-conquer.md`)
- [x] **Lesson 2.5**: Searching Algorithms: Linear Search vs. Binary Search (`2.5-linear-and-binary-search.md`)
- [x] **Lesson 2.6**: Sorting from Scratch: Insertion Sort, Merge Sort, and Quicksort (`2.6-sorting-algorithms.md`)
- [x] **Lesson 2.7**: Trees: Binary Search Trees (BST) and Tree Traversals (`2.7-binary-search-trees.md`)
- [x] **Lesson 2.8**: Priority Queues & Binary Heaps (`2.8-heaps-and-priority-queues.md`)
- [x] **Lesson 2.9**: Hash Maps from Scratch: Hashing Functions, Collisions, and Buckets (`2.9-hashmaps-from-scratch.md`)
- [x] **Lesson 2.10**: Graph Representations: Adjacency Matrices vs. Adjacency Lists (`2.10-graph-representations.md`)
- [x] **Lesson 2.11**: Graph Traversals: Breadth-First Search (BFS) and Depth-First Search (DFS) (`2.11-graph-traversals-bfs-dfs.md`)
- [x] **Lesson 2.12**: Shortest Paths: Dijkstra’s Algorithm (`2.12-dijkstras-algorithm.md`)
- [x] **Lesson 2.13**: Dynamic Programming: Memoization and Tabulation Strategies (`2.13-dynamic-programming.md`)

---

### NeetCode 150 Algorithm Practice Suite (`neetcode-150/`)
*150 categorized technical interview problems with problem statements, constraints, and starter Rust signatures.*
- **01. Arrays & Hashing** (9 problems: Contains Duplicate, Valid Anagram, Two Sum, Group Anagrams, Top K Frequent, etc.)
- **02. Two Pointers** (5 problems: Valid Palindrome, Two Sum II, 3Sum, Container With Most Water, Trapping Rain Water)
- **03. Sliding Window** (6 problems: Buy & Sell Stock, Longest Substring Without Repeating, Min Window Substring, etc.)
- **04. Stack** (7 problems: Valid Parentheses, Min Stack, Evaluate RPN, Daily Temperatures, Largest Rectangle in Histogram)
- **05. Binary Search** (7 problems: Binary Search, Search 2D Matrix, Koko Bananas, Find Min in Rotated Array, Median of Two Sorted Arrays)
- **06. Linked List** (11 problems: Reverse List, Merge Two Lists, Reorder List, LRU Cache, Merge K Sorted Lists, etc.)
- **07. Trees** (15 problems: Invert Tree, Max Depth, Same Tree, Level Order Traversal, Validate BST, Serialize/Deserialize BST)
- **08. Tries** (3 problems: Implement Trie, Add/Search Words, Word Search II)
- **09. Heap / Priority Queue** (7 problems: Kth Largest Stream, K Closest Points, Task Scheduler, Find Median from Data Stream)
- **10. Backtracking** (9 problems: Subsets, Combination Sum, Permutations, Word Search, N-Queens)
- **11. Graphs** (13 problems: Number of Islands, Clone Graph, Rotting Oranges, Course Schedule I/II, Word Ladder)
- **12. Advanced Graphs** (6 problems: Min Cost Connect Points, Network Delay Time, Alien Dictionary, Cheapest Flights)
- **13. 1-D Dynamic Programming** (12 problems: Climbing Stairs, House Robber I/II, Coin Change, LIS, Word Break)
- **14. 2-D Dynamic Programming** (11 problems: Unique Paths, LCS, Target Sum, Edit Distance, Burst Balloons, Regex Matching)
- **15. Greedy** (8 problems: Maximum Subarray, Jump Game I/II, Gas Station, Partition Labels)
- **16. Intervals** (6 problems: Insert Interval, Merge Intervals, Non-Overlapping Intervals, Meeting Rooms I/II)
- **17. Math & Geometry** (8 problems: Rotate Image, Spiral Matrix, Set Matrix Zeroes, Happy Number, Pow(x,n))
- **18. Bit Manipulation** (7 problems: Single Number, Number of 1 Bits, Counting Bits, Reverse Integer)

---

### Phase 3: Intermediate Rust & Systems Foundations
- [x] **Lesson 3.1**: Deep Dive into Lifetimes and Lifetime Annotations (`'a`) (`3.1-lifetimes-and-annotations.md`)
- [x] **Lesson 3.2**: Static vs. Dynamic Dispatch: Generics (`impl Trait`) vs. Trait Objects (`dyn Trait`) (`3.2-static-vs-dynamic-dispatch.md`)
- [x] **Lesson 3.3**: Functional Rust: Iterators and Closures (`Fn`, `FnMut`, `FnOnce`) (`3.3-iterators-and-closures.md`)
- [x] **Lesson 3.4**: Smart Pointers: Unique Heap Allocation with `Box<T>` (`3.4-smart-pointers-box.md`)
- [x] **Lesson 3.5**: Reference Counting: Shared Ownership with `Rc<T>` and `Arc<T>` (`3.5-reference-counting-rc-arc.md`)
- [x] **Lesson 3.6**: Interior Mutability: `Cell<T>`, `RefCell<T>`, and Runtime Borrow Checks (`3.6-interior-mutability.md`)
- [x] **Lesson 3.7**: Thread-Safe Concurrency: `Mutex<T>`, `RwLock<T>`, and `Send`/`Sync` (`3.7-thread-safe-concurrency-mutex-rwlock.md`)
- [x] **Lesson 3.8**: Message Passing: Multi-Producer, Single-Consumer (`mpsc`) Channels (`3.8-message-passing-mpsc.md`)
- [x] **Lesson 3.9**: Asynchronous Programming Foundations: Futures, Tasks, and Tokio (`3.9-async-futures-tokio.md`)
- [x] **Lesson 3.10**: Advanced Error Handling Patterns in Production Systems (`3.10-advanced-error-handling.md`)
- [x] **Lesson 3.11**: Unsafe Rust Conceptually: Raw Pointers, FFI, and Safety Invariants (`3.11-unsafe-rust-conceptually.md`)

---

### Phase 4: Networking & HTTP from Scratch
- [x] **Lesson 4.1**: Networking Foundations: TCP/IP Stack, Sockets, and Wire Formats (`4.1-networking-foundations-tcp-sockets.md`)
- [x] **Lesson 4.2**: Raw TCP in Rust: `TcpListener` and `TcpStream` (`4.2-raw-tcp-listener-stream.md`)
- [x] **Lesson 4.3**: HTTP/1.1 Specification: Requests, Headers, Status Codes, and CRLF Delimiters (`4.3-http-specification-crlf.md`)
- [x] **Lesson 4.4**: Parsing HTTP Requests by Hand from Raw Byte Buffers (`4.4-parsing-http-requests-raw-bytes.md`)
- [x] **Lesson 4.5**: Building a Type-Safe HTTP `Response` Serializer (`4.5-type-safe-http-response-serializer.md`)
- [x] **Lesson 4.6**: Handling Multiple Concurrent Connections: Thread Pools vs. Async I/O (`4.6-concurrency-thread-pools-async.md`)
- [x] **Lesson 4.7**: Designing a Routing Engine and Path Matcher (`4.7-routing-engine-path-matcher.md`)
- [x] **Lesson 4.8**: Middleware Architecture: Logging, Timing, and Request Mutation (`4.8-middleware-architecture.md`)
- [x] **Lesson 4.9**: Building an End-to-End Async HTTP Server with Tokio (`4.9-async-http-server-tokio.md`)

---

### 🌟 Capstone 1: High-Performance Custom HTTP/1.1 Server & Reverse Proxy
*Location: `capstones/capstone-1-custom-http-server/`*
- Zero-dependency HTTP/1.1 engine with keep-alive, streaming chunked transfer, reverse proxy load-balancing, and rate-limiting.

---

### Phase 5: Modern Rust Web Frameworks: Axum, Actix-web, and Loco.rs
- [x] **Lesson 5.1**: The Modern Rust Web Ecosystem: Axum vs. Actix-web vs. Loco.rs (`5.1-modern-web-ecosystem-axum-actix-loco.md`)
- [x] **Lesson 5.2**: Axum Architecture: State Extraction, Tower Services, and Type-Safe Handlers (`5.2-axum-state-extractors-tower.md`)
- [x] **Lesson 5.3**: Actix-web: Actor Model Foundations, App Factory, and High-Throughput Request Pipelines (`5.3-actix-web-architecture.md`)
- [x] **Lesson 5.4**: Loco.rs: The Rails of Rust (Convention over Configuration, Scaffolding, CLI) (`5.4-loco-rs-rails-of-rust.md`)
- [x] **Lesson 5.5**: Real-Time WebSockets & Server-Sent Events (SSE) in Modern Rust (`5.5-websockets-and-sse.md`)
- [x] **Lesson 5.6**: Declarative OpenAPI/Swagger Generation with `utoipa` (`5.6-openapi-swagger-utoipa.md`)

---

### 🌟 Capstone 2: Multi-Tenant Enterprise REST & WebSocket API Engine
*Location: `capstones/capstone-2-web-frameworks-axum-loco/`*
- Production REST & WebSocket microservice built with Axum & Loco featuring JWT auth, role-based access control (RBAC), auto-generated OpenAPI documentation, and real-time event broadcasting.

---

### Phase 6: Database Engineering & SQL in Rust: Zero to Hero
- [x] **Lesson 6.1**: SQL Foundations for Systems Engineers: Relational Models, Indexes, and Query Plans (`6.1-sql-foundations-for-systems-engineers.md`)
- [x] **Lesson 6.2**: Low-Level Database Wire Protocols & Raw TCP Communication with PostgreSQL (`6.2-low-level-postgres-wire-protocol.md`)
- [x] **Lesson 6.3**: Compile-Time Checked SQL with `SQLx` (`query!`, `query_as!`, and Offline Verification) (`6.3-compile-time-checked-sql-sqlx.md`)
- [x] **Lesson 6.4**: Connection Pooling Mechanics: Hikari-Style Pools, Max Connections, and Timeouts (`6.4-connection-pooling-mechanics.md`)
- [x] **Lesson 6.5**: Type-Safe ORMs: `Diesel` (Compile-Time DSL) vs. `SeaORM` (Dynamic Async ORM) (`6.5-type-safe-orms-diesel-seaorm.md`)
- [x] **Lesson 6.6**: Database Migrations as Code (Embedded Migrations & Version Control) (`6.6-database-migrations-as-code.md`)
- [x] **Lesson 6.7**: ACID Transactions, Isolation Levels (Read Committed, Repeatable Read, Serializable), and Row Locking (`SELECT FOR UPDATE`) (`6.7-acid-transactions-and-row-locking.md`)
- [x] **Lesson 6.8**: Performance Tuning: EXPLAIN ANALYZE, B-Tree vs. GIN Indexes, Connection Multiplexing (`6.8-performance-tuning-indexing-query-plans.md`)

---

### 🌟 Capstone 3: High-Throughput Financial Ledger & SQL Query Engine
*Location: `capstones/capstone-3-database-sql-engine/`*
- Double-entry bookkeeping ledger with strict ACID compliance, serializable transaction isolation, automated migrations, audit trails, and sub-millisecond account balance queries under heavy concurrency.

---

### Phase 7: High-Performance Caching & Object Storage (Redis & S3 / MinIO)
- [x] **Lesson 7.1**: In-Memory Data Stores: Redis & Dragonfly Architecture and Wire Protocol (RESP) (`7.1-in-memory-datastores-redis-resp.md`)
- [x] **Lesson 7.2**: Building a High-Throughput Redis Client in Rust (`redis-rs`, Multiplexing, Pub/Sub) (`7.2-building-high-throughput-redis-client.md`)
- [x] **Lesson 7.3**: Distributed Caching Patterns: Cache-Aside, Write-Through, Cache Stampede Mitigation, and Probabilistic Early Expiration (XFetch) (`7.3-distributed-caching-patterns-stampede.md`)
- [x] **Lesson 7.4**: Object Storage Foundations: S3 API Specification, Buckets, Objects, and MinIO Local Setup (`7.4-object-storage-foundations-s3-minio.md`)
- [x] **Lesson 7.5**: The `aws-sdk-s3` Crate: Streaming Uploads, Multipart Chunking, and Presigned URLs (`7.5-aws-sdk-s3-streaming-multipart.md`)
- [x] **Lesson 7.6**: Resilient Storage Pipelines: Retries with Exponential Backoff, Checksums, and ETag Validation (`7.6-resilient-storage-pipelines-retries-etags.md`)

---

### 🌟 Capstone 4: Distributed Real-Time Media Storage & Queue Pipeline
*Location: `capstones/capstone-4-caching-object-storage-redis-s3/`*
- Media ingestion and transcoding service using Redis Streams for distributed job queuing, MinIO for high-throughput S3 multipart uploads, and presigned download tokens with real-time progress events.

---

### Phase 8: Systems Performance Optimization & Low-Level Profiling
- [x] **Lesson 8.1**: Profiling Systems: CPU Sampling, Flamegraphs (`cargo-flamegraph`), and Perf Events (`8.1-profiling-flamegraphs-perf-events.md`)
- [x] **Lesson 8.2**: CPU Cache Alignment, False Sharing, and Struct Padding (`#[repr(align)]`) (`8.2-cpu-cache-alignment-false-sharing.md`)
- [x] **Lesson 8.3**: High-Performance Memory Allocators: System Allocator vs. `jemalloc` vs. `mimalloc` (`8.3-memory-allocators-jemalloc-mimalloc.md`)
- [x] **Lesson 8.4**: SIMD (Single Instruction, Multiple Data) Vectorization in Rust (`8.4-simd-vectorization-in-rust.md`)
- [x] **Lesson 8.5**: Zero-Copy Deserialization with `rkyv` and Buffer Pooling (`bytes::BytesMut`) (`8.5-zero-copy-deserialization-rkyv.md`)
- [x] **Lesson 8.6**: Lock Contention Elimination: Read-Copy-Update (RCU), Crossbeam Epochs, and Lock-Free Rings (`8.6-lock-contention-rcu-lock-free.md`)

---

### 🌟 Capstone 5: Sub-Millisecond In-Memory Time-Series Analytics Engine
*Location: `capstones/capstone-5-performance-profiling-engine/`*
- Columnar in-memory metrics engine utilizing SIMD aggregations, jemalloc memory pools, lock-free ring buffers, and zero-copy binary ingestion capable of processing 10,000,000 datapoints/second.

---

### Phase 9: Distributed Systems in Rust
- [x] **Lesson 9.1**: Distributed Systems Fundamentals: CAP Theorem, Network Partitions, and Clock Skew (`9.1-distributed-systems-cap-clock-skew.md`)
- [x] **Lesson 9.2**: High-Performance Remote Procedure Calls (gRPC) with `tonic` and Protocol Buffers (`9.2-grpc-tonic-protocol-buffers.md`)
- [x] **Lesson 9.3**: Cluster Membership and Gossip Protocols (SWIM Protocol from Scratch) (`9.3-cluster-membership-swim-gossip.md`)
- [x] **Lesson 9.4**: Consensus Mechanics: The Raft Consensus Algorithm (Leader Election, Log Replication, Safety) (`9.4-raft-consensus-leader-election-replication.md`)
- [x] **Lesson 9.5**: Distributed State Machines, Snapshots, and Log Compaction (`9.5-distributed-state-machines-snapshots-compaction.md`)
- [x] **Lesson 9.6**: Causality, Vector Clocks, and Conflict Resolution (CRDTs) (`9.6-causality-vector-clocks-crdts.md`)

---

### 🌟 Capstone 6: Fault-Tolerant Distributed Key-Value Store with Raft Consensus
*Location: `capstones/capstone-6-distributed-raft-store/`*
- Clustered distributed database implementing the full Raft consensus algorithm, leader election, replicated write-ahead logging (WAL), dynamic cluster reconfiguration, and gRPC client interface with linearizable reads/writes.

---

### Phase 10: WebAssembly (WASM) & Edge Computing
- [x] **Lesson 10.1**: WebAssembly Architecture: Virtual ISA, Linear Memory, & Sandboxing (`10.1-wasm-architecture-linear-memory.md`)
- [x] **Lesson 10.2**: Toolchain & Bindings: `wasm-pack`, `wasm-bindgen`, & DOM Integration (`10.2-wasm-bindgen-dom-integration.md`)
- [x] **Lesson 10.3**: WASI (WebAssembly System Interface) & Standalone Runtimes (Wasmtime) (`10.3-wasi-sandboxed-runtimes-wasmtime.md`)
- [x] **Lesson 10.4**: Serverless Edge Computing: Rust WASM on Cloudflare Workers & Fastly (`10.4-edge-computing-cloudflare-workers-wasm.md`)
- [x] **Lesson 10.5**: High-Performance In-Browser Systems: Image Processing & Cryptography (`10.5-in-browser-wasm-crypto-processing.md`)

---

### 🌟 Capstone 7: Interactive High-Performance WASM Graphics & Edge Computing Engine
*Location: `capstones/capstone-7-webassembly-edge-engine/`*
- Browser-based real-time fluid simulation and cryptographic hashing engine compiled to WebAssembly with zero-copy HTML5 Canvas shared-memory rendering and edge deployment configuration.

---

### Phase 11: Clean Code, Design Patterns, & Software Craftsmanship in Rust
- [x] **Lesson 11.1**: The Rust Philosophy of Clean Code: Type-Driven Design & "Parse, Don't Validate" (`11.1-idiomatic-rust-type-driven-design.md`)
- [x] **Lesson 11.2**: Applying SOLID Principles in Rust (Without OOP Bloat) (`11.2-solid-principles-in-rust.md`)
- [x] **Lesson 11.3**: Idiomatic Rust Design Patterns: Type-State, Builders, & RAII Guards (`11.3-essential-rust-design-patterns-typestate.md`)
- [x] **Lesson 11.4**: Public API Ergonomics & Professional Error Architecture (`11.4-api-ergonomics-and-error-architecture.md`)
- [x] **Lesson 11.5**: Refactoring Rust Code Smells & Common Anti-Patterns (`11.5-refactoring-code-smells-and-anti-patterns.md`)
- [x] **Lesson 11.6**: Clean Architecture, Hexagonal Architecture (Ports & Adapters), & DDD in Rust (`11.6-clean-hexagonal-architecture-ddd.md`)

---

### 🌟 Capstone 8: Enterprise Domain-Driven Design (DDD) & Clean Architecture Core Engine
*Location: `capstones/capstone-8-clean-architecture-ddd/`*
- Modular, hexagonal architecture order fulfillment and multi-currency banking settlement engine featuring pure domain aggregates, type-state state machines, input/output ports, and property-based verification suites.

---

### Phase 12: Linux Systems, Server Operations, & OS Internals (Zero to Hero)

#### Part 1: Linux Foundations & Server Administration (Zero to Intermediate)
- [x] **Lesson 12.1**: Linux Architecture & Kernel Interface: Syscalls, Rings, VDSO, & Assembly (`12.1-linux-architecture-syscalls-vdso-asm.md`)
- [x] **Lesson 12.2**: Linux Filesystem Hierarchy (FHS), Permissions, Ownership, & Storage Mounts (`12.2-linux-fhs-permissions-storage-mounts.md`)
- [x] **Lesson 12.3**: Systemd, Service Daemons, Journald Logging, & Log Rotation (`12.3-systemd-services-daemons-journald.md`)
- [x] **Lesson 12.4**: SSH Mastery, Server Hardening, Key Management, & Tunneling (`12.4-ssh-mastery-server-hardening-tunneling.md`)
- [x] **Lesson 12.5**: Linux Server Networking, DNS, Firewalls (`nftables`/`ufw`), & Port Inspection (`12.5-linux-server-networking-dns-firewalls.md`)
- [x] **Lesson 12.6**: Docker for Backend Engineers: Multi-Stage Rust Builds, Compose, & Networking (`12.6-docker-containers-rust-backend-engineering.md`)

#### Part 2: OS Internals & Systems Programming in Rust (Intermediate to Advanced)
- [x] **Lesson 12.7**: The Linux Process Model: `task_struct`, `fork`, `execve`, ELF Layout, & Zombies (`12.7-linux-process-model-fork-exec-zombies.md`)
- [x] **Lesson 12.8**: Linux Signals & Asynchronous Notification: `sigaction`, `signalfd`, & Real-Time Signals (`12.8-linux-signals-graceful-shutdown-realtime.md`)
- [x] **Lesson 12.9**: Virtual Memory & Memory Mapping: Page Tables, `mmap`, `mprotect`, `madvise`, & HugePages (`12.9-virtual-memory-mmap-hugepages-mprotect.md`)
- [x] **Lesson 12.10**: The Virtual Filesystem (VFS): Inodes, File Descriptors, `openat`, `getdents64`, & Page Cache (`12.10-vfs-inodes-file-descriptors-page-cache.md`)
- [x] **Lesson 12.11**: High-Scale I/O Multiplexing: Non-blocking I/O, `epoll` (ET vs LT), & `io_uring` (`12.11-file-descriptors-epoll-io-uring.md`)
- [x] **Lesson 12.12**: Inter-Process Communication (IPC): Pipes, Unix Sockets (`SCM_RIGHTS`), Shared Memory, & Futexes (`12.12-ipc-unix-sockets-shared-memory-futex.md`)
- [x] **Lesson 12.13**: Linux Networking Subsystem: Raw Sockets (`AF_PACKET`), Netlink (`rtnetlink`), & Zero-Copy Sockets (`12.13-linux-networking-raw-sockets-netlink-zerocopy.md`)

#### Part 3: Sandboxing, Security & Low-Level Kernel Observability (Hero Tier)
- [x] **Lesson 12.14**: Linux Namespaces & Process Isolation: PID, Mount, Net, UTS, IPC, User, & `pivot_root` (`12.14-linux-namespaces-containers-from-scratch.md`)
- [x] **Lesson 12.15**: Control Groups v2 (cgroups) & Resource Sandboxing: CPU CFS, Memory Hard Caps, & PSI (`12.15-cgroups-v2-resource-sandboxing-psi.md`)
- [x] **Lesson 12.16**: Linux Security Architecture: POSIX Capabilities, `prctl`, `seccomp-bpf`, & Landlock (`12.16-linux-security-capabilities-seccomp-landlock.md`)
- [x] **Lesson 12.17**: Kernel Introspection & Building a Debugger: `/proc` Parsing, `ptrace`, & Breakpoints (`12.17-kernel-tracing-procfs-ptrace-debugger.md`)
- [x] **Lesson 12.18**: Linux Kernel Tracing, eBPF & Pure Rust Observability with Aya (`12.18-kernel-tracing-ebpf-aya-rust.md`)

---

### 🌟 Capstone 9: Lightweight Linux Container Engine & eBPF Security Monitor
*Location: `capstones/capstone-9-linux-container-ebpf/`*
- Production OCI-style container runtime (`contain-rs`) and kernel security daemon (`ebpf-guard`) utilizing 7 Linux namespaces, cgroups v2 resource quotas, rootfs pivot isolation, and pure Rust Aya eBPF probes for real-time syscall auditing and driver-level XDP packet filtering.

---

### Phase 13: Zed Editor Mastery & High-Velocity Developer Workflows
- [x] **Lesson 13.1**: The Zed Editor Architecture: GPUI, Tree-Sitter, & Multi-Buffer Workflows (`13.1-zed-architecture-gpui-multi-buffer.md`)
- [x] **Lesson 13.2**: Turbocharging `rust-analyzer`, Clippy, & Inlay Hints in Zed (`13.2-rust-analyzer-clippy-inlay-hints-zed.md`)
- [x] **Lesson 13.3**: Modal Editing, Vim Mastery, & Multi-Cursor Velocity in Zed (`13.3-keyboard-navigation-multi-cursor-modal-vim.md`)
- [x] **Lesson 13.4**: Automated Workflows, Zed Tasks, & Native LLDB Debugging (`13.4-custom-tasks-cargo-workspaces-debugging.md`)
- [x] **Lesson 13.5**: Remote SSH Development, CRDT Real-Time Collaboration, & AI Workflows in Zed (`13.5-remote-development-crdt-collaboration-ai.md`)

---

## Repository Directory Structure

```
.
├── README.md
├── phase-0-programming-concepts/
├── phase-1-rust-fundamentals/
├── phase-2-data-structures-and-algorithms/
├── neetcode-150/
│   ├── 01-arrays-and-hashing.md
│   ├── 02-two-pointers.md
│   ├── ...
│   └── 18-bit-manipulation.md
├── phase-3-intermediate-rust/
├── phase-4-networking-and-http/
├── capstones/
│   ├── capstone-1-custom-http-server/
│   ├── capstone-2-web-frameworks-axum-loco/
│   ├── capstone-3-database-sql-engine/
│   ├── capstone-4-caching-object-storage-redis-s3/
│   ├── capstone-5-performance-profiling-engine/
│   ├── capstone-6-distributed-raft-store/
│   ├── capstone-7-webassembly-edge-engine/
│   ├── capstone-8-clean-architecture-ddd/
│   └── capstone-9-linux-container-ebpf/
├── phase-5-web-frameworks-axum-actix-loco/
├── phase-6-database-engineering-sql/
├── phase-7-caching-and-object-storage/
├── phase-8-performance-and-profiling/
├── phase-9-distributed-systems/
├── phase-10-webassembly-and-edge/
├── phase-11-clean-code-and-craftsmanship/
├── phase-12-linux-systems-and-os-internals/
└── phase-13-zed-editor-developer-velocity/
```
