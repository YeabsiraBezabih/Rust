# Capstone 4: Distributed Real-Time Media Storage & Queue Pipeline

## 1. Project Overview

Capstone 4 builds a high-throughput, distributed media ingestion, processing, and caching architecture in Rust utilizing **Redis (or Dragonfly)** for distributed message queues and caching, alongside **Amazon S3 (or MinIO)** for blob object storage.

The system handles direct multipart binary uploads, coordinates asynchronous video/image transcoding workers via Redis Streams and Consumer Groups, implements cache-stampede-protected metadata caching, and generates secure, time-limited presigned S3 URLs for client downloads.

```
                                ┌────────────────────────────────────────────────────────┐
                                │   Distributed Media Pipeline (Redis + S3 / MinIO)      │
                                │                                                        │
Client Direct Upload ──────────►│ [ Media Ingestion Gateway (Axum / Tokio) ]             │
(Large Video / Audio / Image)   │           │                                            │
                                │           ├──► Generates Presigned Multipart S3 Upload │
                                │           │    Token via `aws-sdk-s3`                  │
                                │           ▼                                            │
                                │ [ MinIO / S3 Object Storage ] (Raw Ingestion Bucket)   │
                                │           │                                            │
                                │           ▼ (Upload Complete Event)                    │
                                │ [ Redis Stream: `stream:media:jobs` ]                  │
                                │           │                                            │
                                ├──► [ Redis Consumer Group Worker Pool ]                │
                                │          ├── Transcoder Worker 1 (Image Thumbnailer)   │
                                │          ├── Transcoder Worker 2 (HLS Video Segmenter) │
                                │          └── Audio Extractor Worker                    │
                                │                                                        │
                                ├──► [ Processed S3 Storage ] (Delivery Bucket)          │
                                │                                                        │
                                ├──► [ Redis Cache-Aside Layer with XFetch ]             │
                                │          └── Probabilistic early refresh for metadata  │
                                │                                                        │
Client Stream Download ◄────────│ [ Presigned S3 Download Gateway ]                      │
                                └────────────────────────────────────────────────────────┘
```

---

## 2. Core Functional Requirements

### A. S3 / MinIO Object Storage Engine
- **Direct S3 Streaming**: Stream binary buffers directly from incoming HTTP requests to MinIO/S3 using the `aws-sdk-s3` crate without holding multi-gigabyte files in RAM.
- **Multipart Chunked Uploads**: Support files $> 5\text{GB}$ split into 10MB chunks with parallel chunk uploads and automated aborts on failure.
- **Presigned Upload & Download URLs**: Generate cryptographically signed URLs with configurable expiration lifetimes (e.g., 15-minute validity) allowing browsers to upload and download directly to S3.
- **Bucket Lifecycle & Tagging**: Automated tagging of files (`status=processing`, `retention=30d`) and multipart upload cleanup for orphaned parts.

### B. Redis Distributed Caching & Anti-Stampede
- **Cache-Aside Pattern**: High-speed metadata retrieval with automated database fallback.
- **Cache Stampede Prevention via XFetch Algorithm**: Uses probabilistic early expiration ($\Delta \cdot \beta \ln(\text{rand})$) to trigger background cache refreshes *before* hot cache keys expire, preventing 10,000 concurrent requests from hammering the database simultaneously.
- **Redis Connection Multiplexing**: Thread-safe connection pooling using `bb8-redis` or `deadpool-redis`.

### C. Distributed Job Queuing with Redis Streams
- **Redis Streams (`XADD`, `XREADGROUP`, `XACK`)**: Reliable distributed work queue with persistent consumer groups.
- **At-Least-Once Delivery**: Workers acknowledge (`XACK`) processed jobs only after successful S3 write and transcode.
- **Dead-Letter Queue (DLQ) & Auto-Claim**: Background monitor identifying stalled jobs (using `XPENDING` and `XCLAIM`) to reassign failed jobs to healthy workers after 3 retry attempts.

### D. Real-Time Processing Telemetry
- **Pub/Sub Progress Updates**: Redis Pub/Sub channels broadcasting transcoding progress percentages (`{"job_id": "abc", "percent": 75}`) to connected frontend WebSocket clients.

---

## 3. Recommended Project Architecture & Module Hierarchy

```
capstone-media-pipeline/
├── Cargo.toml
├── docker-compose.yml            # Local Redis & MinIO infrastructure
├── src/
│   ├── main.rs                   # Pipeline orchestration and worker spawner
│   ├── config.rs                 # S3 credentials, MinIO endpoints, Redis URLs
│   ├── storage/
│   │   ├── mod.rs
│   │   ├── s3_client.rs          # aws-sdk-s3 client initialization and retry policy
│   │   ├── multipart.rs          # 5GB+ chunked parallel upload manager
│   │   └── presign.rs            # Presigned PUT and GET URL generator
│   ├── cache/
│   │   ├── mod.rs
│   │   ├── redis_pool.rs         # Redis multiplexed connection manager
│   │   └── xfetch.rs             # Probabilistic early cache refresh implementation
│   ├── queue/
│   │   ├── mod.rs
│   │   ├── producer.rs           # Redis Streams XADD job publisher
│   │   ├── consumer.rs           # XREADGROUP worker pool loop
│   │   └── dead_letter.rs        # Stalled job reclaimer (XPENDING / XCLAIM)
│   ├── processor/
│   │   ├── mod.rs
│   │   ├── image_resizer.rs      # Fast image manipulation and format converter
│   │   └── video_transcoder.rs   # Video frame extraction and thumbnail pipeline
│   └── api/
│       ├── mod.rs
│       ├── upload_handler.rs     # Initiation of upload sessions
│       └── media_handler.rs      # Cached metadata queries and presigned downloads
└── tests/
    ├── s3_integration_test.rs    # Direct MinIO bucket upload/download verification
    ├── redis_stream_test.rs      # Worker consumer group and failover test
    └── xfetch_cache_test.rs      # High-concurrency stampede resistance test
```

---

## 4. Distributed Queue Protocol (`stream:media:jobs`)

```json
{
  "job_id": "job_01J6XYZ987",
  "tenant_id": "tenant_1001",
  "raw_s3_key": "raw/uploads/2026/09/video_input.mp4",
  "output_prefix": "processed/2026/09/video_input/",
  "operations": ["generate_thumbnails", "transcode_1080p", "transcode_720p"],
  "created_at": 1756982400000,
  "retry_count": 0
}
```

---

## 5. Verification & Failure-Recovery Targets

| Scenario | Condition | Pass Criteria |
| :--- | :--- | :--- |
| **Worker Crash During Transcoding** | Kill worker process mid-execution while holding an unacknowledged job | Monitor detects idle job via `XPENDING`, claims it via `XCLAIM`, and secondary worker finishes S3 upload |
| **Cache Stampede (1,000 req/sec)** | 1,000 concurrent threads querying a single hot media metadata key during expiration | Exactly 1 background thread refreshes DB/S3; 999 requests served from Redis cache with zero latency spike |
| **Multipart Large File Upload** | 2GB file uploaded via 10MB chunks with simulated 20% network packet drop | Chunk retry logic succeeds; S3 ETag matches local file SHA-256 hash |
