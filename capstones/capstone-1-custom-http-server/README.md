# Capstone 1: High-Performance Custom HTTP/1.1 Server & Reverse Proxy

## 1. Project Overview

Capstone 1 is a production-grade, zero-third-party-dependency (core runtime built on `std::net` and standard primitives, with an optional Tokio async backend engine) **HTTP/1.1 Web Server and Reverse Proxy**. 

You will build the entire networking, request parsing, response serialization, routing, middleware, connection pooling, and upstream proxying pipeline from the ground up to gain deep mechanical mastery over low-level socket I/O.

```
                                  ┌────────────────────────────────────────────────────────┐
                                  │           Custom HTTP/1.1 Server Engine                │
                                  │                                                        │
Client HTTP/1.1 ──► [ TCP Socket ] ──► [ Incremental Byte Buffer ]                          │
Requests            (Port 8080)   │              │                                         │
                                  │              ▼                                         │
                                  │       [ Zero-Copy Parser ]                             │
                                  │              │ (Extracts Method, Path, Headers, Body)  │
                                  │              ▼                                         │
                                  │    [ Middleware Pipeline ] (Logger, Auth, Rate Limiter)│
                                  │              │                                         │
                                  │              ├──► [ Static File & API Router ] ──────┐ │
                                  │              │                                       │ │
                                  │              └──► [ Reverse Proxy & Load Balancer ] ─┤ │
                                  │                                                      │ │
                                  │              ┌───────────────────────────────────────┘ │
                                  │              ▼                                         │
                                  │       [ Type-Safe Response Serializer ]                │
                                  │              │ (Computes Content-Length, Status, CRLF) │
                                  │              ▼                                         │
Client Response ◄── [ TCP Socket ] ◄── [ Direct Socket Writer (Chunked / Buffered) ]       │
                                  └────────────────────────────────────────────────────────┘
```

---

## 2. Core Functional Requirements

### A. HTTP Protocol Engine
- **HTTP/1.1 Compliance**: Strict RFC 7230 / RFC 9112 conformance.
- **Zero-Copy Request Parsing**: Borrow method, path, query parameters, and headers directly from the socket buffer slice (`&'a str` / `&'a [u8]`) without allocating intermediate `String` instances.
- **HTTP Methods**: Full support for `GET`, `POST`, `PUT`, `DELETE`, `PATCH`, `HEAD`, and `OPTIONS`.
- **Persistent Connections (Keep-Alive)**: Support connection reuse with configurable keep-alive timeouts and maximum request limits per TCP connection before closing.
- **Chunked Transfer Encoding**: Support streaming large payloads using `Transfer-Encoding: chunked` (decoding incoming chunked requests and encoding outgoing chunked responses).

### B. Routing & Static File Serving
- **Segment Tree / Trie Routing**: Dynamic route matching with path variables (`/api/v1/users/:id/posts/:post_id`) and wildcard catch-alls (`/static/*filepath`).
- **Query Parameter Decoder**: Automatic parsing of URL-encoded query strings (`?filter=active&sort=desc&page=2`).
- **Static File Handler**: High-throughput file serving with automatic MIME-type detection (`text/html`, `application/javascript`, `image/png`, `application/json`), HTTP `304 Not Modified` caching via `ETag` and `If-None-Match`, and range requests (`Range: bytes=0-1023` with `206 Partial Content`).

### C. Reverse Proxy & Load Balancing
- **Upstream Forwarding**: Ability to act as a reverse proxy, forwarding client requests to multiple upstream backend servers.
- **Load Balancing Algorithms**:
  - Round-Robin distribution.
  - Least-Connections routing.
  - IP-Hash sticky session routing.
- **Upstream Health Checks**: Background worker periodically probing backend endpoints (`/health`) to automatically remove dead nodes from the active pool.
- **Header Forwarding**: Automatic injection of standard proxy headers: `X-Forwarded-For`, `X-Forwarded-Proto`, `X-Forwarded-Host`, `X-Real-IP`.

