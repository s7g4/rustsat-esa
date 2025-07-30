// Mesh networking and routing algorithms for CubeSat constellations
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet, VecDeque};
use chrono::{DateTime, Utc, Duration};
use log::{info, warn, error, debug};

/// Network node representing a CubeSat or ground station
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkNode {
    pub node_id: u32,
    pub node_type: NodeType,
    pub position: OrbitalPosition,
    pub communication_range: f64,  // km
    pub is_active: bool,
    pub last_seen: DateTime<Utc>,
    pub battery_level: f64,  // 0.0 to 1.0
    pub neighbors: HashSet<u32>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum NodeType {
    CubeSat,
    GroundStation,
    Relay,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrbitalPosition {
    pub latitude: f64,   // degrees
    pub longitude: f64,  // degrees
    pub altitude: f64,   // km above Earth
    pub velocity: (f64, f64, f64),  // km/s in x, y, z
}

/// Routing table entry for network path finding
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoutingEntry {
    pub destination: u32,
    pub next_hop: u32,
    pub hop_count: u8,
    pub cost: f64,
    pub last_updated: DateTime<Utc>,
    pub reliability: f64,  // 0.0 to 1.0
}

/// Routing table for mesh network
#[derive(Debug, Clone, Default)]
pub struct RoutingTable {
    entries: HashMap<u32, RoutingEntry>,
    update_interval: Duration,
}

/// Network packet for routing through the mesh
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkPacket {
    pub packet_id: u32,
    pub source: u32,
    pub destination: u32,
    pub next_hop: u32,
    pub ttl: u8,
    pub priority: u8,
    pub timestamp: DateTime<Utc>,
    pub payload: Vec<u8>,
    pub route_history: Vec<u32>,
}

/// Advanced mesh network implementation for CubeSat constellations
pub struct MeshNetwork {
    nodes: HashMap<u32, NetworkNode>,
    routing_table: RoutingTable,
    packet_buffer: VecDeque<NetworkPacket>,
    network_topology: NetworkTopology,
    ground_stations: HashSet<u32>,
    statistics: NetworkStatistics,
}

#[derive(Debug, Clone, Default)]
pub struct NetworkTopology {
    adjacency_matrix: HashMap<(u32, u32), f64>,  // (node1, node2) -> link quality
    connectivity_graph: HashMap<u32, HashSet<u32>>,
}

#[derive(Debug, Clone, Default)]
pub struct NetworkStatistics {
    pub packets_routed: u64,
    pub packets_dropped: u64,
    pub average_hop_count: f64,
    pub network_utilization: f64,
    pub handovers_completed: u64,
    pub total_latency: Duration,
}

impl MeshNetwork {
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
            routing_table: RoutingTable::default(),
            packet_buffer: VecDeque::new(),
            network_topology: NetworkTopology::default(),
            ground_stations: HashSet::new(),
            statistics: NetworkStatistics::default(),
        }
    }

    /// Initialize routing protocols and network discovery
    pub fn initialize_routing(&mut self) -> Result<(), String> {
        info!("Initializing mesh network routing protocols");
        
        // Start network discovery
        self.discover_neighbors()?;
        
        // Build initial routing table
        self.build_routing_table()?;
        
        // Initialize ground station connections
        self.initialize_ground_stations()?;
        
        Ok(())
    }

    /// Add a new node to the mesh network
    pub fn add_node(&mut self, node: NetworkNode) {
        let node_id = node.node_id;
        
        if node.node_type == NodeType::GroundStation {
            self.ground_stations.insert(node_id);
        }
        
        self.nodes.insert(node_id, node);
        self.update_network_topology();
        
        info!("Added node {} to mesh network", node_id);
    }

    /// Remove a node from the mesh network
    pub fn remove_node(&mut self, node_id: u32) {
        if let Some(_node) = self.nodes.remove(&node_id) {
            self.ground_stations.remove(&node_id);
            
            // Update routing table to remove routes through this node
            self.routing_table.entries.retain(|_, entry| {
                entry.next_hop != node_id && entry.destination != node_id
            });
            
            // Update topology
            self.update_network_topology();
            
            info!("Removed node {} from mesh network", node_id);
        }
    }

    /// Route a message through the mesh network using advanced algorithms
    pub fn route_message(&mut self, source: u32, destination: u32, data: &[u8]) -> Result<bool, String> {
        // Create network packet
        let packet = NetworkPacket {
            packet_id: rand::random::<u32>(),
            source,
            destination,
            next_hop: 0,  // Will be determined by routing
            ttl: 32,  // Maximum hops
            priority: 1,
            timestamp: Utc::now(),
            payload: data.to_vec(),
            route_history: vec![source],
        };

        // Find optimal route
        let route = self.find_optimal_route(source, destination)?;
        
        if route.is_empty() {
            warn!("No route found from {} to {}", source, destination);
            self.statistics.packets_dropped += 1;
            return Ok(false);
        }

        // Forward packet along the route
        self.forward_packet(packet, &route)?;
        
        self.statistics.packets_routed += 1;
        self.statistics.average_hop_count = 
            (self.statistics.average_hop_count * (self.statistics.packets_routed - 1) as f64 + route.len() as f64) 
            / self.statistics.packets_routed as f64;

        info!("Successfully routed message from {} to {} via {} hops", 
              source, destination, route.len());
        
        Ok(true)
    }

    /// Find optimal route using Dijkstra's algorithm with space-specific metrics
    fn find_optimal_route(&self, source: u32, destination: u32) -> Result<Vec<u32>, String> {
        if source == destination {
            return Ok(vec![]);
        }

        let mut distances: HashMap<u32, f64> = HashMap::new();
        let mut previous: HashMap<u32, u32> = HashMap::new();
        let mut unvisited: HashSet<u32> = self.nodes.keys().cloned().collect();

        // Initialize distances
        for &node_id in &unvisited {
            distances.insert(node_id, if node_id == source { 0.0 } else { f64::INFINITY });
        }

        while !unvisited.is_empty() {
            // Find unvisited node with minimum distance
            let current = *unvisited.iter()
                .min_by(|&&a, &&b| distances[&a].partial_cmp(&distances[&b]).unwrap())
                .ok_or("No unvisited nodes")?;

            if current == destination {
                break;
            }

            unvisited.remove(&current);

            // Update distances to neighbors
            if let Some(current_node) = self.nodes.get(&current) {
                for &neighbor in &current_node.neighbors {
                    if unvisited.contains(&neighbor) {
                        let edge_cost = self.calculate_link_cost(current, neighbor);
                        let alt_distance = distances[&current] + edge_cost;

                        if alt_distance < distances[&neighbor] {
                            distances.insert(neighbor, alt_distance);
                            previous.insert(neighbor, current);
                        }
                    }
                }
            }
        }

        // Reconstruct path
        let mut path = Vec::new();
        let mut current = destination;

        while let Some(&prev) = previous.get(&current) {
            path.push(current);
            current = prev;
        }

        if current != source {
            return Err("No route found".to_string());
        }

        path.reverse();
        Ok(path)
    }

    /// Calculate link cost considering orbital mechanics and power constraints
    fn calculate_link_cost(&self, node1: u32, node2: u32) -> f64 {
        let (n1, n2) = match (self.nodes.get(&node1), self.nodes.get(&node2)) {
            (Some(n1), Some(n2)) => (n1, n2),
            _ => return f64::INFINITY,
        };

        // Base cost from distance
        let distance = self.calculate_distance(&n1.position, &n2.position);
        let mut cost = distance / 1000.0;  // Normalize to reasonable range

        // Adjust for battery levels (prefer nodes with higher battery)
        cost *= (2.0 - n1.battery_level.min(n2.battery_level));

        // Adjust for node reliability
        if let Some(entry) = self.routing_table.entries.get(&node2) {
            cost *= (2.0 - entry.reliability);
        }

        // Penalty for ground station handovers (more complex)
        if n1.node_type != n2.node_type {
            cost *= 1.5;
        }

        cost
    }

    /// Calculate 3D distance between two orbital positions
    fn calculate_distance(&self, pos1: &OrbitalPosition, pos2: &OrbitalPosition) -> f64 {
        let earth_radius = 6371.0; // km

        // Convert to Cartesian coordinates
        let (x1, y1, z1) = self.spherical_to_cartesian(pos1, earth_radius);
        let (x2, y2, z2) = self.spherical_to_cartesian(pos2, earth_radius);

        // Calculate Euclidean distance
        ((x2 - x1).powi(2) + (y2 - y1).powi(2) + (z2 - z1).powi(2)).sqrt()
    }

    fn spherical_to_cartesian(&self, pos: &OrbitalPosition, earth_radius: f64) -> (f64, f64, f64) {
        let lat_rad = pos.latitude.to_radians();
        let lon_rad = pos.longitude.to_radians();
        let r = earth_radius + pos.altitude;

        let x = r * lat_rad.cos() * lon_rad.cos();
        let y = r * lat_rad.cos() * lon_rad.sin();
        let z = r * lat_rad.sin();

        (x, y, z)
    }

    /// Forward packet along the determined route
    fn forward_packet(&mut self, mut packet: NetworkPacket, route: &[u32]) -> Result<(), String> {
        for (i, &next_hop) in route.iter().enumerate() {
            packet.next_hop = next_hop;
            packet.route_history.push(next_hop);
            packet.ttl -= 1;

            if packet.ttl == 0 {
                warn!("Packet {} exceeded TTL", packet.packet_id);
                self.statistics.packets_dropped += 1;
                return Err("TTL exceeded".to_string());
            }

            // Simulate packet transmission delay
            let transmission_delay = self.calculate_transmission_delay(
                if i == 0 { packet.source } else { route[i-1] },
                next_hop
            );

            debug!("Forwarding packet {} to node {} (delay: {:.2}ms)", 
                   packet.packet_id, next_hop, transmission_delay * 1000.0);
        }

        Ok(())
    }

    /// Calculate transmission delay based on distance and link quality
    fn calculate_transmission_delay(&self, from: u32, to: u32) -> f64 {
        let (n1, n2) = match (self.nodes.get(&from), self.nodes.get(&to)) {
            (Some(n1), Some(n2)) => (n1, n2),
            _ => return 1.0,  // Default delay
        };

        let distance = self.calculate_distance(&n1.position, &n2.position);
        let speed_of_light = 299792.458; // km/ms

        // Propagation delay
        let propagation_delay = distance / speed_of_light;

        // Processing delay (varies by node type)
        let processing_delay = match n2.node_type {
            NodeType::CubeSat => 0.001,      // 1ms
            NodeType::GroundStation => 0.005, // 5ms
            NodeType::Relay => 0.002,        // 2ms
        };

        propagation_delay + processing_delay
    }

    /// Discover neighboring nodes within communication range
    fn discover_neighbors(&mut self) -> Result<(), String> {
        let node_ids: Vec<u32> = self.nodes.keys().cloned().collect();
        
        for &node1_id in &node_ids {
            for &node2_id in &node_ids {
                if node1_id != node2_id {
                    if let (Some(node1), Some(node2)) = (self.nodes.get(&node1_id), self.nodes.get(&node2_id)) {
                        let distance = self.calculate_distance(&node1.position, &node2.position);
                        
                        if distance <= node1.communication_range.min(node2.communication_range) {
                            // Nodes are within communication range
                            if let Some(node1_mut) = self.nodes.get_mut(&node1_id) {
                                node1_mut.neighbors.insert(node2_id);
                            }
                        }
                    }
                }
            }
        }

        info!("Network discovery completed. Found {} total connections", 
              self.nodes.values().map(|n| n.neighbors.len()).sum::<usize>());
        
        Ok(())
    }

    /// Build and update routing table using distance vector algorithm
    fn build_routing_table(&mut self) -> Result<(), String> {
        self.routing_table.entries.clear();

        // Initialize direct routes
        for (&node_id, node) in &self.nodes {
            for &neighbor in &node.neighbors {
                let cost = self.calculate_link_cost(node_id, neighbor);
                let entry = RoutingEntry {
                    destination: neighbor,
                    next_hop: neighbor,
                    hop_count: 1,
                    cost,
                    last_updated: Utc::now(),
                    reliability: 0.9,  // Initial reliability
                };
                self.routing_table.entries.insert(neighbor, entry);
            }
        }

        // Bellman-Ford algorithm for multi-hop routes
        for _ in 0..self.nodes.len() {
            let mut updated = false;
            
            for (&node_id, node) in &self.nodes {
                for &neighbor in &node.neighbors {
                    if let Some(neighbor_routes) = self.get_routes_from_node(neighbor) {
                        for (dest, route) in neighbor_routes {
                            if dest != node_id {  // Avoid loops
                                let new_cost = self.calculate_link_cost(node_id, neighbor) + route.cost;
                                let new_hop_count = route.hop_count + 1;
                                
                                let should_update = match self.routing_table.entries.get(&dest) {
                                    Some(existing) => new_cost < existing.cost,
                                    None => true,
                                };
                                
                                if should_update && new_hop_count < 16 {  // Prevent infinite loops
                                    let entry = RoutingEntry {
                                        destination: dest,
                                        next_hop: neighbor,
                                        hop_count: new_hop_count,
                                        cost: new_cost,
                                        last_updated: Utc::now(),
                                        reliability: route.reliability * 0.95,  // Degrade with hops
                                    };
                                    self.routing_table.entries.insert(dest, entry);
                                    updated = true;
                                }
                            }
                        }
                    }
                }
            }
            
            if !updated {
                break;  // Convergence reached
            }
        }

        info!("Routing table built with {} entries", self.routing_table.entries.len());
        Ok(())
    }

    /// Get routing entries for a specific node (helper for routing algorithm)
    fn get_routes_from_node(&self, _node_id: u32) -> Option<HashMap<u32, RoutingEntry>> {
        // In a real implementation, this would query the specific node
        // For simulation, we use the global routing table
        Some(self.routing_table.entries.clone())
    }

    /// Initialize ground station connections and handover protocols
    fn initialize_ground_stations(&mut self) -> Result<(), String> {
        for &gs_id in &self.ground_stations.clone() {
            if let Some(gs_node) = self.nodes.get_mut(&gs_id) {
                gs_node.communication_range = 5000.0;  // Extended range for ground stations
                gs_node.battery_level = 1.0;  // Always powered
                
                info!("Initialized ground station {} with extended range", gs_id);
            }
        }
        
        Ok(())
    }

    /// Update network topology based on current node positions
    fn update_network_topology(&mut self) {
        self.network_topology.adjacency_matrix.clear();
        self.network_topology.connectivity_graph.clear();

        for (&node1_id, node1) in &self.nodes {
            let mut connections = HashSet::new();
            
            for (&node2_id, node2) in &self.nodes {
                if node1_id != node2_id {
                    let distance = self.calculate_distance(&node1.position, &node2.position);
                    let max_range = node1.communication_range.min(node2.communication_range);
                    
                    if distance <= max_range {
                        let link_quality = (1.0 - (distance / max_range)).max(0.1);
                        self.network_topology.adjacency_matrix.insert((node1_id, node2_id), link_quality);
                        connections.insert(node2_id);
                    }
                }
            }
            
            self.network_topology.connectivity_graph.insert(node1_id, connections);
        }
    }

    /// Handle ground station handover for continuous connectivity
    pub fn handle_ground_station_handover(&mut self, cubesat_id: u32) -> Result<Option<u32>, String> {
        let cubesat = self.nodes.get(&cubesat_id)
            .ok_or("CubeSat not found")?;

        if cubesat.node_type != NodeType::CubeSat {
            return Err("Node is not a CubeSat".to_string());
        }

        // Find best ground station based on signal strength and availability
        let mut best_gs = None;
        let mut best_quality = 0.0;

        for &gs_id in &self.ground_stations {
            if let Some(gs_node) = self.nodes.get(&gs_id) {
                let distance = self.calculate_distance(&cubesat.position, &gs_node.position);
                
                if distance <= gs_node.communication_range {
                    let signal_quality = (1.0 - (distance / gs_node.communication_range)).max(0.0);
                    
                    if signal_quality > best_quality {
                        best_quality = signal_quality;
                        best_gs = Some(gs_id);
                    }
                }
            }
        }

        if let Some(gs_id) = best_gs {
            self.statistics.handovers_completed += 1;
            info!("Handover completed: CubeSat {} -> Ground Station {} (quality: {:.2})", 
                  cubesat_id, gs_id, best_quality);
        }

        Ok(best_gs)
    }

    /// Get network statistics for monitoring and optimization
    pub fn get_statistics(&self) -> &NetworkStatistics {
        &self.statistics
    }

    /// Update node positions (for orbital simulation)
    pub fn update_node_position(&mut self, node_id: u32, new_position: OrbitalPosition) -> Result<(), String> {
        if let Some(node) = self.nodes.get_mut(&node_id) {
            node.position = new_position;
            node.last_seen = Utc::now();
            
            // Trigger topology update
            self.update_network_topology();
            
            debug!("Updated position for node {}", node_id);
            Ok(())
        } else {
            Err(format!("Node {} not found", node_id))
        }
    }

    /// Simulate network evolution over time
    pub fn simulate_network_step(&mut self, time_delta: Duration) -> Result<(), String> {
        // Update orbital positions based on velocity
        let node_ids: Vec<u32> = self.nodes.keys().cloned().collect();
        
        for node_id in node_ids {
            if let Some(node) = self.nodes.get_mut(&node_id) {
                if node.node_type == NodeType::CubeSat {
                    // Simple orbital mechanics simulation
                    let dt = time_delta.num_seconds() as f64;
                    
                    node.position.latitude += node.position.velocity.0 * dt / 111.0; // Rough conversion
                    node.position.longitude += node.position.velocity.1 * dt / 111.0;
                    node.position.altitude += node.position.velocity.2 * dt;
                    
                    // Simulate battery drain
                    node.battery_level = (node.battery_level - 0.001 * dt).max(0.0);
                }
            }
        }

        // Update network topology
        self.update_network_topology();
        
        // Rebuild routing table periodically
        if self.statistics.packets_routed % 100 == 0 {
            self.build_routing_table()?;
        }

        Ok(())
    }
}

