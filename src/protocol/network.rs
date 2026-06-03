#![allow(missing_docs)]
#![allow(unused_variables)]
use crate::error::RustSatError;

use heapless::FnvIndexMap;

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum NodeType {
    CubeSat,
    GroundStation,
    Relay,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct OrbitalPosition {
    pub latitude: f32,
    pub longitude: f32,
    pub altitude: f32,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct NetworkNode {
    pub node_id: u32,
    pub node_type: NodeType,
    pub position: OrbitalPosition,
    pub battery_level: f32, // 0.0 to 100.0
}

impl NetworkNode {
    pub fn new(
        node_id: u32,
        node_type: NodeType,
        position: OrbitalPosition,
        battery_level: f32,
    ) -> Self {
        debug_assert!(
            (0.0..=100.0).contains(&battery_level),
            "Battery level must be between 0.0 and 100.0"
        );
        Self {
            node_id,
            node_type,
            position,
            battery_level,
        }
    }
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct RoutingEntry {
    pub destination: u32,
    pub next_hop: u32,
    pub hop_count: u8,
}

use crate::protocol::spacecan::SpaceCANFrame;

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
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
    buffer: &'a [u8],
}

impl<'a> NetworkHeaderView<'a> {
    pub const HEADER_SIZE: usize = 17;

    pub fn new(buffer: &'a [u8]) -> Result<Self, RustSatError> {
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

    pub fn ttl(&self) -> u8 {
        self.buffer[16]
    }

    pub fn payload(&self) -> &[u8] {
        &self.buffer[Self::HEADER_SIZE..]
    }
}

pub struct NetworkHeaderViewMut<'a> {
    buffer: &'a mut [u8],
}

impl<'a> NetworkHeaderViewMut<'a> {
    pub const HEADER_SIZE: usize = 17;

    pub fn new(buffer: &'a mut [u8]) -> Result<Self, RustSatError> {
        if buffer.len() < Self::HEADER_SIZE {
            return Err(RustSatError::InvalidFormat);
        }
        Ok(Self { buffer })
    }

    pub fn view(&self) -> NetworkHeaderView<'_> {
        NetworkHeaderView {
            buffer: self.buffer,
        }
    }

    pub fn set_next_hop(&mut self, next_hop: u32) {
        let bytes = next_hop.to_be_bytes();
        self.buffer[12..16].copy_from_slice(&bytes);
    }

    pub fn set_ttl(&mut self, ttl: u8) {
        self.buffer[16] = ttl;
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
        #[cfg(feature = "defmt")]
        defmt::info!("Initializing mesh network routing protocols");
        // No-op for now. AODV or static table initialization to be implemented.
        Ok(())
    }

    pub fn add_node(&mut self, node: NetworkNode) -> Result<(), RustSatError> {
        if self.nodes.insert(node.node_id, node).is_err() {
            return Err(RustSatError::SystemError("Node capacity exceeded"));
        }
        Ok(())
    }

    pub fn remove_node(&mut self, node_id: u32) {
        self.nodes.remove(&node_id);

        // Purge any routing entries pointing to this node as a next hop
        let mut keys_to_remove = heapless::Vec::<u32, 32>::new();
        for (dest, entry) in self.routing_table.iter() {
            if entry.next_hop == node_id {
                keys_to_remove
                    .push(*dest)
                    .expect("infallible: capacity matches routing_table");
            }
        }
        for k in keys_to_remove {
            self.routing_table.remove(&k);
        }
    }

    pub fn route_in_place(
        &mut self,
        local_node_id: u32,
        frame: &mut SpaceCANFrame,
    ) -> Result<RouteAction, RustSatError> {
        let mut header_mut = NetworkHeaderViewMut::new(frame.data.as_mut_slice())?;

        let header = header_mut.view();
        let dest = header.destination();
        let _packet_id = header.packet_id();

        // If we are the destination, consume it
        if dest == local_node_id {
            #[cfg(feature = "defmt")]
            defmt::info!(
                "Packet {} arrived at final destination (Node {})",
                _packet_id,
                local_node_id
            );
            return Ok(RouteAction::Consume);
        }

        // Check TTL
        let ttl = header.ttl();
        if ttl == 0 {
            #[cfg(feature = "defmt")]
            defmt::warn!("Packet dropped: TTL expired");
            return Ok(RouteAction::Drop);
        }

        // Lookup next hop
        if let Some(route) = self.routing_table.get(&dest) {
            header_mut.set_ttl(ttl - 1);
            header_mut.set_next_hop(route.next_hop);

            #[cfg(feature = "defmt")]
            defmt::info!("Routing packet to Next-Hop: {}", route.next_hop);
            Ok(RouteAction::Forward)
        } else {
            #[cfg(feature = "defmt")]
            defmt::warn!("Packet dropped: No route to destination {}", dest);
            Ok(RouteAction::Drop)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::protocol::spacecan::FramePriority;
    use heapless::Vec as HeaplessVec;

    #[test]
    fn test_network_routing() {
        let mut network = MeshNetwork::new();
        network
            .nodes
            .insert(
                2,
                NetworkNode {
                    node_id: 2,
                    node_type: NodeType::Relay,
                    position: OrbitalPosition {
                        latitude: 0.0,
                        longitude: 0.0,
                        altitude: 0.0,
                    },
                    battery_level: 100.0,
                },
            )
            .unwrap();
        network
            .routing_table
            .insert(
                5,
                RoutingEntry {
                    destination: 5,
                    next_hop: 2,
                    hop_count: 1,
                },
            )
            .unwrap();

        let mut payload = HeaplessVec::new();
        payload.extend_from_slice(&1u32.to_be_bytes()).unwrap(); // packet id
        payload.extend_from_slice(&1u32.to_be_bytes()).unwrap(); // src 1
        payload.extend_from_slice(&5u32.to_be_bytes()).unwrap(); // dest 5
        payload.extend_from_slice(&0u32.to_be_bytes()).unwrap(); // next hop 0
        payload.push(10).unwrap(); // ttl 10
        payload.push(99).unwrap(); // data

        let mut frame = SpaceCANFrame::new(5, payload, FramePriority::Normal, 0, 1).unwrap();

        let action = network.route_in_place(1, &mut frame).unwrap();
        assert_eq!(action, RouteAction::Forward);

        let header = NetworkHeaderView::new(frame.data.as_slice()).unwrap();
        assert_eq!(header.next_hop(), 2);
        assert_eq!(header.ttl(), 9);
    }
}

impl Default for MeshNetwork {
    fn default() -> Self {
        Self::new()
    }
}
