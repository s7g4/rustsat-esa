#![no_std]

pub mod config;
pub mod cubesat;
pub mod error;
pub mod metrics;
pub mod protocol;
pub mod rtos_bindings;
pub mod telemetry;

use error::RustSatError;
use heapless::Vec;
use protocol::network::MeshNetwork;

/// Main RustSat protocol stack integrating all core embedded layers
pub struct RustSatProtocol {
    pub physical_layer: protocol::spacecan::SpaceCANAdapter,
    pub network_layer: MeshNetwork,
    pub telemetry: telemetry::TelemetryProcessor,
}

impl RustSatProtocol {
    pub fn new() -> Self {
        Self {
            physical_layer: protocol::spacecan::SpaceCANAdapter::new(),
            network_layer: MeshNetwork::new(),
            telemetry: telemetry::TelemetryProcessor::new(),
        }
    }

    pub fn initialize(&mut self) -> Result<(), RustSatError> {
        self.network_layer.initialize_routing()?;
        self.telemetry.initialize()?;
        Ok(())
    }

    pub fn send_message(&mut self, destination: u32, payload: &[u8]) -> Result<(), RustSatError> {
        let mut heapless_payload = Vec::new();

        // Zero-Copy: Pre-pend network header directly into the MAC payload buffer
        let _ = heapless_payload.extend_from_slice(&1u32.to_be_bytes()); // packet_id (mock)
        let _ = heapless_payload.extend_from_slice(&0u32.to_be_bytes()); // source (node 0)
        let _ = heapless_payload.extend_from_slice(&destination.to_be_bytes()); // dest
        let _ = heapless_payload.extend_from_slice(&0u32.to_be_bytes()); // next_hop (resolved by router)
        let _ = heapless_payload.push(32); // ttl

        if heapless_payload.extend_from_slice(payload).is_err() {
            return Err(RustSatError::SystemError); // Payload too large for SpaceCAN frame
        }

        let mut frame = protocol::spacecan::SpaceCANFrame::new(
            destination,
            heapless_payload,
            protocol::spacecan::FramePriority::Normal,
            0,
            0,
        );

        match self.network_layer.route_in_place(0, &mut frame)? {
            protocol::network::RouteAction::Forward => {
                self.physical_layer.transmit(&frame)?;
                self.telemetry.log_transmission(destination, payload.len());
            }
            protocol::network::RouteAction::Consume => {
                // Sent to self
            }
            protocol::network::RouteAction::Drop => {
                // Dropped by router (e.g. no route)
            }
        }

        Ok(())
    }

    pub fn receive_message(&mut self) -> Result<Option<Vec<u8, 256>>, RustSatError> {
        if let Some(raw_data) = self.physical_layer.receive()? {
            self.telemetry.log_reception(raw_data.len());

            let mut result = Vec::new();
            if result.extend_from_slice(&raw_data).is_err() {
                return Err(RustSatError::SystemError);
            }
            Ok(Some(result))
        } else {
            Ok(None)
        }
    }
}

impl Default for RustSatProtocol {
    fn default() -> Self {
        Self::new()
    }
}
