# Capstone 2: Multi-Tenant Enterprise REST & WebSocket API Engine

## 1. Project Overview

Capstone 2 builds a high-scale, multi-tenant enterprise backend using modern Rust web frameworks: **Axum**, **Tower**, and **Loco.rs**. 

This system acts as a production SaaS API backend featuring JWT authentication, granular Role-Based Access Control (RBAC), multi-tenant data isolation, bi-directional real-time WebSockets, background task processing, and automated OpenAPI (Swagger) documentation generation via `utoipa`.

```
                                 ┌────────────────────────────────────────────────────────┐
                                 │       Enterprise Axum / Loco.rs SaaS Engine            │
                                 │                                                        │
Client REST API  ──► [ Axum Router ] ──► [ Tower Middleware Layer ]                       │
(JSON / HTTPS)                   │            ├── Tracing & Request ID Injector           │
                                 │            ├── CORS & Security Headers                 │
                                 │            └── JWT & Tenant ID Extractor               │
                                 │                                                        │
                                 ├──► [ Multi-Tenant App State ]                          │
                                 │          ├── Postgres Tenant Isolation Pool            │
                                 │          ├── Redis Token Blocklist & Session Cache     │
                                 │          └── WebSocket Broadcast Hub (tokio::broadcast)│
                                 │                                                        │
Client WebSocket ──► [ WS Handler ] ◄──► [ Real-Time Channel Manager ] ◄── [ Event Bus ]  │
(WSS Protocol)                   │                                                        │
                                 ├──► [ Loco.rs Background Workers ]                      │
                                 │          ├── Email Delivery Worker                     │
                                 │          └── Analytics Aggregator Worker               │
                                 │                                                        │
OpenAPI Swagger  ◄── [ /docs ]   ◄── [ utoipa Auto-Generated Schema Engine ]              │
                                 └────────────────────────────────────────────────────────┘
```

---

## 2. Core Functional Requirements

### A. Multi-Tenant Architecture & State Extraction
- **Tenant Context Extraction**: Custom Axum `FromRequestParts` extractor that inspects subdomain headers (`tenant-a.api.io`) or JWT claims to inject a strongly-typed `TenantContext { tenant_id: Uuid, organization_name: String }` into route handlers.
- **Shared vs. Isolated Data Stores**: Automatic scoping of database queries so tenants can never read or mutate records belonging to other organizations.
- **Dependency Injection via `axum::extract::State`**: Zero-cost dependency injection of database connection pools, Redis clients, configuration registries, and event broadcast channels.

### B. Security, Authentication & RBAC
- **Asymmetric JWT Authentication**: Secure verification of RS256/EdDSA JWT access tokens with short TTLs (15 minutes) and refresh token rotation.
- **Redis Token Blocklist**: Instant revocation of compromised tokens and user session termination via fast Redis lookups.
- **Granular RBAC Guard**: Type-safe authorization layer (`#[require_permissions(Permission::ManageUsers, Permission::BillingRead)]`) returning `403 Forbidden` if user privileges are insufficient.
- **Argon2id Password Hashing**: Cryptographically secure user credential verification.

### C. Real-Time WebSocket & Server-Sent Events (SSE)
- **Bi-Directional WebSocket Hub**: Live multi-room WebSocket gateway supporting thousands of concurrent active connections.
- **Channel Subscriptions**: Clients subscribe to tenant-specific event channels (`room:tenant-101:notifications`).
- **Heartbeats & Ping/Pong**: Automatic disconnection of zombie sockets and client timeout detection.
- **Server-Sent Events (SSE)**: Streaming real-time status feeds (`/api/v1/stream/metrics`) for lightweight dashboard clients.

### D. Loco.rs Scaffolding & Background Task Queuing
- **MVC Architecture & Code Generation**: Fast scaffolding of models, controllers, and database migrations using the Loco.rs CLI.
- **Asynchronous Task Queue**: Background job processor for heavy workloads:
  - Welcome email sending with retry backoff.
  - PDF invoice generation.
  - Periodic hourly aggregation jobs.

### E. Declarative OpenAPI Documentation with `utoipa`
- **Interactive Swagger UI**: Exposes an interactive `/swagger-ui/` endpoint for testing and API discovery.
- **Type-Safe Schema Derivation**: Automatic derivation of JSON schemas directly from Rust request/response structs using `#[derive(ToSchema, IntoParams)]`.

---

## 3. Recommended Project Architecture & Module Hierarchy

```
capstone-web-api-engine/
├── Cargo.toml
├── src/
│   ├── main.rs                   # Server startup, listener binding, and graceful shutdown
│   ├── config.rs                 # Environment variables and application settings
│   ├── state.rs                  # Shared AppState (DB pools, Redis, WS broadcaster)
│   ├── extractors/
│   │   ├── mod.rs
│   │   ├── auth.rs               # Claims extractor and JWT validator
│   │   ├── tenant.rs             # Multi-tenant context extractor
│   │   └── validated_json.rs     # JSON payload extractor with validator integration
│   ├── routes/
│   │   ├── mod.rs                # Master router composition and Swagger UI mounting
│   │   ├── auth_routes.rs        # /api/v1/auth (login, register, refresh, logout)
│   │   ├── user_routes.rs        # /api/v1/users (CRUD operations with RBAC)
│   │   ├── tenant_routes.rs      # /api/v1/tenants (organization management)
│   │   ├── websocket_routes.rs   # /api/v1/ws (WebSocket upgrade and room subscription)
│   │   └── sse_routes.rs         # /api/v1/stream/events (Server-Sent Events)
│   ├── services/
│   │   ├── mod.rs
│   │   ├── auth_service.rs       # Token generation, Argon2 hashing, Redis blocklist
│   │   └── notification.rs       # Event broadcasting and dispatching
│   ├── workers/
│   │   ├── mod.rs
│   │   ├── email_worker.rs       # Background task for sending transactional emails
│   │   └── report_worker.rs      # Background report aggregation job
│   └── docs/
│       ├── mod.rs
│       └── openapi.rs            # Master utoipa OpenAPI schema definition
└── tests/
    ├── auth_integration_test.rs  # Login, JWT, and permission enforcement tests
    ├── tenant_isolation_test.rs  # Cross-tenant data leakage prevention tests
    └── websocket_test.rs         # Real-time pub/sub verification tests
```

---

## 4. API Endpoints Specification

| Method | Endpoint | Description | Auth Required |
| :--- | :--- | :--- | :--- |
| `POST` | `/api/v1/auth/register` | Register new organization & admin user | Public |
| `POST` | `/api/v1/auth/login` | Authenticate and obtain JWT access/refresh pair | Public |
| `POST` | `/api/v1/auth/refresh` | Rotate access token using valid refresh token | Public |
| `POST` | `/api/v1/auth/logout` | Revoke current token into Redis blocklist | Bearer Token |
| `GET` | `/api/v1/users` | List users within the current tenant | `Permission::UserRead` |
| `POST` | `/api/v1/users` | Create user with assigned role | `Permission::UserWrite` |
| `GET` | `/api/v1/ws` | Upgrade connection to full-duplex WebSocket | Bearer Token (Query Param) |
| `GET` | `/api/v1/stream/events` | Stream real-time tenant notifications via SSE | Bearer Token |
| `GET` | `/swagger-ui/` | Interactive Swagger documentation interface | Public |
