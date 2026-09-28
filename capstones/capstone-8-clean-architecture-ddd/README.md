# Capstone 8: Enterprise Domain-Driven Design (DDD) & Clean Architecture Core Engine

## 1. Project Overview

Capstone 8 is an enterprise-grade, modular **Order Fulfillment & Multi-Currency Settlement Engine** built strictly adhering to **Clean Architecture (Hexagonal Architecture / Ports and Adapters)** and **Domain-Driven Design (DDD)** principles in pure idiomatic Rust.

This capstone mimics the core transactional backbone of modern e-commerce and financial platforms (such as **Shopify**, **Stripe Billing**, and **Amazon Order Management**). It demonstrates how to structure large-scale Rust codebases with zero coupling between business rules and external databases, web frameworks, or third-party APIs.

```
┌────────────────────────────────────────────────────────────────────────────────────────┐
│                   Hexagonal Clean Architecture & DDD Core Engine                       │
├────────────────────────────────────────────────────────────────────────────────────────┤
│                                                                                        │
│  [ Driving Adapters (Input) ]                                                          │
│  ┌───────────────────────────┐   ┌───────────────────────────┐                         │
│  │ Axum REST API Controllers │   │ Tonic gRPC Service Stubs  │                         │
│  └─────────────┬─────────────┘   └─────────────┬─────────────┘                         │
│                │ (Invokes Commands/Queries)    │                                       │
│                ▼                               ▼                                       │
│  ┌──────────────────────────────────────────────────────────────────────────────────┐  │
│  │ Application Layer (Use Cases / CQRS)                                             │  │
│  │ [ PlaceOrderUseCase ]   [ PayOrderUseCase ]   [ SettleMerchantBalanceUseCase ]   │  │
│  │                                                                                  │  │
│  │ Input / Output Ports (Abstract Traits):                                          │  │
│  │ - trait OrderRepositoryPort                                                      │  │
│  │ - trait PaymentGatewayPort                                                       │  │
│  │ - trait DomainEventPublisherPort                                                 │  │
│  │ - trait CurrencyExchangePort                                                     │  │
│  └──────────────────────────────────────────┬───────────────────────────────────────┘  │
│                                             │ (Invokes Domain Logic)                   │
│                                             ▼                                          │
│  ┌──────────────────────────────────────────────────────────────────────────────────┐  │
│  │ Domain Core Layer (Pure Rust - Zero External Dependencies!)                      │  │
│  │                                                                                  │  │
│  │  • Aggregates:     OrderAggregate, MerchantAccountAggregate                      │  │
│  │  • Value Objects:  Money, CurrencyCode, EmailAddress, OrderId, TrackingNumber    │  │
│  │  • Type-States:    Order<Draft> ➔ Order<Paid> ➔ Order<Shipped>                   │  │
│  │  • Domain Events:  OrderPlacedEvent, PaymentReceivedEvent, OrderCancelledEvent   │  │
│  └──────────────────────────────────────────────────────────────────────────────────┘  │
│                                             ▲                                          │
│                                             │ (Implements Ports)                       │
│  ┌──────────────────────────────────────────┴───────────────────────────────────────┐  │
│  │ Infrastructure Layer (Driven Adapters / Output)                                  │  │
│  │                                                                                  │  │
│  │  • PostgresOrderRepository (SQLx PostgreSQL with ACID Transactions)              │  │
│  │  • StripePaymentAdapter (HTTP REST Client with Exponential Backoff)              │  │
│  │  • RedisEventPublisher (Redis Streams & Pub/Sub Message Bus)                     │  │
│  │  • InMemoryTestFakes (100% Deterministic In-Memory Test Harness)                 │  │
│  └──────────────────────────────────────────────────────────────────────────────────┘  │
└────────────────────────────────────────────────────────────────────────────────────────┘
```

---

## 2. Core Functional Requirements

### A. Domain Core Layer (`domain/`)
- **Strict Inward Dependency**: The domain crate/module must contain **zero external infrastructure dependencies** (only `std` and zero-cost standard traits).
- **Domain Aggregates & Invariants**:
  - `OrderAggregate`: Encapsulates line items, discounts, shipping address, and fulfillment status. Protects the invariant that order items cannot be modified once paid.
  - `MerchantAccountAggregate`: Encapsulates ledger balances, currency conversion, and payout schedules. Protects invariant that account balance cannot go negative during payout.
