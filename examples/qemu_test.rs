#![no_std]
#![no_main]

// Import the global logger required by defmt, even though we can't see RTT easily in QEMU
use defmt_rtt as _;

// Panic handler that uses semihosting to exit QEMU with a non-zero code and print to console
use panic_semihosting as _;

use cortex_m_rt::entry;
use cortex_m_semihosting::{debug, hprintln};

// Unused import removed
use rustsat_esa::protocol::fec::Hamming84;
use rustsat_esa::protocol::network::{MeshNetwork, RouteAction, RoutingEntry};
use rustsat_esa::protocol::spacecan::{FramePriority, SpaceCANFrame};

#[entry]
fn main() -> ! {
    hprintln!("Starting QEMU Automated Tests...");

    let mut passed = true;

    passed &= test_fec_clean_encode_decode();
    passed &= test_fec_single_bit_flip_recovery();
    passed &= test_zero_copy_routing();

    if passed {
        hprintln!("SUCCESS: All integration tests passed!");
        debug::exit(debug::EXIT_SUCCESS);
    } else {
        hprintln!("FAILURE: One or more integration tests failed!");
        debug::exit(debug::EXIT_FAILURE);
    }

    loop {}
}

fn test_fec_clean_encode_decode() -> bool {
    hprintln!("Running test_fec_clean_encode_decode...");
    let original: [u8; 4] = [0x11, 0x22, 0x33, 0x44];
    if let Ok(encoded) = Hamming84::encode::<8>(&original) {
        if let Ok(decoded) = Hamming84::decode::<4>(&encoded) {
            return decoded.as_slice() == original.as_slice();
        }
    }
    false
}

fn test_fec_single_bit_flip_recovery() -> bool {
    hprintln!("Running test_fec_single_bit_flip_recovery...");
    let original: [u8; 2] = [0xAA, 0xBB];

    if let Ok(mut encoded) = Hamming84::encode::<4>(&original) {
        // Inject radiation bit-flip into the encoded payload
        encoded[0] ^= 0b0000_1000;

        if let Ok(decoded) = Hamming84::decode::<2>(&encoded) {
            return decoded.as_slice() == original.as_slice();
        }
    }
    false
}

fn test_zero_copy_routing() -> bool {
    hprintln!("Running test_zero_copy_routing...");

    let mut network = MeshNetwork::new();

    // Setup routing table: Dest 42 is reachable via Next-Hop 99
    let _ = network.routing_table.insert(
        42,
        RoutingEntry {
            destination: 42,
            next_hop: 99,
            hop_count: 1,
        },
    );

    // Create a mock MAC frame payload
    let mut payload = heapless::Vec::<u8, 256>::new();
    let _ = payload.extend_from_slice(&[
        0, 0, 0, 1, // Packet ID: 1
        0, 0, 0, 10, // Source: 10
        0, 0, 0, 42, // Destination: 42
        0, 0, 0, 10, // Next-Hop: 10 (needs to be updated)
        5,  // TTL: 5
        0xAA, 0xBB, // Data
    ]);

    let mut frame = SpaceCANFrame::new(1, payload, FramePriority::Normal, 0, 0);

    // We are Node 10 routing a packet to Node 42
    if let Ok(action) = network.route_in_place(10, &mut frame) {
        if action != RouteAction::Forward {
            return false;
        }

        // Verify TTL was decremented (5 -> 4)
        if frame.data[16] != 4 {
            return false;
        }

        // Verify Next-Hop was rewritten in-place (10 -> 99)
        let next_hop = u32::from_be_bytes([
            frame.data[12],
            frame.data[13],
            frame.data[14],
            frame.data[15],
        ]);
        if next_hop != 99 {
            return false;
        }

        return true;
    }

    false
}
