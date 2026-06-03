use crate::error::RustSatError;
use defmt::{info, warn, Format};
use heapless::FnvIndexMap;

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

use crate::protocol::spacecan::SpaceCANFrame;

#[derive(Debug, Clone, PartialEq, Eq, Format)]
pub enum RouteAction {
    Consume, // The packet has arrived at its final destination (this node)
    Forward, // The packet's TTL and Next-Hop were updated, it should be transmitted
    Drop,    // The packet TTL expired or destination is unknown
}

/// A zero-copy view into a raw network packet payload residing within a MAC frame.
///
/// Binary Layout:
/// - Bytes 0-3: Packet ID (u32)
/// - Bytes 4-7: Source ID (u32)
/// - Bytes 8-11: Destination ID (u32)
/// - Bytes 12-15: Next Hop ID (u32)
/// - Byte 16: TTL (u8)
/// - Bytes 17+: Payload Data
pub struct NetworkHeaderView<'a> {
    buffer: &'a mut [u8],
}

impl<'a> NetworkHeaderView<'a> {
    pub const HEADER_SIZE: usize = 17;

    pub fn new(buffer: &'a mut [u8]) -> Result<Self, RustSatError> {
        if buffer.len() < Self::HEADER_SIZE {
            return Err(RustSatError::InvalidFormat);
        }
        Ok(Self { buffer })
    }

    pub fn packet_id(&self) -> u32 {
        u32::from_be_bytes([
            self.buffer[0],
            self.buffer[1],
            self.buffer[2],
            self.buffer[3],
        ])
    }

    pub fn source(&self) -> u32 {
        u32::from_be_bytes([
            self.buffer[4],
            self.buffer[5],
            self.buffer[6],
            self.buffer[7],
        ])
    }

    pub fn destination(&self) -> u32 {
        u32::from_be_bytes([
            self.buffer[8],
            self.buffer[9],
            self.buffer[10],
            self.buffer[11],
        ])
    }

    pub fn next_hop(&self) -> u32 {
        u32::from_be_bytes([
            self.buffer[12],
            self.buffer[13],
            self.buffer[14],
            self.buffer[15],
        ])
    }

    pub fn set_next_hop(&mut self, next_hop: u32) {
        let bytes = next_hop.to_be_bytes();
        self.buffer[12..16].copy_from_slice(&bytes);
    }

    pub fn ttl(&self) -> u8 {
        self.buffer[16]
    }

    pub fn set_ttl(&mut self, ttl: u8) {
        self.buffer[16] = ttl;
    }

    pub fn payload(&self) -> &[u8] {
        &self.buffer[Self::HEADER_SIZE..]
    }
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

    pub fn route_in_place(
        &mut self,
        local_node_id: u32,
        frame: &mut SpaceCANFrame,
    ) -> Result<RouteAction, RustSatError> {
        let mut header = NetworkHeaderView::new(frame.data.as_mut_slice())?;

        let dest = header.destination();

        // If we are the destination, consume it
        if dest == local_node_id {
            info!(
                "Packet arrived at final destination: Node {}",
                local_node_id
            );
            return Ok(RouteAction::Consume);
        }

        // Check TTL
        let ttl = header.ttl();
        if ttl == 0 {
            warn!("Packet dropped: TTL expired");
            return Ok(RouteAction::Drop);
        }

        // Lookup next hop
        if let Some(route) = self.routing_table.get(&dest) {
            header.set_ttl(ttl - 1);
            header.set_next_hop(route.next_hop);

            info!("Routing packet to Next-Hop: {}", route.next_hop);
            Ok(RouteAction::Forward)
        } else {
            warn!("Packet dropped: No route to destination {}", dest);
            Ok(RouteAction::Drop)
        }
    }
}

impl Default for MeshNetwork {
    fn default() -> Self {
        Self::new()
    }
}
