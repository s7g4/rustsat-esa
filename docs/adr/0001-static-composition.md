# ADR 0001: Static Composition over Dynamic Dispatch

**Date:** 2026-06-03
**Status:** Accepted

## Context
We need a way to integrate the Physical (SpaceCAN), Network (Mesh), Security (AES-256), and Application (Telemetry) layers. We must choose between:
1. Dynamic Dispatch (`&dyn Trait` or `Box<dyn Trait>`) for dependency injection.
2. Static Composition (Generics and `impl Trait`).

## Decision
We will use **Static Composition (Generics)**.

## Rationale
Space flight software requires absolute determinism. Dynamic dispatch introduces vtable lookups and potential pointer indirection which can degrade execution speed and cache locality on ARM Cortex-M microcontrollers. Static composition forces monomorphization; the compiler resolves all types at compile time, leading to aggressive inlining and zero-cost abstractions. 

## Consequences
- **Positive:** Maximum performance, zero runtime overhead.
- **Negative:** Code syntax can become slightly more complex due to generic trait bounds.
