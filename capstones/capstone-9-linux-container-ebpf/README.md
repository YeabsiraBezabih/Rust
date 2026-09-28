# Capstone 9: Lightweight Linux Container Engine & eBPF Security Monitor

## 1. Project Overview

Capstone 9 is a production-grade, from-scratch **Linux Container Runtime (`contain-rs`)** paired with a real-time **eBPF Kernel Security & Observability Monitor (`ebpf-guard`)** written in pure idiomatic Rust using the native `nix` and `aya` ecosystems.

This capstone mimics the core architecture of industry-defining systems software such as **Docker (`runc`)**, **AWS Firecracker**, and **Cilium / Datadog Cloud Security Agents**. It bridges the gap between low-level Linux OS internals (namespaces, cgroups v2, virtual filesystems) and next-generation in-kernel event-driven telemetry (eBPF, XDP, tracepoints).

```
┌────────────────────────────────────────────────────────────────────────────────────────┐
│              Capstone 9: Container Runtime Engine & eBPF Security Guard                │
├────────────────────────────────────────────────────────────────────────────────────────┤
│                                                                                        │
│  USER SPACE                                                                            │
│  ┌──────────────────────────────────────────┐  ┌────────────────────────────────────┐  │
│  │ Container Runtime CLI (`contain-rs`)     │  │ Security Telemetry (`ebpf-guard`)  │  │
│  │   • Isolated namespaces (PID, Mount...)  │  │   • Async Tokio event loop         │  │
│  │   • cgroups v2 memory & CPU hard caps    │  │   • Polls Aya RingBuffer           │  │
│  │   • Rootfs isolation (`pivot_root`)      │  │   • Real-time policy alerts        │  │
│  │   • `veth` network interface provisioning│  │   • Blocks malicious IPs via Map   │  │
│  └────────────────────┬─────────────────────┘  └─────────────────▲──────────────────┘  │
│                       │ Launches child process                   │ Reads events        │
│                       ▼                                          │ (Lock-free RingBuf) │
│  ┌──────────────────────────────────────────┐                    │                     │
│  │ Sandboxed Container Process              │                    │                     │
│  │ (PID 1 inside sandbox, UID mapped)       │                    │                     │
│  └────────────────────┬─────────────────────┘                    │                     │
│ ──────────────────────┼──────────────────────────────────────────┼──────────────────── │
│  LINUX KERNEL SPACE   │ Triggers Syscalls & Network Packets      │                     │
│                       ▼                                          │                     │
│  ┌───────────────────────────────────────────────────────────────┴──────────────────┐  │
│  │ Aya eBPF Kernel Probes & XDP Filter                                               │  │
│  │                                                                                  │  │
│  │  • Tracepoint Hook: `sys_enter_execve` (Audits spawned binaries & flags)         │  │
│  │  • Tracepoint Hook: `sys_enter_openat` (Detects unauthorized `/etc/shadow` reads)│  │
│  │  • XDP Driver Hook: Ingress packet filter (Sub-microsecond DDoS & IP dropping)   │  │
│  │  • eBPF Hash Map: Dynamic blocklist synchronized from userspace                  │  │
│  └──────────────────────────────────────────────────────────────────────────────────┘  │
└────────────────────────────────────────────────────────────────────────────────────────┘
```

---

## 2. Core Functional Requirements

### A. Container Isolation Engine (`contain-rs`)
1. **Linux Namespace Isolation**:
   - Spawns containerized processes with `CLONE_NEWPID`, `CLONE_NEWNS`, `CLONE_NEWNET`, `CLONE_NEWUTS`, `CLONE_NEWIPC`, `CLONE_NEWUSER`, and `CLONE_NEWCGROUP`.
   - Maps user namespaces so root (`UID 0`) inside the container corresponds to an unprivileged non-root user (`UID 1000`) on the host.
2. **Root Filesystem Pivot (`pivot_root`)**:
   - Isolates the container filesystem using an Alpine or Debian rootfs archive.
   - Safely unmounts the host root filesystem and mounts private virtual filesystems (`/proc`, `/sys`, `/dev`, and `/tmp`).
3. **cgroups v2 Resource Enforcement**:
   - Dynamically provisions `/sys/fs/cgroup/contain_rs_<id>/`.
   - Enforces memory ceiling via `memory.max` (e.g., 256MB).
   - Enforces CPU bandwidth limits via `cpu.max` (e.g., 50,000us per 100,000us period = 50% CPU core).
   - Enforces PID limit via `pids.max` (e.g., 32 max processes) to neutralize fork bomb attacks.
