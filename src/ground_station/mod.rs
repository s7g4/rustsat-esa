// Ground station communication interface and ESA ground network integration
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, VecDeque};
use chrono::{DateTime, Utc, Duration};
use log::{info, warn, error, debug};
use crate::protocol::network::{NetworkNode, NodeType, OrbitalPosition};
use crate::telemetry::{TelemetryPacket, MissionEvent, EventType, EventStatus};

/// Ground station configuration and capabilities
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroundStation {
    pub station_id: u32,
    pub name: String,
    pub location: GeographicLocation,
    pub capabilities: StationCapabilities,
    pub status: StationStatus,
    pub contact_schedule: Vec<ContactWindow>,
    pub data_buffer: VecDeque<GroundStationMessage>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeographicLocation {
    pub latitude: f64,   // degrees
    pub longitude: f64,  // degrees
    pub altitude: f64,   // meters above sea level
    pub timezone: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StationCapabilities {
    pub frequency_bands: Vec<FrequencyBand>,
    pub max_data_rate: f64,  // Mbps
    pub antenna_gain: f64,   // dBi
    pub tracking_capability: bool,
    pub uplink_power: f64,   // Watts
    pub supported_protocols: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FrequencyBand {
    pub name: String,
    pub frequency_mhz: f64,
    pub bandwidth_khz: f64,
    pub polarization: Polarization,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Polarization {
    Linear,
    Circular,
    RHCP,  // Right-Hand Circular Polarization
    LHCP,  // Left-Hand Circular Polarization
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum StationStatus {
    Online,
    Offline,
    Maintenance,
    Tracking,
    Communicating,
}

/// Contact window for satellite communication
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContactWindow {
    pub window_id: u32,
    pub satellite_id: u32,
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
    pub max_elevation: f64,  // degrees
    pub azimuth_range: (f64, f64),  // start, end degrees
    pub predicted_snr: f64,  // dB
    pub priority: u8,
}

/// Ground station message types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GroundStationMessage {
    TelemetryData(TelemetryPacket),
    Command(CommandMessage),
    StatusUpdate(StatusMessage),
    EmergencyAlert(EmergencyMessage),
    TimeSync(TimeSyncMessage),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommandMessage {
    pub command_id: u32,
    pub target_satellite: u32,
    pub command_type: CommandType,
    pub parameters: HashMap<String, String>,
    pub execution_time: Option<DateTime<Utc>>,
    pub priority: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CommandType {
    SystemReboot,
    PayloadActivation,
    OrbitManeuver,
    DataDownload,
    ConfigurationUpdate,
    EmergencyShutdown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatusMessage {
    pub satellite_id: u32,
    pub system_status: String,
    pub battery_level: f64,
    pub temperature: f64,
    pub last_contact: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmergencyMessage {
    pub satellite_id: u32,
    pub emergency_type: EmergencyType,
    pub description: String,
    pub severity: u8,  // 1-10
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EmergencyType {
    PowerFailure,
    CommunicationLoss,
    ThermalAnomaly,
    AttitudeControl,
    PayloadFailure,
    CollisionAvoidance,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeSyncMessage {
    pub ground_time: DateTime<Utc>,
    pub satellite_time: DateTime<Utc>,
    pub time_offset: Duration,
    pub sync_accuracy: Duration,
}

/// ESA ground network interface
pub struct ESAGroundNetwork {
    stations: HashMap<u32, GroundStation>,
    active_contacts: HashMap<u32, ContactSession>,
    message_queue: VecDeque<GroundStationMessage>,
    network_statistics: NetworkStatistics,
    protocol_handlers: HashMap<String, Box<dyn ProtocolHandler>>,
}

/// Active contact session between ground station and satellite
#[derive(Debug, Clone)]
pub struct ContactSession {
    pub session_id: u32,
    pub station_id: u32,
    pub satellite_id: u32,
    pub start_time: DateTime<Utc>,
    pub expected_end_time: DateTime<Utc>,
    pub data_transferred: u64,  // bytes
    pub signal_quality: f64,    // 0.0 to 1.0
    pub status: SessionStatus,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SessionStatus {
    Establishing,
    Active,
    Degraded,
    Terminating,
    Completed,
    Failed,
}

/// Network statistics for monitoring
#[derive(Debug, Clone, Default)]
pub struct NetworkStatistics {
    pub total_contacts: u64,
    pub successful_contacts: u64,
    pub data_volume_gb: f64,
    pub average_contact_duration: Duration,
    pub network_availability: f64,
    pub error_rate: f64,
}

/// Protocol handler trait for different communication protocols
pub trait ProtocolHandler: Send + Sync {
    fn handle_message(&self, message: &[u8]) -> Result<Vec<u8>, String>;
    fn get_protocol_name(&self) -> &str;
}

/// ESA-compatible protocol handler
pub struct ESAProtocolHandler {
    protocol_version: String,
}

impl ESAGroundNetwork {
    pub fn new() -> Self {
        let mut network = Self {
            stations: HashMap::new(),
            active_contacts: HashMap::new(),
            message_queue: VecDeque::new(),
            network_statistics: NetworkStatistics::default(),
            protocol_handlers: HashMap::new(),
        };

        // Register ESA protocol handler
        let esa_handler = ESAProtocolHandler {
            protocol_version: "ESA-CUBESAT-1.0".to_string(),
        };
        network.protocol_handlers.insert(
            "ESA-CUBESAT".to_string(),
            Box::new(esa_handler)
        );

        network
    }

    /// Initialize ESA ground network with default stations
    pub fn initialize(&mut self) -> Result<(), String> {
        info!("Initializing ESA ground network");

        // Add major ESA ground stations
        self.add_esa_stations()?;

        // Initialize contact scheduling
        self.initialize_contact_scheduling()?;

        info!("ESA ground network initialized with {} stations", self.stations.len());
        Ok(())
    }

    /// Add major ESA ground stations
    fn add_esa_stations(&mut self) -> Result<(), String> {
        // ESOC Darmstadt, Germany
        let esoc = GroundStation {
            station_id: 1,
            name: "ESOC Darmstadt".to_string(),
            location: GeographicLocation {
                latitude: 49.8728,
                longitude: 8.6512,
                altitude: 144.0,
                timezone: "Europe/Berlin".to_string(),
            },
            capabilities: StationCapabilities {
                frequency_bands: vec![
                    FrequencyBand {
                        name: "S-band".to_string(),
                        frequency_mhz: 2200.0,
                        bandwidth_khz: 100.0,
                        polarization: Polarization::RHCP,
                    },
                    FrequencyBand {
                        name: "X-band".to_string(),
                        frequency_mhz: 8400.0,
                        bandwidth_khz: 200.0,
                        polarization: Polarization::RHCP,
                    },
                ],
                max_data_rate: 10.0,
                antenna_gain: 45.0,
                tracking_capability: true,
                uplink_power: 1000.0,
                supported_protocols: vec!["ESA-CUBESAT".to_string(), "CCSDS".to_string()],
            },
            status: StationStatus::Online,
            contact_schedule: Vec::new(),
            data_buffer: VecDeque::new(),
        };
        self.stations.insert(1, esoc);

        // Kourou, French Guiana
        let kourou = GroundStation {
            station_id: 2,
            name: "Kourou".to_string(),
            location: GeographicLocation {
                latitude: 5.1664,
                longitude: -52.6843,
                altitude: 50.0,
                timezone: "America/Cayenne".to_string(),
            },
            capabilities: StationCapabilities {
                frequency_bands: vec![
                    FrequencyBand {
                        name: "UHF".to_string(),
                        frequency_mhz: 437.5,
                        bandwidth_khz: 25.0,
                        polarization: Polarization::Linear,
                    },
                    FrequencyBand {
                        name: "S-band".to_string(),
                        frequency_mhz: 2400.0,
                        bandwidth_khz: 100.0,
                        polarization: Polarization::RHCP,
                    },
                ],
                max_data_rate: 5.0,
                antenna_gain: 35.0,
                tracking_capability: true,
                uplink_power: 500.0,
                supported_protocols: vec!["ESA-CUBESAT".to_string()],
            },
            status: StationStatus::Online,
            contact_schedule: Vec::new(),
            data_buffer: VecDeque::new(),
        };
        self.stations.insert(2, kourou);

        // Redu, Belgium
        let redu = GroundStation {
            station_id: 3,
            name: "Redu".to_string(),
            location: GeographicLocation {
                latitude: 50.0019,
                longitude: 5.1456,
                altitude: 380.0,
                timezone: "Europe/Brussels".to_string(),
            },
            capabilities: StationCapabilities {
                frequency_bands: vec![
                    FrequencyBand {
                        name: "S-band".to_string(),
                        frequency_mhz: 2200.0,
                        bandwidth_khz: 100.0,
                        polarization: Polarization::RHCP,
                    },
                ],
                max_data_rate: 8.0,
                antenna_gain: 40.0,
                tracking_capability: true,
                uplink_power: 750.0,
                supported_protocols: vec!["ESA-CUBESAT".to_string(), "CCSDS".to_string()],
            },
            status: StationStatus::Online,
            contact_schedule: Vec::new(),
            data_buffer: VecDeque::new(),
        };
        self.stations.insert(3, redu);

        Ok(())
    }

    /// Initialize contact scheduling for all stations
    fn initialize_contact_scheduling(&mut self) -> Result<(), String> {
        let now = Utc::now();
        
        for (station_id, station) in &mut self.stations {
            // Generate contact windows for the next 24 hours
            for hour in 0..24 {
                let contact_start = now + Duration::hours(hour) + Duration::minutes(rand::random::<i64>() % 60);
                let contact_duration = Duration::minutes(8 + rand::random::<i64>() % 12); // 8-20 minutes
                
                let contact_window = ContactWindow {
                    window_id: (station_id * 1000 + hour as u32),
                    satellite_id: 1, // Default satellite
                    start_time: contact_start,
                    end_time: contact_start + contact_duration,
                    max_elevation: 30.0 + rand::random::<f64>() * 60.0, // 30-90 degrees
                    azimuth_range: (0.0, 360.0),
                    predicted_snr: 10.0 + rand::random::<f64>() * 20.0, // 10-30 dB
                    priority: 1,
                };
                
                station.contact_schedule.push(contact_window);
            }
            
            // Sort by start time
            station.contact_schedule.sort_by_key(|w| w.start_time);
        }

        Ok(())
    }

    /// Establish contact with a satellite
    pub fn establish_contact(&mut self, station_id: u32, satellite_id: u32) -> Result<u32, String> {
        let station = self.stations.get(&station_id)
            .ok_or(format!("Station {} not found", station_id))?;

        if station.status != StationStatus::Online {
            return Err(format!("Station {} is not online", station_id));
        }

        // Check if there's a scheduled contact window
        let now = Utc::now();
        let contact_window = station.contact_schedule.iter()
            .find(|w| w.satellite_id == satellite_id && 
                     w.start_time <= now && 
                     w.end_time >= now);

        if contact_window.is_none() {
            return Err("No active contact window".to_string());
        }

        let window = contact_window.unwrap();
        let session_id = rand::random::<u32>();

        let contact_session = ContactSession {
            session_id,
            station_id,
            satellite_id,
            start_time: now,
            expected_end_time: window.end_time,
            data_transferred: 0,
            signal_quality: window.predicted_snr / 30.0, // Normalize to 0-1
            status: SessionStatus::Establishing,
        };

        self.active_contacts.insert(session_id, contact_session);

        // Update station status
        if let Some(station) = self.stations.get_mut(&station_id) {
            station.status = StationStatus::Tracking;
        }

        info!("Established contact session {} between station {} and satellite {}", 
              session_id, station_id, satellite_id);

        Ok(session_id)
    }

    /// Send command to satellite
    pub fn send_command(&mut self, session_id: u32, command: CommandMessage) -> Result<(), String> {
        let session = self.active_contacts.get_mut(&session_id)
            .ok_or("Contact session not found")?;

        if session.status != SessionStatus::Active {
            return Err("Contact session not active".to_string());
        }

        // Validate command
        self.validate_command(&command)?;

        // Queue command for transmission
        let message = GroundStationMessage::Command(command.clone());
        self.message_queue.push_back(message);

        info!("Queued command {} for satellite {} via session {}", 
              command.command_id, command.target_satellite, session_id);

        Ok(())
    }

    /// Validate command before transmission
    fn validate_command(&self, command: &CommandMessage) -> Result<(), String> {
        // Check command priority
        if command.priority > 10 {
            return Err("Invalid command priority".to_string());
        }

        // Validate execution time
        if let Some(exec_time) = command.execution_time {
            let now = Utc::now();
            if exec_time < now {
                return Err("Command execution time is in the past".to_string());
            }
            
            if exec_time > now + Duration::days(7) {
                return Err("Command execution time too far in future".to_string());
            }
        }

        // Validate critical commands
        match command.command_type {
            CommandType::EmergencyShutdown => {
                if command.priority < 9 {
                    return Err("Emergency shutdown requires high priority".to_string());
                }
            },
            CommandType::OrbitManeuver => {
                if !command.parameters.contains_key("delta_v") {
                    return Err("Orbit maneuver requires delta_v parameter".to_string());
                }
            },
            _ => {}
        }

        Ok(())
    }

    /// Receive telemetry data from satellite
    pub fn receive_telemetry(&mut self, session_id: u32, telemetry: TelemetryPacket) -> Result<(), String> {
        let session = self.active_contacts.get_mut(&session_id)
            .ok_or("Contact session not found")?;

        if session.status != SessionStatus::Active {
            return Err("Contact session not active".to_string());
        }

        // Update session statistics
        let data_size = serde_json::to_vec(&telemetry)
            .map_err(|e| format!("Serialization error: {}", e))?
            .len() as u64;
        
        session.data_transferred += data_size;

        // Store telemetry data
        let message = GroundStationMessage::TelemetryData(telemetry);
        if let Some(station) = self.stations.get_mut(&session.station_id) {
            station.data_buffer.push_back(message);
            
            // Maintain buffer size
            if station.data_buffer.len() > 10000 {
                station.data_buffer.pop_front();
            }
        }

        // Update network statistics
        self.network_statistics.data_volume_gb += data_size as f64 / (1024.0 * 1024.0 * 1024.0);

        debug!("Received telemetry data via session {} ({} bytes)", session_id, data_size);
        Ok(())
    }

    /// Handle ground station handover
    pub fn handle_handover(&mut self, from_station: u32, to_station: u32, satellite_id: u32) -> Result<u32, String> {
        // Find active session with the satellite
        let old_session_id = self.active_contacts.iter()
            .find(|(_, session)| session.station_id == from_station && session.satellite_id == satellite_id)
            .map(|(&id, _)| id)
            .ok_or("No active session found for handover")?;

        // Terminate old session
        self.terminate_contact(old_session_id)?;

        // Establish new session
        let new_session_id = self.establish_contact(to_station, satellite_id)?;

        info!("Completed handover from station {} to station {} for satellite {}", 
              from_station, to_station, satellite_id);

        Ok(new_session_id)
    }

    /// Terminate contact session
    pub fn terminate_contact(&mut self, session_id: u32) -> Result<(), String> {
        if let Some(mut session) = self.active_contacts.remove(&session_id) {
            session.status = SessionStatus::Completed;
            
            // Update station status
            if let Some(station) = self.stations.get_mut(&session.station_id) {
                station.status = StationStatus::Online;
            }

            // Update statistics
            let duration = Utc::now().signed_duration_since(session.start_time);
            self.network_statistics.total_contacts += 1;
            self.network_statistics.successful_contacts += 1;
            self.network_statistics.average_contact_duration = 
                (self.network_statistics.average_contact_duration * (self.network_statistics.total_contacts - 1) as i32 + duration) 
                / self.network_statistics.total_contacts as i32;

            info!("Terminated contact session {} (duration: {} minutes, data: {} bytes)", 
                  session_id, duration.num_minutes(), session.data_transferred);
        }

        Ok(())
    }

    /// Get network statistics
    pub fn get_statistics(&self) -> &NetworkStatistics {
        &self.network_statistics
    }

    /// Get active contact sessions
    pub fn get_active_contacts(&self) -> Vec<&ContactSession> {
        self.active_contacts.values().collect()
    }

    /// Get station information
    pub fn get_station(&self, station_id: u32) -> Option<&GroundStation> {
        self.stations.get(&station_id)
    }

    /// Update station status
    pub fn update_station_status(&mut self, station_id: u32, status: StationStatus) -> Result<(), String> {
        if let Some(station) = self.stations.get_mut(&station_id) {
            station.status = status.clone();
            info!("Updated station {} status to {:?}", station_id, status);
            Ok(())
        } else {
            Err(format!("Station {} not found", station_id))
        }
    }

    /// Process message queue
    pub fn process_message_queue(&mut self) -> Result<Vec<GroundStationMessage>, String> {
        let mut processed_messages = Vec::new();
        
        while let Some(message) = self.message_queue.pop_front() {
            // Process message based on type
            match &message {
                GroundStationMessage::Command(cmd) => {
                    info!("Processing command {} for satellite {}", cmd.command_id, cmd.target_satellite);
                },
                GroundStationMessage::TelemetryData(tel) => {
                    debug!("Processing telemetry from satellite {}", tel.source_node);
                },
                GroundStationMessage::EmergencyAlert(alert) => {
                    error!("Processing emergency alert from satellite {}: {:?}", 
                           alert.satellite_id, alert.emergency_type);
                },
                _ => {}
            }
            
            processed_messages.push(message);
        }

        Ok(processed_messages)
    }
}

impl Default for ESAGroundNetwork {
    fn default() -> Self {
        Self::new()
    }
}

impl ProtocolHandler for ESAProtocolHandler {
    fn handle_message(&self, message: &[u8]) -> Result<Vec<u8>, String> {
        // ESA protocol message handling
        info!("Processing message with ESA protocol {}", self.protocol_version);
        
        // In a real implementation, this would parse and process ESA-specific message formats
        // For now, we'll echo the message back with a protocol header
        let mut response = format!("ESA-RESPONSE:{}", self.protocol_version).into_bytes();
        response.extend_from_slice(message);
        
        Ok(response)
    }

    fn get_protocol_name(&self) -> &str {
        "ESA-CUBESAT"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ground_network_creation() {
        let network = ESAGroundNetwork::new();
        assert_eq!(network.stations.len(), 0);
        assert_eq!(network.active_contacts.len(), 0);
    }

    #[test]
    fn test_ground_network_initialization() {
        let mut network = ESAGroundNetwork::new();
        assert!(network.initialize().is_ok());
        assert!(network.stations.len() > 0);
    }

    #[test]
    fn test_contact_establishment() {
        let mut network = ESAGroundNetwork::new();
        network.initialize().unwrap();
        
        // Should fail without proper contact window
        assert!(network.establish_contact(1, 1).is_err());
    }

    #[test]
    fn test_command_validation() {
        let network = ESAGroundNetwork::new();
        
        let valid_command = CommandMessage {
            command_id: 1,
            target_satellite: 1,
            command_type: CommandType::SystemReboot,
            parameters: HashMap::new(),
            execution_time: Some(Utc::now() + Duration::hours(1)),
            priority: 5,
        };
        
        assert!(network.validate_command(&valid_command).is_ok());
        
        let invalid_command = CommandMessage {
            command_id: 2,
            target_satellite: 1,
            command_type: CommandType::EmergencyShutdown,
            parameters: HashMap::new(),
            execution_time: None,
            priority: 1, // Too low for emergency shutdown
        };
        
        assert!(network.validate_command(&invalid_command).is_err());
    }

    #[test]
    fn test_esa_protocol_handler() {
        let handler = ESAProtocolHandler {
            protocol_version: "ESA-CUBESAT-1.0".to_string(),
        };
        
        let test_message = b"Hello ESA";
        let response = handler.handle_message(test_message).unwrap();
        
        assert!(response.len() > test_message.len());
        assert_eq!(handler.get_protocol_name(), "ESA-CUBESAT");
    }
}