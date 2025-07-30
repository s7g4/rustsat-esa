// RustSat-ESA: SpaceCAN-Compatible CubeSat Communication Stack
// A production-ready communication protocol stack for CubeSats

pub mod protocol;
pub mod cubesat;
pub mod simulation;
pub mod ground_station;
pub mod security;
pub mod telemetry;
pub mod web;
pub mod metrics;
pub mod config;

use protocol::spacecan::SpaceCANFrame;
use protocol::network::MeshNetwork;
use cubesat::CubeSatProtocol;
use simulation::SpaceSimulator;

/// Main RustSat protocol stack integrating all layers
pub struct RustSatProtocol {
    pub physical_layer: protocol::spacecan::SpaceCANAdapter,
    pub network_layer: MeshNetwork,
    pub application_layer: cubesat::MissionControl,
    pub security_layer: security::CryptoModule,
    pub telemetry: telemetry::TelemetryProcessor,
}

impl RustSatProtocol {
    /// Create a new RustSat protocol stack instance
    pub fn new() -> Self {
        Self {
            physical_layer: protocol::spacecan::SpaceCANAdapter::new(),
            network_layer: MeshNetwork::new(),
            application_layer: cubesat::MissionControl::new(),
            security_layer: security::CryptoModule::new(),
            telemetry: telemetry::TelemetryProcessor::new(),
        }
    }

    /// Initialize the protocol stack for a CubeSat mission
    pub fn initialize_mission(&mut self, mission_config: cubesat::MissionConfig) -> Result<(), String> {
        // Configure application layer (MissionControl manages satellites, not missions directly)
        // Create a CubeSat with the mission config instead
        let mut cubesat = cubesat::CubeSatProtocol::new(1);
        cubesat.configure_mission(mission_config)?;
        self.application_layer.add_satellite(cubesat);
        self.network_layer.initialize_routing()?;
        self.security_layer.initialize_keys()?;
        Ok(())
    }

    /// Send a message through the complete protocol stack
    pub fn send_message(&mut self, destination: u32, payload: &[u8]) -> Result<(), String> {
        // Encrypt payload
        let encrypted_payload = self.security_layer.encrypt(payload)?;
        
        // Route through network layer
        let _routed = self.network_layer.route_message(0, destination, &encrypted_payload)?;
        
        // Create SpaceCAN frame for transmission
        let frame = protocol::spacecan::SpaceCANFrame::new(
            destination, 
            encrypted_payload, 
            protocol::spacecan::FramePriority::Normal
        );
        
        // Send via physical layer
        self.physical_layer.transmit(&frame)?;
        
        // Log telemetry
        self.telemetry.log_transmission(destination, payload.len());
        
        Ok(())
    }

    /// Receive and process incoming messages
    pub fn receive_message(&mut self) -> Result<Option<Vec<u8>>, String> {
        if let Some(raw_data) = self.physical_layer.receive()? {
            let decrypted = self.security_layer.decrypt(&raw_data)?;
            self.telemetry.log_reception(raw_data.len());
            Ok(Some(decrypted))
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_protocol_initialization() {
        let mut protocol = RustSatProtocol::new();
        let config = cubesat::MissionConfig::default();
        assert!(protocol.initialize_mission(config).is_ok());
    }

    #[test]
    fn test_message_transmission() {
        let mut protocol = RustSatProtocol::new();
        let config = cubesat::MissionConfig::default();
        protocol.initialize_mission(config).unwrap();
        
        let test_payload = b"Hello CubeSat!";
        assert!(protocol.send_message(1, test_payload).is_ok());
    }
}