- **Value Objects (Parse, Don't Validate)**:
  - `EmailAddress`: Validated at construction; cannot represent invalid emails.
  - `Money`: Multi-currency monetary amount storing cents as `i64` and ISO-4217 `CurrencyCode` (`USD`, `EUR`, `GBP`, `JPY`). Prevents adding mismatched currencies without explicit exchange rates.
- **Type-State Finite State Machine**:
  - `Order<Draft>` ➔ `.pay(tx)` ➔ `Order<Paid>` ➔ `.ship(tracking)` ➔ `Order<Shipped>`.
  - Calling `.ship()` on `Order<Draft>` must fail at compile time.
- **Domain Events**:
  - Emits strongly typed events: `OrderCreatedEvent`, `OrderPaidEvent`, `OrderShippedEvent`, `OrderCancelledEvent`.

### B. Application Layer (`application/`)
- **CQRS (Command Query Responsibility Segregation)**:
  - **Commands**: `PlaceOrderCommand`, `ProcessPaymentCommand`, `ShipOrderCommand`, `CancelOrderCommand`.
  - **Queries**: `GetOrderByIdQuery`, `ListCustomerOrdersQuery`, `GetMerchantBalanceQuery`.
- **Abstract Ports (Traits)**:
  - `OrderRepositoryPort`: `load(id) -> Option<OrderAggregate>`, `save(&OrderAggregate) -> Result<()>`.
  - `PaymentGatewayPort`: `charge(email, amount) -> Result<TransactionId>`.
  - `DomainEventPublisherPort`: `publish(events: &[DomainEvent])`.
  - `CurrencyExchangePort`: `get_rate(from: CurrencyCode, to: CurrencyCode) -> f64`.

### C. Infrastructure Layer (`infrastructure/`)
- **PostgreSQL Adapter (`sqlx`)**: Implements `OrderRepositoryPort` mapping domain aggregates to relational database tables with transactional consistency.
- **Payment Gateway Adapter**: Implements `PaymentGatewayPort` with transient error retries and timeout management.
- **Redis Event Publisher Adapter**: Implements `DomainEventPublisherPort` broadcasting domain events to Redis Streams.
- **In-Memory Test Fakes**: Complete in-memory implementations of all ports enabling sub-millisecond unit test execution.

### D. Presentation / API Layer (`presentation/`)
- **Axum Web Handlers**: Exposes REST endpoints (`POST /api/v1/orders`, `POST /api/v1/orders/:id/pay`, `POST /api/v1/orders/:id/ship`).
- **OpenAPI 3.0 Documentation**: Declarative Swagger UI generated via `utoipa`.

---

## 3. Clean Code & Software Craftsmanship Standards

1. **SOLID Principles Enforced**:
   - Single Responsibility on all use cases and aggregates.
   - Open/Closed via extension traits.
   - Interface Segregation via single-purpose micro-traits.
   - Dependency Inversion with traits decoupling domain from infrastructure.
2. **Zero Defensive Clones**: Use references, borrows, and `Cow<'a, T>` for efficient data processing.
3. **No Unwraps or Panics**: All fallible paths return structured domain errors (`thiserror`) converted into appropriate HTTP status codes.
4. **Property-Based Testing Suite (`proptest`)**:
   - At least 10 property tests validating mathematical invariants (e.g. tax calculations, currency conversions, and discount bounds across 10,000 randomized inputs).

---

## 4. Suggested Directory Structure

```
capstone-8-clean-architecture-ddd/
├── Cargo.toml
├── domain/                     # Layer 1: Core Domain (Zero External Dependencies)
│   ├── src/
│   │   ├── lib.rs
│   │   ├── entities.rs         # OrderAggregate, MerchantAccount
│   │   ├── value_objects.rs    # Money, CurrencyCode, EmailAddress, OrderId
│   │   ├── events.rs           # DomainEvent enum
│   │   └── typestate.rs        # Type-State Order FSM
├── application/                # Layer 2: Application Use Cases & Ports
│   ├── src/
│   │   ├── lib.rs
│   │   ├── ports.rs            # OrderRepositoryPort, PaymentGatewayPort, EventBusPort
│   │   ├── commands/           # PlaceOrderHandler, PayOrderHandler
│   │   └── queries/            # GetOrderHandler
├── infrastructure/             # Layer 3: Adapters (DB, Network, Redis)
│   ├── src/
│   │   ├── lib.rs
│   │   ├── postgres/           # SQLx Repository implementation
│   │   ├── redis/              # Redis Event Publisher implementation
│   │   ├── payment/            # Mock & External Payment Gateway
│   │   └── fakes/              # In-Memory Test Doubles
├── presentation/               # Layer 4: Driving HTTP API (Axum)
│   ├── src/
│   │   ├── lib.rs
│   │   ├── routes.rs           # Axum Route Handlers
│   │   ├── dtos.rs             # Request/Response DTOs with utoipa schemas
│   │   └── openapi.rs          # Swagger UI / OpenAPI 3.0 configuration
├── tests/
│   ├── property_tests.rs       # Proptest property-based verification
│   └── integration_tests.rs    # End-to-end use-case tests with In-Memory Fakes
└── README.md
```

---

## 5. Verification & Acceptance Criteria

To pass Capstone 8, the codebase must satisfy:

1. **Architecture Linter / Dependency Verification**:
   - `domain` crate must not have `sqlx`, `axum`, `tokio`, or `serde_json` in its `[dependencies]`.
2. **Compile-Time State Safety**:
   - Attempting to call `.ship()` on a draft order fails compilation.
3. **Property-Based Test Suite**:
   - `cargo test` executes 10,000 randomized property test cases with 100% pass rate.
4. **End-to-End Use Case Execution**:
   - Running the full application successfully processes orders through the complete DDD lifecycle, emits domain events, and stores states in PostgreSQL.
