# RustSat-ESA: Commercial-Grade Space Protocol Stack

![Rust](https://img.shields.io/badge/rust-1.70%2B-blue.svg)
![License](https://img.shields.io/badge/license-MIT%20%2F%20Commercial-green.svg)
![Target](https://img.shields.io/badge/target-thumbv7em--none--eabihf-orange.svg)
![Status](https://img.shields.io/badge/status-Active%20Development-success.svg)

RustSat-ESA is a hard-real-time, memory-safe, `#![no_std]` communication protocol stack designed for CubeSats and Satellite Constellations. It serves as a comprehensive "Mission-in-a-Box", providing SpaceCAN framing, dynamic Mesh Routing, AES-256 encrypted Telemetry, and ECSS PUS compliance—all with **zero dynamic memory allocation**.

## The Problem
Legacy space communication software heavily relies on C/C++, making it susceptible to memory leaks, buffer overflows, and segmentation faults. For million-dollar hardware orbiting in highly radiative environments, memory safety vulnerabilities are unacceptable.

## The Solution
RustSat-ESA leverages the Rust compiler's strict guarantees to provide mathematical certainty against memory corruption. By combining `#![no_std]` constraints with static memory management (`heapless`), this stack provides commercial NewSpace companies and Defense Agencies with a drop-in, zero-cost networking and security layer.

## Core Architecture
- **Zero Allocations:** No `std`, no `Vec`, no `String`. Statically bounded.
- **Static Composition:** High-performance, vtable-free module integration.
- **Real-Time Integration:** Designed specifically to run on frameworks like RTIC (Real-Time Interrupt-driven Concurrency) on ARM Cortex-M architecture.
- **Defmt Observability:** Zero-allocation deferred formatting for real-time host-side logging.

## Dual Licensing
This project operates under a dual-license model:
1. **Open Source:** Available under the GPLv3 license for academic and personal research.
2. **Commercial License:** Available for proprietary, closed-source integration in commercial satellite buses.

## Documentation
- [Architecture](ARCHITECTURE.md)
- [Engineering Roadmap](docs/ROADMAP.md)
- [Observability Strategy](docs/OBSERVABILITY.md)
- [Testing Strategy](docs/TESTING.md)
- [Developer Log](DEVLOG.md)

## Building
To verify the core library against embedded constraints:
```bash
cargo check --target thumbv7em-none-eabihf --no-default-features
```