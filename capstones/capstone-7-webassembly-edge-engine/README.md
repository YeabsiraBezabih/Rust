# Capstone 7: Interactive High-Performance WebAssembly Graphics & Edge Computing Engine

## 1. Project Overview

Capstone 7 focuses on compiling high-performance Rust systems to **WebAssembly (WASM)** for both client-side browser execution (via `wasm-bindgen` and HTML5 Canvas shared linear memory) and serverless edge runtimes (such as Cloudflare Workers or Fastly Compute@Edge using WASI).

You will build a **Real-Time 2D Particle Physics & Fluid Simulation Engine** that renders 100,000 interacting particles at 60 FPS in the browser, alongside a **Cryptographic Hashing Edge Function** that validates signatures and compresses payloads in sub-millisecond edge compute cycles.

```
                                ┌────────────────────────────────────────────────────────┐
                                │       WebAssembly (WASM) Dual Execution Engine         │
                                │                                                        │
Client Browser (60 FPS) ───────►│ [ Browser WebAssembly Runtime (wasm-bindgen) ]        │
(Canvas Particle Simulation)    │           │                                            │
                                │           ├── Step 1: Rust Particle Physics Engine     │
                                │           │           (Spatial Hash Grid & Gravity)    │
                                │           ├── Step 2: Shared Linear Memory Buffer      │
                                │           │           (Direct Pointer to Uint8Clamped) │
                                │           ▼                                            │
                                │ [ HTML5 Canvas 2D / WebGL Direct Pixel Renderer ]      │
                                │                                                        │
                                ├────────────────────────────────────────────────────────┤
                                │                                                        │
Edge Request (Cloudflare/Fastly)►│ [ Edge WASI Runtime (Serverless Worker) ]              │
(Token Validation & Compression)│           │                                            │
                                │           ├── Fast JWT Ed25519 Signature Verification  │
                                │           ├── Streaming Brotli / Gzip Payload Packager │
                                │           └── Zero-Cold-Start Sub-Millisecond Return   │
                                └────────────────────────────────────────────────────────┘
```

---

## 2. Core Functional Requirements

### A. High-Throughput Physics Engine in Linear Memory
- **100,000 Active Particles**: Real-time simulation of velocity, gravity, particle-particle elastic collisions, and boundary bounces at locked 60 FPS.
- **Spatial Hash Grid Partitioning**: $O(N)$ broad-phase collision detection using a 2D spatial grid rather than $O(N^2)$ brute-force comparisons.
- **Direct Linear Memory Sharing**: Rust exposes the raw pointer to its RGBA particle pixel buffer directly to JavaScript (`wasm.memory.buffer`). The browser's JavaScript layer renders the frame with **zero memory copies across the FFI boundary**.

### B. JavaScript Interoperability with `wasm-bindgen`
- **Type-Safe Bindings**: Expose Rust structs, methods, and configurations cleanly to JavaScript.
- **DOM and Canvas Events**: Capture mouse/touch interaction coordinates in JavaScript and pass them directly into the Rust physics simulation to create interactive gravity vortexes and explosion ripples.

### C. Serverless Edge Computing via WASI
- **Edge Deployment**: Compile Rust algorithms to standard `wasm32-wasi` targets for deployment on Cloudflare Workers and Fastly Compute@Edge.
- **Edge Payload Transformer**: High-speed edge computation:
  - Cryptographic token verification.
  - Geo-IP request routing.
  - Streaming payload compression.

---

## 3. Recommended Project Architecture & Module Hierarchy

```
capstone-wasm-edge-engine/
├── Cargo.toml
├── pkg/                          # Output directory for wasm-pack compiled artifacts
├── www/                          # Frontend Web Application (HTML, JS, CSS)
│   ├── index.html                # Canvas container and interactive UI controls
│   ├── index.js                  # Shared memory canvas renderer and event listeners
│   └── package.json              # Webpack / Vite configuration
├── src/
│   ├── lib.rs                    # wasm-bindgen export interface
│   ├── physics/
│   │   ├── mod.rs
│   │   ├── particle.rs           # Particle state struct (position, velocity, color)
│   │   ├── spatial_grid.rs       # O(N) spatial partitioning for collision queries
│   │   └── universe.rs           # Master tick loop, gravity, and frame step
│   ├── graphics/
│   │   ├── mod.rs
│   │   └── frame_buffer.rs       # RGBA pixel buffer aligned for HTML5 ImageData
│   └── edge/
│       ├── mod.rs
│       ├── crypto.rs             # Edge Ed25519 signature validator
│       └── compression.rs        # In-memory streaming compression
└── tests/
    └── wasm_test.rs              # wasm-bindgen-test headless browser verification
```

---

## 4. Shared Linear Memory Zero-Copy Interface

```rust
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct ParticleUniverse {
    width: u32,
    height: u32,
    particles: Vec<Particle>,
    pixel_buffer: Vec<u8>, // Aligned RGBA buffer
}

#[wasm_bindgen]
impl ParticleUniverse {
    pub fn new(width: u32, height: u32, count: usize) -> Self { ... }
    pub fn tick(&mut self) { ... }
    pub fn pixel_buffer_ptr(&self) -> *const u8 {
        self.pixel_buffer.as_ptr()
    }
    pub fn add_vortex(&mut self, x: f32, y: f32, strength: f32) { ... }
}
```

```javascript
// JavaScript Zero-Copy Render Loop
const universe = ParticleUniverse.new(800, 600, 50000);
const memory = wasm.memory;

function renderLoop() {
    universe.tick();
    
    // Create zero-copy typed array view over WASM linear memory
    const pixelPtr = universe.pixel_buffer_ptr();
    const pixels = new Uint8ClampedArray(memory.buffer, pixelPtr, 800 * 600 * 4);
    
    const imageData = new ImageData(pixels, 800, 600);
    ctx.putImageData(imageData, 0, 0);
    
    requestAnimationFrame(renderLoop);
}
```

---

## 5. Performance Verification Targets

| Metric | Target |
| :--- | :--- |
| **Particle Simulation Framerate** | Solid 60 FPS with $\ge 50,000$ particles on standard laptop browser |
| **WASM Binary Size** | $< 150\text{ KB}$ post `wasm-opt -Oz` compression |
| **JS-to-WASM Copy Overhead** | **$0\text{ bytes copied}$** (100% shared linear memory pointers) |
| **Edge Cold-Start Overhead** | $< 1\text{ ms}$ cold start initialization in edge runtimes |