impl Default for MeshNetwork {
    fn default() -> Self {
        Self::new()
    }
}

impl NetworkNode {
    pub fn new_cubesat(node_id: u32, position: OrbitalPosition) -> Self {
        Self {
            node_id,
            node_type: NodeType::CubeSat,
            position,
            communication_range: 1000.0,  // 1000 km default range
            is_active: true,
            last_seen: Utc::now(),
            battery_level: 1.0,
            neighbors: HashSet::new(),
        }
    }

    pub fn new_ground_station(node_id: u32, latitude: f64, longitude: f64) -> Self {
        Self {
            node_id,
            node_type: NodeType::GroundStation,
            position: OrbitalPosition {
                latitude,
                longitude,
                altitude: 0.0,
                velocity: (0.0, 0.0, 0.0),
            },
            communication_range: 5000.0,  // Extended range for ground stations
            is_active: true,
            last_seen: Utc::now(),
            battery_level: 1.0,  // Always powered
            neighbors: HashSet::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_network_creation() {
        let network = MeshNetwork::new();
        assert_eq!(network.nodes.len(), 0);
        assert_eq!(network.ground_stations.len(), 0);
    }

    #[test]
    fn test_node_addition() {
        let mut network = MeshNetwork::new();
        let position = OrbitalPosition {
            latitude: 0.0,
            longitude: 0.0,
            altitude: 400.0,
            velocity: (7.66, 0.0, 0.0),
        };
        
        let node = NetworkNode::new_cubesat(1, position);
        network.add_node(node);
        
        assert_eq!(network.nodes.len(), 1);
        assert!(network.nodes.contains_key(&1));
    }

    #[test]
    fn test_ground_station_creation() {
        let gs = NetworkNode::new_ground_station(100, 52.5, 13.4); // Berlin
        assert_eq!(gs.node_type, NodeType::GroundStation);
        assert_eq!(gs.communication_range, 5000.0);
        assert_eq!(gs.battery_level, 1.0);
    }

    #[test]
    fn test_distance_calculation() {
        let network = MeshNetwork::new();
        
        let pos1 = OrbitalPosition {
            latitude: 0.0,
            longitude: 0.0,
            altitude: 400.0,
            velocity: (0.0, 0.0, 0.0),
        };
        
        let pos2 = OrbitalPosition {
            latitude: 1.0,
            longitude: 1.0,
            altitude: 400.0,
            velocity: (0.0, 0.0, 0.0),
        };
        
        let distance = network.calculate_distance(&pos1, &pos2);
        assert!(distance > 0.0);
        assert!(distance < 200.0); // Should be reasonable for 1 degree difference
    }

    #[test]
    fn test_routing_initialization() {
        let mut network = MeshNetwork::new();
        
        // Add some test nodes
        let pos1 = OrbitalPosition {
            latitude: 0.0,
            longitude: 0.0,
            altitude: 400.0,
            velocity: (7.66, 0.0, 0.0),
        };
        
        let pos2 = OrbitalPosition {
            latitude: 0.5,
            longitude: 0.5,
            altitude: 400.0,
            velocity: (7.66, 0.0, 0.0),
        };
        
        network.add_node(NetworkNode::new_cubesat(1, pos1));
        network.add_node(NetworkNode::new_cubesat(2, pos2));
        
        assert!(network.initialize_routing().is_ok());
    }
}