### D. Middleware Architecture
- **Structured Access Logger**: Emits JSON-formatted access logs containing timestamp, client IP, method, path, HTTP status code, request duration in microseconds, and user-agent.
- **Token Authentication**: Bearer token and API-key validation middleware with configurable protected route prefixes.
- **Token Bucket Rate Limiter**: Per-IP rate limiting (e.g. 100 requests per minute) returning `429 Too Many Requests` with `Retry-After` headers upon breach.
- **Security Headers Injector**: Enforces `X-Content-Type-Options: nosniff`, `X-Frame-Options: DENY`, `Content-Security-Policy`, and `Strict-Transport-Security`.
- **CORS Handler**: Configurable Cross-Origin Resource Sharing handling `OPTIONS` preflight requests, allowed origins, and credentials headers.

---

## 3. Recommended Project Architecture & Module Hierarchy

```
capstone-http-server/
├── Cargo.toml
├── src/
│   ├── main.rs                   # CLI entrypoint and configuration loading
│   ├── config.rs                 # Server and reverse proxy YAML/TOML configuration
│   ├── server.rs                 # TCP listener loop and connection dispatcher
│   ├── connection.rs             # Client socket handler and keep-alive state machine
│   ├── http/
│   │   ├── mod.rs
│   │   ├── method.rs             # HTTP Method enum and parsers
│   │   ├── status.rs             # HTTP Status codes and reason phrases
│   │   ├── request.rs            # Zero-copy HttpRequest parser and query extractor
│   │   ├── response.rs           # Type-safe HttpResponse builder and wire serializer
│   │   └── chunked.rs            # Chunked transfer encoding and decoding streams
│   ├── router/
│   │   ├── mod.rs
│   │   ├── trie.rs               # Segment prefix tree for O(segments) route lookups
│   │   ├── route.rs              # Route definitions and handler closures
│   │   └── static_files.rs       # Static asset caching, MIME detection, and ETag generator
│   ├── middleware/
│   │   ├── mod.rs
│   │   ├── pipeline.rs           # Composable Chain-of-Responsibility execution engine
│   │   ├── logger.rs             # Structured JSON telemetry and latency timer
│   │   ├── auth.rs               # Bearer and API-Key authorization filter
│   │   ├── rate_limit.rs         # Token Bucket rate limiter
│   │   ├── cors.rs               # CORS preflight and response header injection
│   │   └── security.rs           # Hardened security headers
│   └── proxy/
│       ├── mod.rs
│       ├── balancer.rs           # Round-Robin, Least-Connections, and IP-Hash dispatchers
│       ├── client.rs             # Raw TCP upstream client forwarder
│       └── health.rs             # Background health check daemon
└── tests/
    ├── http_parser_test.rs       # RFC compliance test suite
    ├── router_test.rs            # Parametric and wildcard route tests
    ├── proxy_test.rs             # Load balancer and failover integration tests
    └── load_test.sh              # Benchmarking script with wrk / vegeta
```

---

## 4. Configuration Specification (`server.toml`)

```toml
[server]
host = "0.0.0.0"
port = 8080
workers = 8
max_connections = 10000
keep_alive_timeout_secs = 60
max_payload_mb = 10

[static_files]
enabled = true
route_prefix = "/public"
directory = "./static"
cache_max_age_secs = 86400

[rate_limiting]
enabled = true
requests_per_minute = 120
burst_capacity = 20

[reverse_proxy]
enabled = true
strategy = "round_robin" # "round_robin" | "least_connections" | "ip_hash"
health_check_interval_secs = 5

[[reverse_proxy.upstreams]]
id = "backend-alpha"
url = "http://127.0.0.1:9001"
weight = 1

[[reverse_proxy.upstreams]]
id = "backend-bravo"
url = "http://127.0.0.1:9002"
weight = 2
```

---

## 5. Performance & Verification Target

| Metric | Target |
| :--- | :--- |
| **Throughput** | $\ge 60,000$ req/sec on modern 8-core CPU |
| **P99 Latency** | $< 1.5\text{ ms}$ under concurrent load |
| **Memory Footprint** | $< 25\text{ MB}$ RSS under 5,000 idle keep-alive connections |
| **Test Coverage** | Comprehensive unit tests for parser, router, middleware, and proxy failovers |
