# Capstone 3: High-Throughput Financial Ledger & SQL Query Engine

## 1. Project Overview

Capstone 3 builds a mission-critical, double-entry financial accounting ledger and relational SQL query engine in Rust using **SQLx**, **PostgreSQL**, and **SeaORM**.

Financial systems cannot tolerate race conditions, negative balances, lost updates, or database connection starvation. This project requires implementing strict ACID transaction guarantees, pessimistic row locking (`SELECT FOR UPDATE`), automated version-controlled database migrations, compile-time SQL verification, and sub-millisecond account balance calculations under heavy concurrent load.

```
                               ┌────────────────────────────────────────────────────────┐
                               │     High-Throughput Financial SQL Ledger Engine        │
                               │                                                        │
Client Transaction Requests ──►│ [ SQLx Connection Pool ] (Hikari-Style Multiplexing)   │
(Debit / Credit / Transfer)    │           │                                            │
                               │           ▼                                            │
                               │ [ Strict ACID Transaction Manager ]                    │
                               │           │ (Isolation Level: Serializable / Row Lock) │
                               │           ├── Step 1: Lock Source & Destination Rows   │
                               │           │           (SELECT ... FOR UPDATE)          │
                               │           ├── Step 2: Validate Sufficient Balance      │
                               │           ├── Step 3: Insert Double-Entry Journal Post │
                               │           ├── Step 4: Update Balances Atomically       │
                               │           └── Step 5: Commit or Rollback on Failure    │
                               │                                                        │
                               ├──► [ Automated Schema Migration Engine ]               │
                               │          └── Runs embedded migrations on startup       │
                               │                                                        │
                               ├──► [ Audit Trail & Checksum Verifier ]                 │
                               │          └── SHA-256 Merkle tree of all historical txs │
                               │                                                        │
Analytics & Audit Queries   ◄──│ [ Sub-Millisecond Indexed Query Engine ]               │
                               │          └── B-Tree composite & GIN indexed queries    │
                               └────────────────────────────────────────────────────────┘
```

---

## 2. Core Functional Requirements

### A. Double-Entry Accounting Invariant
- **Fundamental Invariant**: Every transaction consists of at least two journal entries (a **Debit** and a **Credit**). The mathematical sum of debits must **identically equal** the sum of credits ($\sum \text{Debits} = \sum \text{Credits}$).
- **Immutable Ledger History**: Ledger entries are append-only. To reverse an erroneous transaction, an offset transaction must be posted; historical entries are never updated or deleted.

### B. Concurrency Control & ACID Guarantees
- **Pessimistic Row-Level Locking**: Concurrent transfers between accounts must acquire row locks (`SELECT ... FOR UPDATE`) in deterministic order (e.g., sorted by `account_id`) to **prevent deadlocks**.
- **Serializable Isolation**: High-value batch settlements execute under `SERIALIZABLE` transaction isolation to eliminate phantom reads and write skew anomalies.
- **Zero Negative Balances**: Database-level `CHECK (balance >= 0)` constraints combined with pre-commit application logic guarantees accounts can never go into an unapproved overdraft.

### C. Compile-Time SQL Verification with SQLx
- **`sqlx::query!` and `sqlx::query_as!`**: 100% compile-time verification of SQL syntax, table schemas, and column type alignments against a live or offline cached PostgreSQL schema.
- **Connection Pool Tuning**: Configurable minimum idle connections, maximum active pool size, acquisition timeouts, and connection health liveness pings.

### D. Automated Migrations & Schema Evolution
- **Embedded Migrations**: Uses `sqlx::migrate!()` to apply schema updates automatically on application startup.
- **Idempotent Migration Scripts**: Reversible `up.sql` and `down.sql` migrations tracking schema versions in a `_sqlx_migrations` table.

### E. Audit Trail & Merkle Cryptographic Verification
- **Audit Verification**: Every journal entry includes a cryptographic SHA-256 hash chaining back to the previous entry, proving the accounting history has never been tampered with.

---

## 3. Recommended Project Architecture & Module Hierarchy

```
capstone-financial-ledger/
├── Cargo.toml
├── migrations/
│   ├── 20260904000001_create_accounts_table.sql
│   ├── 20260904000002_create_transactions_table.sql
│   └── 20260904000003_create_journal_entries_table.sql
├── src/
│   ├── main.rs                   # App initialization, migration runner, and CLI
│   ├── config.rs                 # Database URL, pool size, and timeout settings
│   ├── db/
│   │   ├── mod.rs
│   │   ├── pool.rs               # PgPool builder and connection health checks
│   │   └── migrations.rs         # Automated embedded migration executor
│   ├── models/
│   │   ├── mod.rs
│   │   ├── account.rs            # Account struct (ID, Currency, Balance, Status)
│   │   ├── transaction.rs        # Transaction metadata (ID, Reference, Timestamp)
│   │   └── journal_entry.rs      # Journal entry (Debit / Credit, Amount, Hash)
│   ├── repository/
│   │   ├── mod.rs
│   │   ├── account_repo.rs       # SQL queries for account management and locks
│   │   └── ledger_repo.rs        # Double-entry posting and balance queries
│   ├── service/
│   │   ├── mod.rs
│   │   ├── transfer_service.rs   # ACID multi-party money transfer orchestration
│   │   └── audit_service.rs      # Merkle chain integrity verification
│   └── api/
│       ├── mod.rs
│       ├── handlers.rs           # REST endpoints for ledger transactions
│       └── error.rs              # Custom domain errors (InsufficientFunds, Deadlock)
└── tests/
    ├── concurrent_transfers_test.rs # 100 concurrent threads transferring money
    ├── migration_rollback_test.rs   # Verifying reversible schema migrations
    └── audit_integrity_test.rs      # Tamper detection verification
```

---

## 4. Database Schema Specification

```sql
CREATE TABLE accounts (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    account_number VARCHAR(32) UNIQUE NOT NULL,
    currency VARCHAR(3) NOT NULL,
    balance_cents BIGINT NOT NULL DEFAULT 0 CHECK (balance_cents >= 0),
    is_frozen BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE transactions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    reference_id VARCHAR(64) UNIQUE NOT NULL,
    description TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE journal_entries (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    transaction_id UUID NOT NULL REFERENCES transactions(id) ON DELETE RESTRICT,
    account_id UUID NOT NULL REFERENCES accounts(id) ON DELETE RESTRICT,
    entry_type VARCHAR(6) NOT NULL CHECK (entry_type IN ('DEBIT', 'CREDIT')),
    amount_cents BIGINT NOT NULL CHECK (amount_cents > 0),
    previous_hash VARCHAR(64) NOT NULL,
    entry_hash VARCHAR(64) NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_journal_account_created ON journal_entries(account_id, created_at DESC);
```

---

## 5. Stress Testing & Verification Target

| Test Scenario | Condition | Pass Criteria |
| :--- | :--- | :--- |
| **Concurrent Balance Integrity** | 100 concurrent threads executing 10,000 randomized transfers between 10 accounts | Total currency across all accounts before test $\equiv$ Total currency after test ($\pm 0\text{ cents}$) |
| **Deadlock Resistance** | Thread A transfers $1 \to 2$ while Thread B simultaneously transfers $2 \to 1$ | Zero deadlock panics; deterministic lock acquisition order |
| **Zero Overdrafts** | 50 concurrent threads trying to drain an account with \$100 balance | Account stops precisely at \$0; exactly 1 transfer succeeds |
