# Developer Log

## Contract
Every entry must contain: Goal, Work Completed, Problems Encountered, Root Cause Analysis, Fix, Lessons Learned, Metrics, Next Steps.

---

### [2026-06-03] Phase 1 Initialization: Lean Documentation & Architecture Strategy
**Goal:** Establish the repository baseline before initiating the `#![no_std]` core overhaul.
**Work Completed:** Initialized `README.md`, `DEVLOG.md`, `.github/workflows/ci.yml`, and `docs/adr/0001-static-composition.md`.
**Problems Encountered:** Waterfall-style documentation mapping generated unnecessary overhead for an agile embedded project.
**Root Cause Analysis:** Attempting to define Observability, Testing, and Architecture entirely upfront before proving out the `#![no_std]` constraints leads to brittle documentation that inevitably desyncs with code reality.
**Fix:** Adopted a senior-level, lean documentation approach. We rely on the `DEVLOG.md` to capture daily technical decisions. `ARCHITECTURE.md` will be finalized *after* the core data paths are proven.
**Lessons Learned:** Keep documentation lean and closely coupled to the actual code changes.
**Metrics:** 0 `std` dependencies planned for the core.
**Next Steps:** Begin Code Refactoring Phase 1 (Core Overhaul).

### [2026-06-03] Feature Implementation: Reliability, Error Correction & RTOS Abstraction
**Goal:** Introduce hardware-agnostic RTOS bindings and protect telemetry against single-event upsets (SEUs) using Forward Error Correction.
**Work Completed:** 
- Implemented `HardwareTimer` and `HardwareInterrupt` traits in `rtos_bindings.rs`.
- Created a simulated RTOS environment in `examples/rtic_demo.rs` for HIL (Hardware-In-The-Loop) testing.
- Implemented SECDED Hamming(8,4) error correction in `src/protocol/fec.rs`.
- Wrote pure-logic test suite for FEC in `tests/phase2_suite.rs`.
**Problems Encountered:** Host-side `cargo test` failures due to `defmt` linkage errors. `defmt` requires an ARM linker script (`defmt.x`) that doesn't exist on MSVC/Windows.
**Root Cause Analysis:** Testing embedded code that relies on `defmt` directly on the host machine attempts to link microcontroller-specific sections into a native executable.
**Fix:** Segregated purely mathematical logic (FEC) from the `defmt`-dependent network layers. Built a dedicated HIL test suite (`tests/phase2_suite.rs`) designed to be cross-compiled to `thumbv7em` rather than run on the host.
**Lessons Learned:** Never mix hardware-dependent macros (`defmt`) into mathematical or algorithmic modules (`fec.rs`). Pure logic should remain entirely generic and testable on the host.
**Metrics:** Achieved 100% compilation on `thumbv7em-none-eabihf` with 0 Clippy warnings. Hamming(8,4) introduces a 100% byte overhead (1:2 ratio) but guarantees single-bit recovery per nibble.
**Next Steps:** Implement Zero-Copy Mesh Routing algorithms.
