#![no_std]

pub mod error;
pub mod config;
pub mod cubesat;
pub mod metrics;
pub mod protocol;
pub mod telemetry;

use protocol::network::MeshNetwork;
use protocol::spacecan::SpaceCANFrame;
use error::RustSatError;
use heapless::Vec;

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
        if heapless_payload.extend_from_slice(payload).is_err() {
            return Err(RustSatError::SystemError); // Payload too large for SpaceCAN frame
        }

        let _routed = self.network_layer.route_message(0, destination, payload)?;

        let frame = protocol::spacecan::SpaceCANFrame::new(
            destination,
            heapless_payload,
            protocol::spacecan::FramePriority::Normal,
            0,
            0,
        );

        self.physical_layer.transmit(&frame)?;
        self.telemetry.log_transmission(destination, payload.len());

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
