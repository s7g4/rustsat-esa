# ADR 0002: Forward Error Correction (FEC)

## Status
Accepted

## Context
In low-earth orbit (LEO) and deep space, radiation induces Single Event Upsets (SEUs) which cause arbitrary bit-flips in memory and communication buses. The physical CAN bus handles some CRC checks, but critical software payloads require application-layer recovery to prevent dropped telemetry packets.

## Decision
We chose to implement a **SECDED (Single Error Correction, Double Error Detection) Hamming(8,4)** encoding scheme in `src/protocol/fec.rs`.

## Rationale
- **Why not Reed-Solomon?** Standard Reed-Solomon algorithms require significant computational overhead and polynomial arithmetic, which drains power on a Cortex-M4.
- **Why Hamming(8,4)?** It allows encoding 4 bits of data into an 8-bit byte. This is incredibly fast to compute using bitwise shifts and XOR operations.
- While the 100% data overhead (rate 1/2) is steep, we only apply this encoding to `Emergency` and `High` priority `SpaceCAN` frames, leaving `Normal` and `Low` priority telemetry unpenalized.

## Consequences
- **Positive:** Critical telemetry is guaranteed to survive a single bit-flip per nibble (2 bit-flips per byte if distributed correctly) without packet loss.
- **Negative:** High priority payload bandwidth is effectively halved at the physical layer.
