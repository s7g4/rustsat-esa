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

#[entry]
fn main() -> ! {
    hprintln!("Starting QEMU Automated Tests...");

    let mut passed = true;

    passed &= test_fec_clean_encode_decode();
    passed &= test_fec_single_bit_flip_recovery();

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
