#![allow(missing_docs)]
use crate::error::RustSatError;

// ----------------------------------------------------------------------------
// Zero-Sized Typestate Definitions
// ----------------------------------------------------------------------------

/// The satellite has just booted or lost connection and is completely isolated.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Unsynchronized;

/// The satellite is actively listening for the swarm beacon to align its clock.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Syncing;

/// The satellite is perfectly aligned with the swarm and authorized for maneuvers.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Synchronized;

// ----------------------------------------------------------------------------
// Swarm Orchestrator
// ----------------------------------------------------------------------------

/// A formalized Swarm Orchestrator that uses the Rust Typestate Pattern to
/// guarantee at compile-time that restricted operations cannot be executed
/// unless the satellite is fully synchronized.
pub struct SwarmOrchestrator<State> {
    state: core::marker::PhantomData<State>,
    pub local_node_id: u32,
    sync_confidence: u8,
}

// ----------------------------------------------------------------------------
// State Transitions
// ----------------------------------------------------------------------------

impl SwarmOrchestrator<Unsynchronized> {
    pub fn new(local_node_id: u32) -> Self {
        #[cfg(feature = "defmt")]
        defmt::info!(
            "Swarm Node {} booted in Unsynchronized state",
            local_node_id
        );
        Self {
            state: core::marker::PhantomData,
            local_node_id,
            sync_confidence: 0,
        }
    }

    /// Transitions the node from Unsynchronized to Syncing state.
    pub fn begin_sync(self) -> SwarmOrchestrator<Syncing> {
        #[cfg(feature = "defmt")]
        defmt::info!(
            "Node {}: Beginning swarm synchronization phase...",
            self.local_node_id
        );
        SwarmOrchestrator {
            state: core::marker::PhantomData,
            local_node_id: self.local_node_id,
            sync_confidence: 0,
        }
    }
}

impl SwarmOrchestrator<Syncing> {
    /// Processes a sync beacon. If confidence threshold is met, transitions to Synchronized.
    pub fn process_sync_beacon(mut self) -> Result<SwarmOrchestrator<Synchronized>, Self> {
        self.sync_confidence += 1;
        #[cfg(feature = "defmt")]
        defmt::info!(
            "Node {}: Received sync beacon. Confidence: {}/3",
            self.local_node_id,
            self.sync_confidence
        );

        if self.sync_confidence >= 3 {
            #[cfg(feature = "defmt")]
            defmt::info!(
                "Node {}: Synchronization threshold met! Node is now Synchronized.",
                self.local_node_id
            );
            Ok(SwarmOrchestrator {
                state: core::marker::PhantomData,
                local_node_id: self.local_node_id,
                sync_confidence: self.sync_confidence,
            })
        } else {
            // Stay in Syncing state
            Err(self)
        }
    }
}

// ----------------------------------------------------------------------------
// Restricted Capabilities (Formal Verification)
// ----------------------------------------------------------------------------

impl<State> SwarmOrchestrator<State> {
    /// Telemetry can be transmitted regardless of the synchronization state.
    pub fn send_telemetry(&self) {
        #[cfg(feature = "defmt")]
        defmt::info!(
            "Node {}: Transmitting general telemetry.",
            self.local_node_id
        );
    }
}

impl SwarmOrchestrator<Synchronized> {
    /// MISSION CRITICAL OPERATION:
    /// This method is mathematically impossible to call unless the orchestrator
    /// is in the `Synchronized` state. If a developer attempts to call this on an
    /// `Unsynchronized` or `Syncing` orchestrator, the Rust compiler will throw a hard error!
    pub fn execute_maneuver(&self, pitch: f32, yaw: f32, roll: f32) -> Result<(), RustSatError> {
        if !pitch.is_finite() || !yaw.is_finite() || !roll.is_finite() {
            return Err(RustSatError::SystemError(
                "Maneuver inputs must be finite numbers",
            ));
        }

        #[cfg(feature = "defmt")]
        defmt::info!(
            "Node {}: EXECUTING MISSION CRITICAL MANEUVER [P: {}, Y: {}, R: {}]",
            self.local_node_id,
            pitch,
            yaw,
            roll
        );
        Ok(())
    }

    /// Gracefully degrades the orchestrator back to Unsynchronized if a solar flare
    /// or other anomaly corrupts the timing sync.
    pub fn demote(self) -> SwarmOrchestrator<Unsynchronized> {
        #[cfg(feature = "defmt")]
        defmt::warn!(
            "Node {}: CRITICAL WARNING: Synchronization lost! Demoting to Unsynchronized.",
            self.local_node_id
        );
        SwarmOrchestrator::new(self.local_node_id)
    }
}
