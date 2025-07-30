// Real-time telemetry processing and mission timeline synchronization
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, VecDeque};
use chrono::{DateTime, Utc, Duration};
use log::{info, warn, error, debug};

/// Telemetry data types for CubeSat systems
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum TelemetryType {
    SystemHealth,
    PowerStatus,
    OrbitPosition,
    Communication,
    Payload,
    Temperature,
    Attitude,
    Custom(String),
}

/// Individual telemetry data point
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TelemetryData {
    pub timestamp: DateTime<Utc>,
    pub source_node: u32,
    pub data_type: TelemetryType,
    pub value: TelemetryValue,
    pub quality: f64,  // 0.0 to 1.0
    pub sequence_number: u64,
}

/// Telemetry value variants
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TelemetryValue {
    Float(f64),
    Integer(i64),
    Boolean(bool),
    String(String),
    Vector3D(f64, f64, f64),
    Array(Vec<f64>),
}

/// Telemetry packet for transmission
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TelemetryPacket {
    pub packet_id: u32,
    pub source_node: u32,
    pub timestamp: DateTime<Utc>,
    pub data_points: Vec<TelemetryData>,
    pub compression_type: CompressionType,
    pub priority: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CompressionType {
    None,
    LZ4,
    Gzip,
    Custom,
}

/// Mission timeline event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MissionEvent {
    pub event_id: u32,
    pub event_type: EventType,
    pub scheduled_time: DateTime<Utc>,
    pub duration: Duration,
    pub priority: u8,
    pub parameters: HashMap<String, String>,
    pub status: EventStatus,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum EventType {
    DataCollection,
    GroundContact,
    OrbitManeuver,
    PayloadOperation,
    SystemMaintenance,
    Emergency,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum EventStatus {
    Scheduled,
    InProgress,
    Completed,
    Failed,
    Cancelled,
}

/// Real-time telemetry processor
pub struct TelemetryProcessor {
    telemetry_buffer: VecDeque<TelemetryData>,
    mission_timeline: Vec<MissionEvent>,
    data_aggregators: HashMap<TelemetryType, DataAggregator>,
    statistics: TelemetryStatistics,
    alert_thresholds: HashMap<TelemetryType, AlertThreshold>,
    downlink_queue: VecDeque<TelemetryPacket>,
}

/// Data aggregator for telemetry analysis
#[derive(Debug, Clone)]
pub struct DataAggregator {
    pub data_type: TelemetryType,
    pub window_size: Duration,
    pub samples: VecDeque<TelemetryData>,
    pub min_value: f64,
    pub max_value: f64,
    pub average: f64,
    pub last_updated: DateTime<Utc>,
}

/// Alert threshold configuration
#[derive(Debug, Clone)]
pub struct AlertThreshold {
    pub min_value: Option<f64>,
    pub max_value: Option<f64>,
    pub rate_of_change_limit: Option<f64>,
    pub alert_level: AlertLevel,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AlertLevel {
    Info,
    Warning,
    Critical,
    Emergency,
}

/// Telemetry processing statistics
#[derive(Debug, Clone, Default)]
pub struct TelemetryStatistics {
    pub data_points_processed: u64,
    pub packets_transmitted: u64,
    pub alerts_generated: u64,
    pub compression_ratio: f64,
    pub average_latency: Duration,
    pub data_quality_score: f64,
}

impl TelemetryProcessor {
    pub fn new() -> Self {
        Self {
            telemetry_buffer: VecDeque::new(),
            mission_timeline: Vec::new(),
            data_aggregators: HashMap::new(),
            statistics: TelemetryStatistics::default(),
            alert_thresholds: HashMap::new(),
            downlink_queue: VecDeque::new(),
        }
    }

    /// Initialize telemetry processing with default configurations
    pub fn initialize(&mut self) -> Result<(), String> {
        info!("Initializing telemetry processor");
        
        // Set up default data aggregators
        self.setup_default_aggregators();
        
        // Configure default alert thresholds
        self.setup_default_thresholds();
        
        // Initialize mission timeline
        self.initialize_mission_timeline();
        
        info!("Telemetry processor initialized successfully");
        Ok(())
    }

    /// Set up default data aggregators for common telemetry types
    fn setup_default_aggregators(&mut self) {
        let telemetry_types = vec![
            TelemetryType::SystemHealth,
            TelemetryType::PowerStatus,
            TelemetryType::Temperature,
            TelemetryType::Communication,
        ];

        for data_type in telemetry_types {
            let aggregator = DataAggregator {
                data_type: data_type.clone(),
                window_size: Duration::minutes(10),
                samples: VecDeque::new(),
                min_value: f64::INFINITY,
                max_value: f64::NEG_INFINITY,
                average: 0.0,
                last_updated: Utc::now(),
            };
            self.data_aggregators.insert(data_type, aggregator);
        }
    }

    /// Configure default alert thresholds
    fn setup_default_thresholds(&mut self) {
        // Power status thresholds
        self.alert_thresholds.insert(
            TelemetryType::PowerStatus,
            AlertThreshold {
                min_value: Some(20.0), // 20% battery minimum
                max_value: None,
                rate_of_change_limit: Some(-5.0), // -5% per minute
                alert_level: AlertLevel::Warning,
            }
        );

        // Temperature thresholds
        self.alert_thresholds.insert(
            TelemetryType::Temperature,
            AlertThreshold {
                min_value: Some(-40.0), // -40°C minimum
                max_value: Some(85.0),  // 85°C maximum
                rate_of_change_limit: Some(10.0), // 10°C per minute
                alert_level: AlertLevel::Critical,
            }
        );

        // System health thresholds
        self.alert_thresholds.insert(
            TelemetryType::SystemHealth,
            AlertThreshold {
                min_value: Some(0.7), // 70% health minimum
                max_value: None,
                rate_of_change_limit: Some(-0.1), // -10% per minute
                alert_level: AlertLevel::Warning,
            }
        );
    }

    /// Initialize mission timeline with default events
    fn initialize_mission_timeline(&mut self) {
        let now = Utc::now();
        
        // Schedule regular ground contacts
        for i in 0..24 {
            let contact_time = now + Duration::hours(i * 4); // Every 4 hours
            let event = MissionEvent {
                event_id: i as u32,
                event_type: EventType::GroundContact,
                scheduled_time: contact_time,
                duration: Duration::minutes(10),
                priority: 2,
                parameters: HashMap::new(),
                status: EventStatus::Scheduled,
            };
            self.mission_timeline.push(event);
        }

        // Schedule data collection events
        for i in 0..48 {
            let collection_time = now + Duration::hours(i * 2); // Every 2 hours
            let event = MissionEvent {
                event_id: (100 + i) as u32,
                event_type: EventType::DataCollection,
                scheduled_time: collection_time,
                duration: Duration::minutes(30),
                priority: 1,
                parameters: HashMap::new(),
                status: EventStatus::Scheduled,
            };
            self.mission_timeline.push(event);
        }

        // Sort timeline by scheduled time
        self.mission_timeline.sort_by_key(|e| e.scheduled_time);
        
        info!("Initialized mission timeline with {} events", self.mission_timeline.len());
    }

    /// Process incoming telemetry data
    pub fn process_telemetry(&mut self, data: TelemetryData) -> Result<(), String> {
        debug!("Processing telemetry data: {:?}", data.data_type);
        
        // Validate data quality
        if data.quality < 0.5 {
            warn!("Low quality telemetry data received (quality: {:.2})", data.quality);
        }
        
        // Check for alerts
        self.check_alerts(&data)?;
        
        // Update data aggregator
        self.update_aggregator(&data)?;
        
        // Add to buffer
        self.telemetry_buffer.push_back(data.clone());
        
        // Maintain buffer size
        if self.telemetry_buffer.len() > 10000 {
            self.telemetry_buffer.pop_front();
        }
        
        // Update statistics
        self.statistics.data_points_processed += 1;
        self.statistics.data_quality_score = 
            (self.statistics.data_quality_score * (self.statistics.data_points_processed - 1) as f64 + data.quality) 
            / self.statistics.data_points_processed as f64;
        
        Ok(())
    }

    /// Check telemetry data against alert thresholds
    fn check_alerts(&mut self, data: &TelemetryData) -> Result<(), String> {
        if let Some(threshold) = self.alert_thresholds.get(&data.data_type).cloned() {
            let value = match &data.value {
                TelemetryValue::Float(v) => *v,
                TelemetryValue::Integer(v) => *v as f64,
                _ => return Ok(()), // Skip non-numeric values
            };

            let mut alert_triggered = false;
            let mut alert_message = String::new();

            // Check minimum threshold
            if let Some(min_val) = threshold.min_value {
                if value < min_val {
                    alert_triggered = true;
                    alert_message.push_str(&format!("Value {} below minimum {}", value, min_val));
                }
            }

            // Check maximum threshold
            if let Some(max_val) = threshold.max_value {
                if value > max_val {
                    alert_triggered = true;
                    alert_message.push_str(&format!("Value {} above maximum {}", value, max_val));
                }
            }

            // Check rate of change
            if let Some(rate_limit) = threshold.rate_of_change_limit {
                if let Some(aggregator) = self.data_aggregators.get(&data.data_type) {
                    if let Some(last_sample) = aggregator.samples.back() {
                        let time_diff = data.timestamp.signed_duration_since(last_sample.timestamp);
                        if time_diff > Duration::zero() {
                            let last_value = match &last_sample.value {
                                TelemetryValue::Float(v) => *v,
                                TelemetryValue::Integer(v) => *v as f64,
                                _ => return Ok(()),
                            };
                            
                            let rate = (value - last_value) / time_diff.num_seconds() as f64 * 60.0; // per minute
                            if rate.abs() > rate_limit.abs() {
                                alert_triggered = true;
                                alert_message.push_str(&format!("Rate of change {} exceeds limit {}", rate, rate_limit));
                            }
                        }
                    }
                }
            }

            if alert_triggered {
                self.generate_alert(data, &threshold.alert_level, &alert_message)?;
            }
        }

        Ok(())
    }

    /// Generate alert for telemetry anomaly
    fn generate_alert(&mut self, data: &TelemetryData, level: &AlertLevel, message: &str) -> Result<(), String> {
        match level {
            AlertLevel::Info => info!("Telemetry alert: {} - {}", data.data_type.type_name(), message),
            AlertLevel::Warning => warn!("Telemetry warning: {} - {}", data.data_type.type_name(), message),
            AlertLevel::Critical => error!("Telemetry critical: {} - {}", data.data_type.type_name(), message),
            AlertLevel::Emergency => {
                error!("TELEMETRY EMERGENCY: {} - {}", data.data_type.type_name(), message);
                // In a real system, this would trigger emergency protocols
            }
        }

        self.statistics.alerts_generated += 1;
        Ok(())
    }

    /// Update data aggregator with new telemetry
    fn update_aggregator(&mut self, data: &TelemetryData) -> Result<(), String> {
        if let Some(aggregator) = self.data_aggregators.get_mut(&data.data_type) {
            // Add new sample
            aggregator.samples.push_back(data.clone());
            aggregator.last_updated = data.timestamp;

            // Remove old samples outside the window
            let cutoff_time = data.timestamp - aggregator.window_size;
            while let Some(front) = aggregator.samples.front() {
                if front.timestamp < cutoff_time {
                    aggregator.samples.pop_front();
                } else {
                    break;
                }
            }

            // Update statistics
            if let TelemetryValue::Float(value) = &data.value {
                aggregator.min_value = aggregator.min_value.min(*value);
                aggregator.max_value = aggregator.max_value.max(*value);
                
                // Calculate average
                let sum: f64 = aggregator.samples.iter()
                    .filter_map(|s| match &s.value {
                        TelemetryValue::Float(v) => Some(*v),
                        TelemetryValue::Integer(v) => Some(*v as f64),
                        _ => None,
                    })
                    .sum();
                
                let count = aggregator.samples.len() as f64;
                if count > 0.0 {
                    aggregator.average = sum / count;
                }
            }
        }

        Ok(())
    }

    /// Create telemetry packet for downlink
    pub fn create_telemetry_packet(&mut self, node_id: u32, max_data_points: usize) -> Result<TelemetryPacket, String> {
        let mut data_points = Vec::new();
        
        // Collect recent telemetry data
        let mut count = 0;
        while let Some(data) = self.telemetry_buffer.pop_front() {
            if data.source_node == node_id {
                data_points.push(data);
                count += 1;
                if count >= max_data_points {
                    break;
                }
            }
        }

        if data_points.is_empty() {
            return Err("No telemetry data available".to_string());
        }

        let packet = TelemetryPacket {
            packet_id: rand::random::<u32>(),
            source_node: node_id,
            timestamp: Utc::now(),
            data_points,
            compression_type: CompressionType::LZ4,
            priority: 1,
        };

        info!("Created telemetry packet with {} data points", packet.data_points.len());
        Ok(packet)
    }

    /// Log transmission event
    pub fn log_transmission(&mut self, destination: u32, bytes_sent: usize) {
        debug!("Logged transmission to node {}: {} bytes", destination, bytes_sent);
        // Update statistics would go here
    }

    /// Log reception event
    pub fn log_reception(&mut self, bytes_received: usize) {
        debug!("Logged reception: {} bytes", bytes_received);
        // Update statistics would go here
    }

    /// Get current mission events
    pub fn get_current_events(&self) -> Vec<&MissionEvent> {
        let now = Utc::now();
        let window_start = now - Duration::minutes(30);
        let window_end = now + Duration::hours(2);

        self.mission_timeline.iter()
            .filter(|event| event.scheduled_time >= window_start && event.scheduled_time <= window_end)
            .collect()
    }

    /// Update mission event status
    pub fn update_event_status(&mut self, event_id: u32, status: EventStatus) -> Result<(), String> {
        if let Some(event) = self.mission_timeline.iter_mut().find(|e| e.event_id == event_id) {
            event.status = status.clone();
            info!("Updated event {} status to {:?}", event_id, status);
            Ok(())
        } else {
            Err(format!("Event {} not found", event_id))
        }
    }

    /// Get telemetry statistics
    pub fn get_statistics(&self) -> &TelemetryStatistics {
        &self.statistics
    }

    /// Get aggregated data for a specific telemetry type
    pub fn get_aggregated_data(&self, data_type: &TelemetryType) -> Option<&DataAggregator> {
        self.data_aggregators.get(data_type)
    }

    /// Compress telemetry data for efficient transmission
    pub fn compress_telemetry_data(&self, data: &[u8]) -> Result<Vec<u8>, String> {
        // Simplified compression - in production, use proper compression algorithms
        let mut compressed = Vec::new();
        
        // Simple run-length encoding for demonstration
        let mut i = 0;
        while i < data.len() {
            let current_byte = data[i];
            let mut count = 1u8;
            
            while (i + count as usize) < data.len() && 
                  data[i + count as usize] == current_byte && 
                  count < 255 {
                count += 1;
            }
            
            compressed.push(count);
            compressed.push(current_byte);
            i += count as usize;
        }
        
        info!("Compressed {} bytes to {} bytes (ratio: {:.2})", 
              data.len(), compressed.len(), 
              compressed.len() as f64 / data.len() as f64);
        
        Ok(compressed)
    }

    /// Synchronize mission timeline with ground station
    pub fn synchronize_timeline(&mut self, ground_timeline: Vec<MissionEvent>) -> Result<(), String> {
        // Merge ground station timeline with local timeline
        for ground_event in ground_timeline {
            // Check if event already exists
            if let Some(local_event) = self.mission_timeline.iter_mut()
                .find(|e| e.event_id == ground_event.event_id) {
                // Update existing event
                *local_event = ground_event;
            } else {
                // Add new event
                self.mission_timeline.push(ground_event);
            }
        }

        // Sort timeline by scheduled time
        self.mission_timeline.sort_by_key(|e| e.scheduled_time);
        
        info!("Synchronized mission timeline with {} events", self.mission_timeline.len());
        Ok(())
    }
}

impl Default for TelemetryProcessor {
    fn default() -> Self {
        Self::new()
    }
}

impl TelemetryType {
    fn type_name(&self) -> &str {
        match self {
            TelemetryType::SystemHealth => "SystemHealth",
            TelemetryType::PowerStatus => "PowerStatus",
            TelemetryType::OrbitPosition => "OrbitPosition",
            TelemetryType::Communication => "Communication",
            TelemetryType::Payload => "Payload",
            TelemetryType::Temperature => "Temperature",
            TelemetryType::Attitude => "Attitude",
            TelemetryType::Custom(name) => name,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_telemetry_processor_creation() {
        let processor = TelemetryProcessor::new();
        assert_eq!(processor.telemetry_buffer.len(), 0);
        assert_eq!(processor.mission_timeline.len(), 0);
    }

    #[test]
    fn test_telemetry_data_processing() {
        let mut processor = TelemetryProcessor::new();
        processor.initialize().unwrap();
        
        let data = TelemetryData {
            timestamp: Utc::now(),
            source_node: 1,
            data_type: TelemetryType::Temperature,
            value: TelemetryValue::Float(25.0),
            quality: 0.95,
            sequence_number: 1,
        };
        
        assert!(processor.process_telemetry(data).is_ok());
        assert_eq!(processor.telemetry_buffer.len(), 1);
    }

    #[test]
    fn test_mission_timeline_initialization() {
        let mut processor = TelemetryProcessor::new();
        processor.initialize().unwrap();
        
        assert!(!processor.mission_timeline.is_empty());
        
        // Check that events are sorted by time
        for i in 1..processor.mission_timeline.len() {
            assert!(processor.mission_timeline[i-1].scheduled_time <= processor.mission_timeline[i].scheduled_time);
        }
    }

    #[test]
    fn test_telemetry_packet_creation() {
        let mut processor = TelemetryProcessor::new();
        processor.initialize().unwrap();
        
        // Add some test data
        let data = TelemetryData {
            timestamp: Utc::now(),
            source_node: 1,
            data_type: TelemetryType::SystemHealth,
            value: TelemetryValue::Float(0.85),
            quality: 0.9,
            sequence_number: 1,
        };
        
        processor.process_telemetry(data).unwrap();
        
        let packet = processor.create_telemetry_packet(1, 10).unwrap();
        assert_eq!(packet.source_node, 1);
        assert!(!packet.data_points.is_empty());
    }

    #[test]
    fn test_alert_generation() {
        let mut processor = TelemetryProcessor::new();
        processor.initialize().unwrap();
        
        // Create data that should trigger an alert (high temperature)
        let data = TelemetryData {
            timestamp: Utc::now(),
            source_node: 1,
            data_type: TelemetryType::Temperature,
            value: TelemetryValue::Float(100.0), // Above 85°C threshold
            quality: 0.9,
            sequence_number: 1,
        };
        
        assert!(processor.process_telemetry(data).is_ok());
        assert!(processor.statistics.alerts_generated > 0);
    }

    #[test]
    fn test_data_compression() {
        let processor = TelemetryProcessor::new();
        let test_data = vec![1, 1, 1, 2, 2, 3, 3, 3, 3];
        
        let compressed = processor.compress_telemetry_data(&test_data).unwrap();
        assert!(compressed.len() < test_data.len());
    }
}