4. **Virtual Network Provisioning**:
   - Allocates a `veth` pair (`veth_host` <-> `veth_cont`).
   - Moves `veth_cont` into the container's network namespace and configures static IP routing.

### B. eBPF Kernel Security & Observability Monitor (`ebpf-guard`)
1. **Syscall Interception via Tracepoints**:
   - Attaches eBPF probes to `sys_enter_execve` to log every process execution inside the container with arguments, PID, and timestamp.
   - Attaches probes to `sys_enter_openat` to detect attempts to access sensitive files (e.g. `/etc/shadow`, `/root/.ssh`).
2. **Zero-Copy Event Pipeline**:
   - Streams kernel telemetry to user-space over an eBPF `RingBuffer` with zero allocations and sub-microsecond latency.
3. **XDP Network Ingress Firewall**:
   - Runs an eBPF XDP program at the network interface driver layer.
   - Evaluates incoming packets against an `EbpfHashMap` IP blocklist; drops malicious traffic before the kernel network stack allocates memory.
4. **Security Alert Rule Engine**:
   - User-space daemon evaluates incoming kernel events against security policies (e.g., "Alert if any container spawns `/bin/sh` or attempts privilege escalation").

---

## 3. Systems Programming & Reliability Standards

1. **Deterministic Cleanup & RAII Guards**:
   - Ensure that container exits (or crashes) cleanly destroy cgroup directories, unmount temporary overlay filesystems, and detach eBPF hooks without leaking host kernel resources.
2. **Zero Unwraps & Robust Syscall Error Handling**:
   - All `nix` and `libc` system calls must convert `nix::errno::Errno` into structured Rust errors with actionable diagnostics.
3. **Async Event Processing with Tokio**:
   - The user-space security monitor must process thousands of kernel events per second asynchronously without blocking or dropping telemetry.

---

## 4. Suggested Directory Structure

```
capstone-9-linux-container-ebpf/
├── Cargo.toml
├── contain-rs/                     # Container Runtime CLI & Daemon
│   ├── src/
│   │   ├── main.rs                 # CLI Argument Parser (clap)
│   │   ├── namespaces.rs           # clone, unshare, and UID/GID mapping
│   │   ├── rootfs.rs               # pivot_root, mount /proc, /dev, /sys
│   │   ├── cgroups.rs              # cgroups v2 controller (memory, cpu, pids)
│   │   ├── network.rs              # veth pair & network namespace configuration
│   │   └── process.rs              # PID lifecycle, waitpid, and signal handling
│   └── Cargo.toml
├── ebpf-guard/                     # Userspace eBPF Loader & Telemetry Engine
│   ├── src/
│   │   ├── main.rs                 # Tokio event loop & alert monitor
│   │   ├── loader.rs               # Aya BPF program loader & map manager
│   │   ├── policy.rs               # Security policy evaluation engine
│   │   └── events.rs               # RingBuffer deserializer & logger
│   └── Cargo.toml
├── ebpf-guard-ebpf/                # Pure Rust Kernel eBPF Probes (no_std)
│   ├── src/
│   │   ├── main.rs                 # eBPF tracepoint & XDP entrypoints
│   │   ├── vmlinux.rs              # Kernel context bindings
│   │   └── maps.rs                 # Shared RingBuffer & Hash Maps
│   └── Cargo.toml
├── tests/
│   ├── container_isolation_test.rs # Integration test for PID & Mount isolation
│   ├── cgroup_limits_test.rs       # Verifies memory & PID cap enforcement
│   └── ebpf_auditing_test.rs       # End-to-end security alert verification
└── README.md
```

---

## 5. Verification & Acceptance Criteria

To pass Capstone 9, the implementation must satisfy:

1. **Isolation Verification**:
   - Running `contain-rs run --image alpine /bin/ps` displays only PID 1 (isolated from host processes).
   - Running `contain-rs run --image alpine /bin/hostname` displays the container's isolated UTS hostname.
2. **Resource Quota Enforcement**:
   - Attempting a fork bomb (`:(){ :|:& };:`) within the container is instantly stopped by the cgroups `pids.max` limit without affecting the host system.
   - An intentional memory allocation exceeding `memory.max` triggers an immediate container-specific OOM kill.
3. **eBPF Security Telemetry**:
   - Running any command inside the container triggers an instantaneous audit event in `ebpf-guard` with accurate binary path and PID.
   - Attempting to access a restricted file pattern emits a `CRITICAL_SECURITY_ALERT` within 1 millisecond.
4. **XDP Packet Dropping**:
   - Populating the blocked IP map successfully drops 100% of ping/HTTP packets from that IP at the driver level without kernel packet allocation.
