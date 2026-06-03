# rustsat-esa

A `#![no_std]` communication protocol stack for CubeSats targeting ARM Cortex-M architecture (`thumbv7em-none-eabihf`).

## Overview

This repository contains the core flight software communication stack. The stack relies entirely on static memory allocation (`heapless`) and is designed to operate without `std`, `String`, or `Vec`.

Modules:
- `spacecan`: Physical layer framing and transmission channel selection.
- `network`: Mesh routing algorithm (In progress).
- `security`: AES-256 encryption.
- `telemetry`: Metrics and telemetry packet construction.

## Building

```bash
cargo check --target thumbv7em-none-eabihf --no-default-features
```
