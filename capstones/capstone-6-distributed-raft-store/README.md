# Capstone 6: Fault-Tolerant Distributed Key-Value Store with Raft Consensus

## 1. Project Overview

Capstone 6 is an advanced distributed systems project in Rust. You will build a clustered, fault-tolerant, strongly consistent (linearizable) **Distributed Key-Value Store** implementing the complete **Raft Consensus Algorithm** from scratch.

A cluster of 3, 5, or 7 independent nodes will elect a leader, replicate write-ahead logs (WAL) across network partitions, survive node crashes and power failures without data loss, and serve client reads and writes via a **gRPC API** powered by **`tonic`** and **Protocol Buffers**.

```
                               ┌────────────────────────────────────────────────────────┐
                               │       Clustered Distributed Raft Consensus Store       │
                               │                                                        │
Client gRPC Request ──────────►│ [ Node 1: Current Raft Leader ]                        │
(Set "user:1" = "Alice")       │           │                                            │
                               │           ├── Step 1: Append to Leader Local WAL       │
                               │           ├── Step 2: Replicate via AppendEntries RPC  │
                               │           │           ┌────────────────────────────────┤
                               │           │           │                                │
                               │           ▼           ▼                                │
                               │ [ Node 2: Follower ]    [ Node 3: Follower ]           │
                               │ (Appends to WAL)        (Appends to WAL)               │
                               │           │                   │                        │
                               │           └───────────┬───────┘                        │
                               │                       ▼ (Quorum Majority 2/3 ACK)      │
                               │           ├── Step 3: Leader Commits Log Entry         │
                               │           ├── Step 4: Apply to In-Memory State Machine │
                               │           └── Step 5: Return Success to Client         │
                               │                                                        │
                               ├──► [ Automated Heartbeat & Randomized Election Timer ] │
                               │          └── If Leader fails -> Followers elect Leader │
                               │                                                        │
Cluster State Snapshots        ◄─── [ Snapshot Engine & Log Compaction Unit ]           │
                               └────────────────────────────────────────────────────────┘
```

---

## 2. Core Functional Requirements

### A. The Raft Consensus State Machine
- **Three Node Roles**: Every cluster node transitions dynamically between `Follower`, `Candidate`, and `Leader`.
- **Randomized Election Timers**: Heartbeat timeouts randomized between $150\text{ms}$ and $300\text{ms}$ to prevent split-vote deadlocks during leader elections.
- **RequestVote RPC**: Candidates request votes with term numbers and log recency checks; a candidate requires a strict majority quorum ($N/2 + 1$) to become Leader.
- **AppendEntries RPC**: The Leader sends periodic empty heartbeats to maintain authority and packages log entries for replication.

### B. Persistent Write-Ahead Log (WAL) & Crash Recovery
- **Durable Disk Storage**: All term records, voted-for states, and uncommitted log entries are written to an append-only disk file with explicit `fsync` flushing.
- **Crash Recovery**: When a crashed node reboots, it replays its WAL from disk to restore its exact state before rejoining the cluster.

### C. Log Compaction & Snapshotting
- **Log Compaction**: When the WAL reaches a size threshold (e.g. 100,000 entries), the node takes an atomic snapshot of the in-memory state machine and truncates earlier log entries.
- **InstallSnapshot RPC**: If a follower falls too far behind the Leader's truncated log, the Leader streams its latest snapshot over gRPC.

### D. Linearizable Client gRPC Interface (`tonic`)
- **Strong Consistency**: Read and write requests are routed to the current leader. If a client connects to a follower, the follower returns the network address of the current leader.
- **Read-Index / Lease Read Optimization**: Ensure reads reflect the latest committed data without requiring a full round of log replication on every read.

---

## 3. Recommended Project Architecture & Module Hierarchy

```
capstone-distributed-raft/
├── Cargo.toml
├── proto/
│   ├── raft.proto                # Consensus RPCs (RequestVote, AppendEntries, InstallSnapshot)
│   └── kvstore.proto             # Client Key-Value gRPC interface (Get, Put, Delete)
├── src/
│   ├── main.rs                   # Cluster node startup and CLI flags (--node-id, --peers)
│   ├── config.rs                 # Cluster topology and election timeout settings
│   ├── consensus/
│   │   ├── mod.rs
│   │   ├── raft.rs               # Main Raft actor event loop
│   │   ├── state.rs              # Role states (Follower, Candidate, Leader)
│   │   ├── timer.rs              # Randomized election and heartbeat timers
│   │   └── rpc_handlers.rs       # Processing incoming consensus RPCs
│   ├── storage/
│   │   ├── mod.rs
│   │   ├── wal.rs                # Append-only Write-Ahead Log with fsync
│   │   └── snapshot.rs           # State machine snapshot serializer
│   ├── state_machine/
│   │   ├── mod.rs
│   │   └── kv_store.rs           # In-memory Key-Value store applying committed logs
│   ├── network/
│   │   ├── mod.rs
│   │   ├── server.rs             # gRPC server hosting Raft and KV services
│   │   └── peer_client.rs        # Multiplexed gRPC client connecting to peer nodes
│   └── client/
│       ├── mod.rs
│       └── cluster_client.rs     # Smart client with automated leader discovery & retry
└── tests/
    ├── election_test.rs          # Leader failure and new election verification
    ├── log_replication_test.rs   # Quorum write and crash consistency test
    ├── network_partition_test.rs # Split-brain network isolation test (Jepsen-style)
    └── snapshot_test.rs          # Log compaction and InstallSnapshot test
```

---

## 4. Raft Protocol RPC Specification (`raft.proto`)

```protobuf
syntax = "proto3";
package raft;

service RaftService {
  rpc RequestVote (VoteRequest) returns (VoteResponse);
  rpc AppendEntries (AppendRequest) returns (AppendResponse);
  rpc InstallSnapshot (SnapshotRequest) returns (SnapshotResponse);
}

message LogEntry {
  uint64 term = 1;
  uint64 index = 2;
  bytes command = 3;
}

message AppendRequest {
  uint64 term = 1;
  uint64 leader_id = 2;
  uint64 prev_log_index = 3;
  uint64 prev_log_term = 4;
  repeated LogEntry entries = 5;
  uint64 leader_commit = 6;
}

message AppendResponse {
  uint64 term = 1;
  bool success = 2;
  uint64 match_index = 3;
}
```

---

## 5. Distributed Chaos & Fault-Tolerance Verification Targets

| Failure Scenario | Condition | Pass Criteria |
| :--- | :--- | :--- |
| **Leader Crash** | Kill leader process in a 3-node cluster | Remaining 2 followers elect a new leader within $< 400\text{ ms}$; cluster continues accepting writes |
| **Network Partition (Majority vs Minority)** | Isolate Node 1 & 2 from Node 3, 4, 5 in a 5-node cluster | The majority partition (Nodes 3,4,5) elects a leader and continues serving writes; minority partition rejects writes; upon partition heal, minority reconciles state without data loss |
| **Power-Loss Simulation (`SIGKILL`)** | Abruptly kill node with pending uncommitted logs | Upon restart, uncommitted divergent logs are overwritten by the leader; committed logs remain intact |
