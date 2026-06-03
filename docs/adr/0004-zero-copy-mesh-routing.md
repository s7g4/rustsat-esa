# ADR 0004: Zero-Copy Mesh Routing Architecture

## Context
The Satellite Mesh Network involves dynamic routing of packets between CubeSats, Relays, and Ground Stations. Initially, `network.rs` utilized a `NetworkPacket` structure that maintained its own `heapless::Vec<u8, 256>` payload array.
Whenever the physical layer (SpaceCAN) received a frame destined for another node, the networking layer was required to copy the 256 bytes out of the MAC frame, into the `NetworkPacket`, process the TTL and hop data, and then copy it back into a new MAC frame for re-transmission. This resulted in dual-allocation and severe memory bandwidth taxation on a 48MHz Cortex-M4 core.

## Decision
We decided to completely deprecate payload-owning network structs. 
Instead, we implemented `NetworkHeaderView<'a>`, a zero-cost abstraction that directly maps over a mutable slice of the pre-allocated MAC payload (`&'a mut [u8]`).

- All routing headers (Packet ID, Source, Dest, Next-Hop, TTL) are packed into the first 17 bytes of the MAC payload.
- The router (`MeshNetwork::route_in_place()`) takes a `&mut SpaceCANFrame` and manipulates the TTL and Next-Hop pointers strictly in-place.
- The routing table utilizes an O(1) Static `FnvIndexMap` to determine routes, as satellite orbital trajectories and ground-station passes are highly deterministic and can be pre-loaded, removing the need for dynamic bandwidth-heavy flood protocols (like AODV).

## Consequences

**Positive:**
- Zero memory allocations during intermediate hops.
- O(1) latency overhead per routed packet.
- Significantly reduced power consumption (fewer CPU cycles spent copying data).
- Reduced stack depth, preventing potential stack overflows on deeply nested function calls.

**Negative:**
- Tight coupling of binary layout protocols; the application layer must respect the 17-byte offset required by the Network protocol before injecting the actual telemetry payload.
