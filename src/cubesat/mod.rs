// CubeSat-specific protocol adaptations and mission control
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use chrono::{DateTime, Utc, Duration};
use log::{info, warn, error, debug};
use crate::protocol::network::OrbitalPosition;
use crate::telemetry::{TelemetryData, TelemetryType, TelemetryValue, MissionEvent, EventType, EventStatus};

/// CubeSat frame with enhanced features
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CubeSatFrame {
    pub frame_type: FrameType,
    pub payload: Vec<u8>,
    pub timestamp: DateTime<Utc>,
    pub source_id: u32,
    pub destination_id: u32,
    pub sequence_number: u16,
    pub acknowledgment_required: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum FrameType {
    Telemetry = 0x01,
    Command = 0x02,
    Acknowledgment = 0x03,
    Emergency = 0x04,
    Beacon = 0x05,
    FileTransfer = 0x06,
    TimeSync = 0x07,
}

/// CubeSat mission configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MissionConfig {
    pub mission_id: String,
    pub satellite_id: u32,
    pub launch_date: DateTime<Utc>,
    pub mission_duration: Duration,
    pub orbital_parameters: OrbitalParameters,
    pub power_budget: PowerBudget,
    pub communication_schedule: CommunicationSchedule,
    pub payload_config: PayloadConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrbitalParameters {
    pub semi_major_axis: f64,    // km
    pub eccentricity: f64,       // 0-1
    pub inclination: f64,        // degrees
    pub argument_of_perigee: f64, // degrees
    pub longitude_of_ascending_node: f64, // degrees
    pub mean_anomaly: f64,       // degrees
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PowerBudget {
    pub solar_panel_power: f64,  // Watts
    pub battery_capacity: f64,   // Watt-hours
    pub system_power_consumption: f64, // Watts
    pub communication_power: f64, // Watts
    pub payload_power: f64,      // Watts
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommunicationSchedule {
    pub beacon_interval: Duration,
    pub telemetry_interval: Duration,
    pub ground_contact_windows: Vec<ContactWindow>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContactWindow {
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
    pub ground_station_id: u32,
    pub priority: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PayloadConfig {
    pub payload_type: PayloadType,
    pub operating_modes: Vec<OperatingMode>,
    pub data_collection_schedule: Vec<DataCollectionEvent>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PayloadType {
    Camera,
    Spectrometer,
    Magnetometer,
    RadioReceiver,
    Scientific,
    Technology,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperatingMode {
    pub mode_name: String,
    pub power_consumption: f64,
    pub data_rate: f64,
    pub duration_limit: Option<Duration>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataCollectionEvent {
    pub event_id: u32,
    pub start_time: DateTime<Utc>,
    pub duration: Duration,
    pub target_coordinates: Option<(f64, f64)>, // lat, lon
    pub operating_mode: String,
}

/// CubeSat protocol implementation
pub struct CubeSatProtocol {
    satellite_id: u32,
    mission_config: Option<MissionConfig>,
    system_state: SystemState,
    command_queue: Vec<CubeSatCommand>,
    telemetry_buffer: Vec<TelemetryData>,
    beacon_counter: u32,
    last_ground_contact: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemState {
    pub power_level: f64,        // 0.0 to 1.0
    pub temperature: f64,        // Celsius
    pub attitude: (f64, f64, f64), // roll, pitch, yaw in degrees
    pub position: OrbitalPosition,
    pub system_health: f64,      // 0.0 to 1.0
    pub uptime: Duration,
    pub last_updated: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CubeSatCommand {
    pub command_id: u32,
    pub command_type: CommandType,
    pub parameters: HashMap<String, String>,
    pub scheduled_execution: Option<DateTime<Utc>>,
    pub priority: u8,
    pub status: CommandStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CommandType {
    SystemReboot,
    PayloadActivate,
    PayloadDeactivate,
    AttitudeControl,
    PowerManagement,
    DataDownload,
    ConfigUpdate,
    EmergencyMode,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum CommandStatus {
    Queued,
    Executing,
    Completed,
    Failed,
    Cancelled,
}

/// Mission control system for CubeSat operations
pub struct MissionControl {
    satellites: HashMap<u32, CubeSatProtocol>,
    mission_timeline: Vec<MissionEvent>,
    ground_contacts: Vec<GroundContact>,
    emergency_procedures: HashMap<EmergencyType, EmergencyProcedure>,
    statistics: MissionStatistics,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroundContact {
    pub contact_id: u32,
    pub satellite_id: u32,
    pub ground_station_id: u32,
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
    pub data_volume: u64,
    pub commands_sent: u32,
    pub signal_quality: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EmergencyType {
    PowerCritical,
    ThermalEmergency,
    CommunicationLoss,
    AttitudeFailure,
    PayloadMalfunction,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmergencyProcedure {
    pub procedure_id: String,
    pub emergency_type: EmergencyType,
    pub actions: Vec<EmergencyAction>,
    pub timeout: Duration,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmergencyAction {
    pub action_type: String,
    pub parameters: HashMap<String, String>,
    pub delay: Duration,
}

#[derive(Debug, Clone, Default)]
pub struct MissionStatistics {
    pub total_satellites: u32,
    pub active_satellites: u32,
    pub total_commands_executed: u64,
    pub successful_ground_contacts: u64,
    pub data_volume_downlinked: u64,
    pub mission_uptime: Duration,
    pub emergency_events: u64,
}

impl CubeSatFrame {
    pub fn new(frame_type: FrameType, payload: Vec<u8>, source_id: u32, destination_id: u32) -> Self {
        Self {
            frame_type,
            payload,
            timestamp: Utc::now(),
            source_id,
            destination_id,
            sequence_number: rand::random::<u16>(),
            acknowledgment_required: false,
        }
    }

    pub fn with_acknowledgment(mut self) -> Self {
        self.acknowledgment_required = true;
        self
    }

    pub fn encode(&self) -> Vec<u8> {
        let mut encoded = Vec::new();
        
        // Frame header
        encoded.push(self.frame_type.clone() as u8);
        encoded.extend_from_slice(&self.source_id.to_be_bytes());
        encoded.extend_from_slice(&self.destination_id.to_be_bytes());
        encoded.extend_from_slice(&self.sequence_number.to_be_bytes());
        encoded.extend_from_slice(&self.timestamp.timestamp().to_be_bytes());
        encoded.push(if self.acknowledgment_required { 1 } else { 0 });
        
        // Payload length and data
        encoded.extend_from_slice(&(self.payload.len() as u16).to_be_bytes());
        encoded.extend_from_slice(&self.payload);
        
        encoded
    }

    pub fn decode(data: &[u8]) -> Option<Self> {
        if data.len() < 19 { // Minimum frame size
            return None;
        }

        let mut offset = 0;
        
        let frame_type = match data[offset] {
            0x01 => FrameType::Telemetry,
            0x02 => FrameType::Command,
            0x03 => FrameType::Acknowledgment,
            0x04 => FrameType::Emergency,
            0x05 => FrameType::Beacon,
            0x06 => FrameType::FileTransfer,
            0x07 => FrameType::TimeSync,
            _ => return None,
        };
        offset += 1;

        let source_id = u32::from_be_bytes([data[offset], data[offset+1], data[offset+2], data[offset+3]]);
        offset += 4;

        let destination_id = u32::from_be_bytes([data[offset], data[offset+1], data[offset+2], data[offset+3]]);
        offset += 4;

        let sequence_number = u16::from_be_bytes([data[offset], data[offset+1]]);
        offset += 2;

        let timestamp_secs = i64::from_be_bytes([
            data[offset], data[offset+1], data[offset+2], data[offset+3],
            data[offset+4], data[offset+5], data[offset+6], data[offset+7],
        ]);
        let timestamp = DateTime::from_timestamp(timestamp_secs, 0)?;
        offset += 8;

        let acknowledgment_required = data[offset] == 1;
        offset += 1;

        let payload_len = u16::from_be_bytes([data[offset], data[offset+1]]) as usize;
        offset += 2;

        if offset + payload_len > data.len() {
            return None;
        }

        let payload = data[offset..offset + payload_len].to_vec();

        Some(Self {
            frame_type,
            payload,
            timestamp,
            source_id,
            destination_id,
            sequence_number,
            acknowledgment_required,
        })
    }
}

impl CubeSatProtocol {
    pub fn new(satellite_id: u32) -> Self {
        Self {
            satellite_id,
            mission_config: None,
            system_state: SystemState {
                power_level: 1.0,
                temperature: 20.0,
                attitude: (0.0, 0.0, 0.0),
                position: OrbitalPosition {
                    latitude: 0.0,
                    longitude: 0.0,
                    altitude: 400.0,
                    velocity: (7.66, 0.0, 0.0),
                },
                system_health: 1.0,
                uptime: Duration::zero(),
                last_updated: Utc::now(),
            },
            command_queue: Vec::new(),
            telemetry_buffer: Vec::new(),
            beacon_counter: 0,
            last_ground_contact: None,
        }
    }

    pub fn configure_mission(&mut self, config: MissionConfig) -> Result<(), String> {
        if config.satellite_id != self.satellite_id {
            return Err("Mission config satellite ID mismatch".to_string());
        }

        self.mission_config = Some(config);
        info!("Configured mission for satellite {}", self.satellite_id);
        Ok(())
    }

    pub fn execute_command(&mut self, command: CubeSatCommand) -> Result<(), String> {
        info!("Executing command {} of type {:?}", command.command_id, command.command_type);

        match command.command_type {
            CommandType::SystemReboot => {
                self.system_state.uptime = Duration::zero();
                self.system_state.last_updated = Utc::now();
                info!("System rebooted");
            },
            CommandType::PayloadActivate => {
                // Simulate payload activation
                self.system_state.power_level -= 0.1; // Increased power consumption
                info!("Payload activated");
            },
            CommandType::PayloadDeactivate => {
                self.system_state.power_level += 0.05; // Reduced power consumption
                info!("Payload deactivated");
            },
            CommandType::AttitudeControl => {
                if let (Some(roll), Some(pitch), Some(yaw)) = (
                    command.parameters.get("roll").and_then(|s| s.parse::<f64>().ok()),
                    command.parameters.get("pitch").and_then(|s| s.parse::<f64>().ok()),
                    command.parameters.get("yaw").and_then(|s| s.parse::<f64>().ok()),
                ) {
                    self.system_state.attitude = (roll, pitch, yaw);
                    info!("Attitude adjusted to ({:.2}, {:.2}, {:.2})", roll, pitch, yaw);
                }
            },
            CommandType::PowerManagement => {
                if let Some(mode) = command.parameters.get("mode") {
                    match mode.as_str() {
                        "low_power" => {
                            self.system_state.power_level = self.system_state.power_level.max(0.3);
                            info!("Entered low power mode");
                        },
                        "normal" => {
                            info!("Entered normal power mode");
                        },
                        _ => return Err("Unknown power mode".to_string()),
                    }
                }
            },
            CommandType::EmergencyMode => {
                self.system_state.power_level = 0.2; // Minimal power
                warn!("Entered emergency mode");
            },
            _ => {
                debug!("Command type {:?} not fully implemented", command.command_type);
            }
        }

        Ok(())
    }

    pub fn generate_telemetry(&mut self) -> Vec<TelemetryData> {
        let now = Utc::now();
        let mut telemetry = Vec::new();

        // System health telemetry
        telemetry.push(TelemetryData {
            timestamp: now,
            source_node: self.satellite_id,
            data_type: TelemetryType::SystemHealth,
            value: TelemetryValue::Float(self.system_state.system_health),
            quality: 0.95,
            sequence_number: self.telemetry_buffer.len() as u64,
        });

        // Power status telemetry
        telemetry.push(TelemetryData {
            timestamp: now,
            source_node: self.satellite_id,
            data_type: TelemetryType::PowerStatus,
            value: TelemetryValue::Float(self.system_state.power_level * 100.0),
            quality: 0.98,
            sequence_number: self.telemetry_buffer.len() as u64 + 1,
        });

        // Temperature telemetry
        telemetry.push(TelemetryData {
            timestamp: now,
            source_node: self.satellite_id,
            data_type: TelemetryType::Temperature,
            value: TelemetryValue::Float(self.system_state.temperature),
            quality: 0.92,
            sequence_number: self.telemetry_buffer.len() as u64 + 2,
        });

        // Attitude telemetry
        telemetry.push(TelemetryData {
            timestamp: now,
            source_node: self.satellite_id,
            data_type: TelemetryType::Attitude,
            value: TelemetryValue::Vector3D(
                self.system_state.attitude.0,
                self.system_state.attitude.1,
                self.system_state.attitude.2,
            ),
            quality: 0.90,
            sequence_number: self.telemetry_buffer.len() as u64 + 3,
        });

        // Orbital position telemetry
        telemetry.push(TelemetryData {
            timestamp: now,
            source_node: self.satellite_id,
            data_type: TelemetryType::OrbitPosition,
            value: TelemetryValue::Vector3D(
                self.system_state.position.latitude,
                self.system_state.position.longitude,
                self.system_state.position.altitude,
            ),
            quality: 0.88,
            sequence_number: self.telemetry_buffer.len() as u64 + 4,
        });

        // Add to buffer
        self.telemetry_buffer.extend(telemetry.clone());

        // Maintain buffer size
        if self.telemetry_buffer.len() > 1000 {
            self.telemetry_buffer.drain(0..100);
        }

        telemetry
    }

    pub fn generate_beacon(&mut self) -> CubeSatFrame {
        self.beacon_counter += 1;
        
        let beacon_data = format!(
            "BEACON:{};PWR:{:.1};TEMP:{:.1};HEALTH:{:.2};UPTIME:{}",
            self.beacon_counter,
            self.system_state.power_level * 100.0,
            self.system_state.temperature,
            self.system_state.system_health,
            self.system_state.uptime.num_seconds()
        );

        CubeSatFrame::new(
            FrameType::Beacon,
            beacon_data.into_bytes(),
            self.satellite_id,
            0, // Broadcast
        )
    }

    pub fn update_system_state(&mut self, time_delta: Duration) {
        // Simulate system evolution
        self.system_state.uptime += time_delta;
        self.system_state.last_updated = Utc::now();

        // Simulate power consumption and solar charging
        let power_consumption = 0.001 * time_delta.num_seconds() as f64 / 3600.0; // 0.1% per hour
        let solar_charging = if self.is_in_sunlight() { 0.002 } else { 0.0 };
        
        self.system_state.power_level = (self.system_state.power_level - power_consumption + solar_charging)
            .max(0.0).min(1.0);

        // Simulate temperature variations
        let temp_variation = (rand::random::<f64>() - 0.5) * 2.0; // ±1°C
        self.system_state.temperature += temp_variation;

        // Update system health based on power and temperature
        let power_health = if self.system_state.power_level > 0.5 { 1.0 } else { self.system_state.power_level * 2.0 };
        let temp_health = if self.system_state.temperature > -20.0 && self.system_state.temperature < 60.0 { 1.0 } else { 0.5 };
        
        self.system_state.system_health = (power_health * temp_health).min(1.0);

        // Update orbital position (simplified)
        let orbital_period = 90.0 * 60.0; // 90 minutes in seconds
        let angular_velocity = 360.0 / orbital_period; // degrees per second
        let delta_longitude = angular_velocity * time_delta.num_seconds() as f64;
        
        self.system_state.position.longitude = (self.system_state.position.longitude + delta_longitude) % 360.0;
        if self.system_state.position.longitude > 180.0 {
            self.system_state.position.longitude -= 360.0;
        }
    }

    fn is_in_sunlight(&self) -> bool {
        // Simplified sunlight calculation based on orbital position
        // In reality, this would consider Earth's shadow
        let sun_angle = (self.system_state.position.longitude + 180.0) % 360.0;
        sun_angle < 180.0 // Simplified: half the orbit is in sunlight
    }

    pub fn get_system_state(&self) -> &SystemState {
        &self.system_state
    }

    pub fn get_telemetry_buffer(&self) -> &[TelemetryData] {
        &self.telemetry_buffer
    }
}

impl MissionControl {
    pub fn new() -> Self {
        Self {
            satellites: HashMap::new(),
            mission_timeline: Vec::new(),
            ground_contacts: Vec::new(),
            emergency_procedures: HashMap::new(),
            statistics: MissionStatistics::default(),
        }
    }

    pub fn add_satellite(&mut self, satellite: CubeSatProtocol) {
        let satellite_id = satellite.satellite_id;
        self.satellites.insert(satellite_id, satellite);
        self.statistics.total_satellites += 1;
        self.statistics.active_satellites += 1;
        
        info!("Added satellite {} to mission control", satellite_id);
    }

    pub fn send_command_to_satellite(&mut self, satellite_id: u32, command: CubeSatCommand) -> Result<(), String> {
        if let Some(satellite) = self.satellites.get_mut(&satellite_id) {
            satellite.execute_command(command)?;
            self.statistics.total_commands_executed += 1;
            Ok(())
        } else {
            Err(format!("Satellite {} not found", satellite_id))
        }
    }

    pub fn collect_telemetry(&mut self) -> HashMap<u32, Vec<TelemetryData>> {
        let mut all_telemetry = HashMap::new();
        
        for (satellite_id, satellite) in &mut self.satellites {
            let telemetry = satellite.generate_telemetry();
            all_telemetry.insert(*satellite_id, telemetry);
        }
        
        all_telemetry
    }

    pub fn update_all_satellites(&mut self, time_delta: Duration) {
        for satellite in self.satellites.values_mut() {
            satellite.update_system_state(time_delta);
        }
        
        self.statistics.mission_uptime += time_delta;
    }

    pub fn get_statistics(&self) -> &MissionStatistics {
        &self.statistics
    }

    pub fn get_satellite_status(&self, satellite_id: u32) -> Option<&SystemState> {
        self.satellites.get(&satellite_id).map(|s| s.get_system_state())
    }
}

impl Default for MissionControl {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for MissionConfig {
    fn default() -> Self {
        Self {
            mission_id: "DEFAULT-MISSION".to_string(),
            satellite_id: 1,
            launch_date: Utc::now(),
            mission_duration: Duration::days(365),
            orbital_parameters: OrbitalParameters {
                semi_major_axis: 6771.0, // 400km altitude
                eccentricity: 0.0,
                inclination: 51.6,
                argument_of_perigee: 0.0,
                longitude_of_ascending_node: 0.0,
                mean_anomaly: 0.0,
            },
            power_budget: PowerBudget {
                solar_panel_power: 10.0,
                battery_capacity: 20.0,
                system_power_consumption: 2.0,
                communication_power: 3.0,
                payload_power: 5.0,
            },
            communication_schedule: CommunicationSchedule {
                beacon_interval: Duration::minutes(1),
                telemetry_interval: Duration::minutes(5),
                ground_contact_windows: Vec::new(),
            },
            payload_config: PayloadConfig {
                payload_type: PayloadType::Camera,
                operating_modes: vec![
                    OperatingMode {
                        mode_name: "Standby".to_string(),
                        power_consumption: 0.5,
                        data_rate: 0.0,
                        duration_limit: None,
                    },
                    OperatingMode {
                        mode_name: "Active".to_string(),
                        power_consumption: 5.0,
                        data_rate: 1.0,
                        duration_limit: Some(Duration::minutes(30)),
                    },
                ],
                data_collection_schedule: Vec::new(),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cubesat_frame_encoding_decoding() {
        let frame = CubeSatFrame::new(
            FrameType::Telemetry,
            vec![1, 2, 3, 4, 5],
            1,
            2
        );
        
        let encoded = frame.encode();
        let decoded = CubeSatFrame::decode(&encoded).unwrap();
        
        assert_eq!(frame.frame_type, decoded.frame_type);
        assert_eq!(frame.payload, decoded.payload);
        assert_eq!(frame.source_id, decoded.source_id);
        assert_eq!(frame.destination_id, decoded.destination_id);
    }

    #[test]
    fn test_cubesat_protocol_creation() {
        let protocol = CubeSatProtocol::new(1);
        assert_eq!(protocol.satellite_id, 1);
        assert_eq!(protocol.system_state.power_level, 1.0);
    }

    #[test]
    fn test_mission_config() {
        let mut protocol = CubeSatProtocol::new(1);
        let config = MissionConfig::default();
        
        assert!(protocol.configure_mission(config).is_ok());
        assert!(protocol.mission_config.is_some());
    }

    #[test]
    fn test_command_execution() {
        let mut protocol = CubeSatProtocol::new(1);
        
        let command = CubeSatCommand {
            command_id: 1,
            command_type: CommandType::SystemReboot,
            parameters: HashMap::new(),
            scheduled_execution: None,
            priority: 5,
            status: CommandStatus::Queued,
        };
        
        assert!(protocol.execute_command(command).is_ok());
    }

    #[test]
    fn test_telemetry_generation() {
        let mut protocol = CubeSatProtocol::new(1);
        let telemetry = protocol.generate_telemetry();
        
        assert!(!telemetry.is_empty());
        assert_eq!(telemetry[0].source_node, 1);
    }

    #[test]
    fn test_beacon_generation() {
        let mut protocol = CubeSatProtocol::new(1);
        let beacon = protocol.generate_beacon();
        
        assert_eq!(beacon.frame_type, FrameType::Beacon);
        assert_eq!(beacon.source_id, 1);
        assert_eq!(beacon.destination_id, 0); // Broadcast
    }

    #[test]
    fn test_mission_control() {
        let mut mission_control = MissionControl::new();
        let satellite = CubeSatProtocol::new(1);
        
        mission_control.add_satellite(satellite);
        assert_eq!(mission_control.statistics.total_satellites, 1);
        
        let telemetry = mission_control.collect_telemetry();
        assert!(telemetry.contains_key(&1));
    }

    #[test]
    fn test_system_state_update() {
        let mut protocol = CubeSatProtocol::new(1);
        let initial_uptime = protocol.system_state.uptime;
        
        protocol.update_system_state(Duration::minutes(10));
        
        assert!(protocol.system_state.uptime > initial_uptime);
    }
}
