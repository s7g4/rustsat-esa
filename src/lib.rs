#![no_std]
#![deny(unsafe_code)]

pub mod config;
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
    pub local_node_id: u32,
    pub packet_sequence: u32,
    pub physical_layer: protocol::spacecan::SpaceCANAdapter,
    pub network_layer: MeshNetwork,
    pub telemetry: telemetry::TelemetryProcessor,
}

impl RustSatProtocol {
    pub fn new(local_node_id: u32) -> Self {
        Self {
            local_node_id,
            packet_sequence: 0,
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

        self.packet_sequence = self.packet_sequence.wrapping_add(1);

        // Zero-Copy: Pre-pend network header directly into the MAC payload buffer
        heapless_payload
            .extend_from_slice(&self.packet_sequence.to_be_bytes())
            .map_err(|_| RustSatError::SystemError("Buffer overflow writing packet_id"))?;
        heapless_payload
            .extend_from_slice(&self.local_node_id.to_be_bytes())
            .map_err(|_| RustSatError::SystemError("Buffer overflow writing source node"))?;
        heapless_payload
            .extend_from_slice(&destination.to_be_bytes())
            .map_err(|_| RustSatError::SystemError("Buffer overflow writing destination"))?;
        heapless_payload
            .extend_from_slice(&0u32.to_be_bytes())
            .map_err(|_| RustSatError::SystemError("Buffer overflow writing next_hop"))?; // resolved by router
        heapless_payload
            .push(32)
            .map_err(|_| RustSatError::SystemError("Buffer overflow writing ttl"))?;

        if heapless_payload.extend_from_slice(payload).is_err() {
            return Err(RustSatError::SystemError(
                "Payload too large for SpaceCAN frame",
            ));
        }

        let mut frame = protocol::spacecan::SpaceCANFrame::new(
            destination,
            heapless_payload,
            protocol::spacecan::FramePriority::Normal,
            0,
            0,
        )?;

        match self
            .network_layer
            .route_in_place(self.local_node_id, &mut frame)?
        {
            protocol::network::RouteAction::Forward => {
                self.physical_layer.transmit(&frame)?;
                self.telemetry.log_transmission(destination, payload.len());
            }
            protocol::network::RouteAction::Consume => {
                // Sent to self
            }
            protocol::network::RouteAction::Drop => {
                // Dropped by router (e.g. no route)
                return Err(RustSatError::NetworkError("Packet dropped by mesh router"));
            }
        }

        Ok(())
    }

    pub fn receive_message(&mut self) -> Result<Option<Vec<u8, 256>>, RustSatError> {
        if let Some(raw_data) = self.physical_layer.receive()? {
            self.telemetry.log_reception(raw_data.len());

            let mut result = Vec::new();
            if result.extend_from_slice(&raw_data).is_err() {
                return Err(RustSatError::SystemError("Receive payload too large"));
            }
            Ok(Some(result))
        } else {
            Ok(None)
        }
    }
}
