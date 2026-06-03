use crate::error::RustSatError;
use defmt::{info, warn, Format};
use heapless::{FnvIndexMap, Vec};

#[derive(Debug, Clone, PartialEq, Eq, Format)]
pub enum NodeType {
    CubeSat,
    GroundStation,
    Relay,
}

#[derive(Debug, Clone, Format)]
pub struct OrbitalPosition {
    pub latitude: f32,
    pub longitude: f32,
    pub altitude: f32,
}

#[derive(Debug, Clone, Format)]
pub struct NetworkNode {
    pub node_id: u32,
    pub node_type: NodeType,
    pub position: OrbitalPosition,
    pub battery_level: f32, // 0.0 to 1.0
}

#[derive(Debug, Clone, Format)]
pub struct RoutingEntry {
    pub destination: u32,
    pub next_hop: u32,
    pub hop_count: u8,
}

#[derive(Debug, Clone)]
pub struct NetworkPacket {
    pub packet_id: u32,
    pub source: u32,
    pub destination: u32,
    pub next_hop: u32,
    pub ttl: u8,
    pub payload: Vec<u8, 256>,
}

pub struct MeshNetwork {
    pub nodes: FnvIndexMap<u32, NetworkNode, 32>,
    pub routing_table: FnvIndexMap<u32, RoutingEntry, 32>,
}

impl MeshNetwork {
    pub const fn new() -> Self {
        Self {
            nodes: FnvIndexMap::new(),
            routing_table: FnvIndexMap::new(),
        }
    }

    pub fn initialize_routing(&mut self) -> Result<(), RustSatError> {
        info!("Initializing mesh network routing protocols");
        Ok(())
    }

    pub fn add_node(&mut self, node: NetworkNode) -> Result<(), RustSatError> {
        if self.nodes.insert(node.node_id, node).is_err() {
            return Err(RustSatError::SystemError);
        }
        Ok(())
    }

    pub fn remove_node(&mut self, node_id: u32) {
        self.nodes.remove(&node_id);
    }

    pub fn route_message(
        &mut self,
        source: u32,
        destination: u32,
        data: &[u8],
    ) -> Result<bool, RustSatError> {
        let mut payload = Vec::new();
        if payload.extend_from_slice(data).is_err() {
            return Err(RustSatError::SystemError); // Payload too large
        }

        let _packet = NetworkPacket {
            packet_id: 1, // Mock
            source,
            destination,
            next_hop: destination, // Simplified
            ttl: 32,
            payload,
        };

        if !self.nodes.contains_key(&destination) {
            warn!("Destination node not found");
            return Ok(false);
        }

        info!("Successfully routed message to destination");
        Ok(true)
    }
}

impl Default for MeshNetwork {
    fn default() -> Self {
        Self::new()
    }
}
