#![allow(missing_docs)]

use crate::error::RustSatError;

/// Trait representing a hardware-level timer interrupt driver.
/// Real-Time Operating Systems (RTOS) like RTIC or FreeRTOS will implement this
/// to drive the protocol stack deterministically.
pub trait HardwareTimer {
    /// Start the hardware timer with a specific frequency (in Hz).
    fn start(&mut self, frequency_hz: u32) -> Result<(), RustSatError>;

    /// Get the current monotonic hardware tick count.
    fn current_ticks(&self) -> u64;

    /// Clear the interrupt pending flag (must be called in the ISR).
    fn clear_interrupt(&mut self);
}

/// Trait representing a hardware-level external interrupt (e.g., GPIO pin for CAN RX).
pub trait HardwareInterrupt {
    /// Enable the external interrupt source.
    fn enable(&mut self);

    /// Disable the external interrupt source.
    fn disable(&mut self);

    /// Check if the interrupt flag is currently set.
    fn is_pending(&self) -> bool;

    /// Clear the hardware interrupt flag.
    fn clear_pending(&mut self);
}

/// A generic RTOS bindings manager.
/// This allows the core stack to orchestrate subsystem ticks.
pub struct RtosManager<T: HardwareTimer, I: HardwareInterrupt> {
    pub systick: T,
    pub radio_rx_irq: I,
}

impl<T: HardwareTimer, I: HardwareInterrupt> RtosManager<T, I> {
    pub fn new(systick: T, radio_rx_irq: I) -> Self {
        Self {
            systick,
            radio_rx_irq,
        }
    }

    /// Initializes all interrupts and timers for the mission.
    pub fn init_hardware(&mut self) -> Result<(), RustSatError> {
        self.radio_rx_irq.enable();
        self.systick.start(1000)?; // 1kHz Base Tick
        Ok(())
    }
}
