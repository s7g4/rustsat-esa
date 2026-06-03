#![no_std]
#![no_main]

//! Developer Sandbox: RTOS Simulation (RTIC-style)
//! This file is designed for you to learn how the RustSat protocol stack
//! integrates with a Real-Time Operating System like RTIC.
//!
//! You can experiment here by adding new tasks, sending mock telemetry,
//! or testing bit-flips with the FEC engine!

// Panic handler for bare-metal targets
use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

use rustsat_esa::error::RustSatError;
use rustsat_esa::rtos_bindings::{HardwareInterrupt, HardwareTimer};
use rustsat_esa::RustSatProtocol;

// Mock hardware mocks for learning
pub struct MockTimer {
    ticks: u64,
}

impl HardwareTimer for MockTimer {
    fn start(&mut self, _frequency_hz: u32) -> Result<(), RustSatError> {
        Ok(())
    }
    fn current_ticks(&self) -> u64 {
        self.ticks
    }
    fn clear_interrupt(&mut self) {
        self.ticks += 1;
    }
}

pub struct MockIrq {
    pending: bool,
}

impl HardwareInterrupt for MockIrq {
    fn enable(&mut self) {}
    fn disable(&mut self) {}
    fn is_pending(&self) -> bool {
        self.pending
    }
    fn clear_pending(&mut self) {
        self.pending = false;
    }
}

// ------------------------------------------------------------------
// RTOS Application (Simulated)
// ------------------------------------------------------------------

/// Simulated entry point for an embedded device
#[no_mangle]
pub extern "C" fn main() -> ! {
    // 1. Initialize Hardware Peripherals
    let timer = MockTimer { ticks: 0 };
    let irq = MockIrq { pending: false };

    let mut rtos = rustsat_esa::rtos_bindings::RtosManager::new(timer, irq);
    let _ = rtos.init_hardware();

    // 2. Initialize the RustSat Stack
    let mut stack = RustSatProtocol::new();
    let _ = stack.initialize();

    // 3. RTOS Infinite Loop (Simulating hardware interrupts)
    loop {
        // [SIMULATION] Pretend a hardware timer interrupt just fired
        rtos.systick.clear_interrupt();

        // At exactly 1000 ticks, let's trigger a Telemetry broadcast!
        if rtos.systick.current_ticks() % 1000 == 0 {
            let mock_payload = [0xDE, 0xAD, 0xBE, 0xEF];
            // Since this payload is sent through `stack.send_message`,
            // the SpaceCAN layer will automatically apply FEC (Hamming 8,4) encoding!
            let _ = stack.send_message(1, &mock_payload);
        }

        // [SIMULATION] Pretend a radio interrupt occurred
        if rtos.radio_rx_irq.is_pending() {
            rtos.radio_rx_irq.clear_pending();

            // Process incoming packets
            // SpaceCAN will automatically run the Hamming FEC decoder and correct
            // any radiation-induced bit flips before returning the payload!
            if let Ok(Some(_payload)) = stack.receive_message() {
                // Handle payload...
            }
        }
    }
}
