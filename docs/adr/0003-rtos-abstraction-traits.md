# ADR 0003: Hardware-Agnostic RTOS Abstraction

## Status
Accepted

## Context
Flight software is rarely deployed as a naked super-loop. Depending on the mission requirements, the payload might run on FreeRTOS, Zephyr, RTIC (Real-Time Interrupt-driven Concurrency), or a custom scheduler. Hardcoding the protocol stack to a specific RTOS severely limits its reusability across missions.

## Decision
We implemented `HardwareTimer` and `HardwareInterrupt` traits in `src/rtos_bindings.rs`. The `rustsat-esa` protocol stack now accepts an injected `RtosManager<T, I>` orchestrator.

## Rationale
By relying on traits (Dependency Inversion), the core library does not need to know *how* a tick is generated or *how* a CAN bus RX interrupt is triggered. The mission-specific firmware (the final binary crate) will implement these traits using its chosen RTOS HAL.

## Consequences
- **Positive:** The library is 100% decoupled from the execution environment. We can write simulated implementations (like `MockTimer`) to test the entire stack natively or in QEMU.
- **Negative:** The end-user must write a thin boilerplate layer to wire up their specific RTOS interrupts to the library traits.
