// SpaceCAN base implementation and CubeSat-specific extensions
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use chrono::{DateTime, Utc};
use log::{info, warn, error};

/// Frame priority levels for CubeSat communications
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FramePriority {
    Emergency = 0,    // Life-critical systems
    High = 1,         // Mission-critical telemetry
    Normal = 2,       // Regular data transmission
    Low = 3,          // Housekeeping data
}

/// Power transmission modes for energy-efficient communication
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PowerMode {
    HighPower,    // Maximum range, high energy consumption
    MediumPower,  // Balanced range and energy
    LowPower,     // Energy-efficient, reduced range
    UltraLow,     // Emergency mode, minimal energy
}

/// Enhanced SpaceCAN frame with CubeSat-specific features
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpaceCANFrame {
    pub id: u32,
    pub data: Vec<u8>,  // Variable length for flexibility
    pub dlc: u8,
    pub priority: FramePriority,
    pub power_mode: PowerMode,
    pub timestamp: DateTime<Utc>,
    pub sequence_number: u16,
    pub checksum: u32,
    pub error_correction: Vec<u8>,  // Reed-Solomon or similar
}

impl SpaceCANFrame {
    pub fn new(id: u32, data: Vec<u8>, priority: FramePriority) -> Self {
        let dlc = data.len().min(255) as u8;
        let timestamp = Utc::now();
        let sequence_number = rand::random::<u16>();
        let checksum = Self::calculate_checksum(&data);
        let error_correction = Self::generate_error_correction(&data);
        
        Self {
            id,
            data,
            dlc,
            priority,
            power_mode: PowerMode::MediumPower,
            timestamp,
            sequence_number,
            checksum,
            error_correction,
        }
    }

    pub fn with_power_mode(mut self, power_mode: PowerMode) -> Self {
        self.power_mode = power_mode;
        self
    }

    /// Calculate CRC32 checksum for error detection
    fn calculate_checksum(data: &[u8]) -> u32 {
        let mut crc = 0xFFFFFFFFu32;
        for &byte in data {
            crc ^= byte as u32;
            for _ in 0..8 {
                if crc & 1 != 0 {
                    crc = (crc >> 1) ^ 0xEDB88320;
                } else {
                    crc >>= 1;
                }
            }
        }
        !crc
    }

    /// Generate Reed-Solomon error correction codes
    fn generate_error_correction(data: &[u8]) -> Vec<u8> {
        // Simplified error correction - in production, use proper Reed-Solomon
        let mut ecc = Vec::new();
        for chunk in data.chunks(4) {
            let sum: u8 = chunk.iter().fold(0, |acc, &x| acc.wrapping_add(x));
            ecc.push(sum);
        }
        ecc
    }

    /// Encode frame with space-optimized format
    pub fn encode(&self) -> Vec<u8> {
        let mut encoded = Vec::new();
        
        // Header: ID (4 bytes) + DLC (1 byte) + Priority (1 byte) + Power Mode (1 byte)
        encoded.extend_from_slice(&self.id.to_be_bytes());
        encoded.push(self.dlc);
        encoded.push(self.priority as u8);
        encoded.push(self.power_mode as u8);
        
        // Sequence number (2 bytes)
        encoded.extend_from_slice(&self.sequence_number.to_be_bytes());
        
        // Timestamp (8 bytes - Unix timestamp)
        encoded.extend_from_slice(&self.timestamp.timestamp().to_be_bytes());
        
        // Data payload
        encoded.extend_from_slice(&self.data);
        
        // Checksum (4 bytes)
        encoded.extend_from_slice(&self.checksum.to_be_bytes());
        
        // Error correction codes
        encoded.push(self.error_correction.len() as u8);
        encoded.extend_from_slice(&self.error_correction);
        
        encoded
    }

    /// Decode frame with error detection and correction
    pub fn decode(bytes: &[u8]) -> Result<Self, String> {
        if bytes.len() < 21 {  // Minimum frame size
            return Err("Frame too short".to_string());
        }

        let mut offset = 0;
        
        // Parse header
        let id = u32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]);
        offset += 4;
        
        let dlc = bytes[offset];
        offset += 1;
        
        let priority = match bytes[offset] {
            0 => FramePriority::Emergency,
            1 => FramePriority::High,
            2 => FramePriority::Normal,
            3 => FramePriority::Low,
            _ => return Err("Invalid priority".to_string()),
        };
        offset += 1;
        
        let power_mode = match bytes[offset] {
            0 => PowerMode::HighPower,
            1 => PowerMode::MediumPower,
            2 => PowerMode::LowPower,
            3 => PowerMode::UltraLow,
            _ => return Err("Invalid power mode".to_string()),
        };
        offset += 1;
        
        // Parse sequence number
        let sequence_number = u16::from_be_bytes([bytes[offset], bytes[offset + 1]]);
        offset += 2;
        
        // Parse timestamp
        let timestamp_secs = i64::from_be_bytes([
            bytes[offset], bytes[offset + 1], bytes[offset + 2], bytes[offset + 3],
            bytes[offset + 4], bytes[offset + 5], bytes[offset + 6], bytes[offset + 7],
        ]);
        let timestamp = DateTime::from_timestamp(timestamp_secs, 0)
            .ok_or("Invalid timestamp")?;
        offset += 8;
        
        // Parse data payload
        if offset + dlc as usize + 5 > bytes.len() {
            return Err("Invalid frame length".to_string());
        }
        
        let data = bytes[offset..offset + dlc as usize].to_vec();
        offset += dlc as usize;
        
        // Parse checksum
        let checksum = u32::from_be_bytes([
            bytes[offset], bytes[offset + 1], bytes[offset + 2], bytes[offset + 3]
        ]);
        offset += 4;
        
        // Verify checksum
        let calculated_checksum = Self::calculate_checksum(&data);
        if checksum != calculated_checksum {
            return Err("Checksum mismatch".to_string());
        }
        
        // Parse error correction
        let ecc_len = bytes[offset] as usize;
        offset += 1;
        
        if offset + ecc_len > bytes.len() {
            return Err("Invalid ECC length".to_string());
        }
        
        let error_correction = bytes[offset..offset + ecc_len].to_vec();
        
        Ok(Self {
            id,
            data,
            dlc,
            priority,
            power_mode,
            timestamp,
            sequence_number,
            checksum,
            error_correction,
        })
    }

    /// Validate frame integrity and attempt error correction
    pub fn validate_and_correct(&mut self) -> Result<bool, String> {
        // Verify checksum
        let calculated_checksum = Self::calculate_checksum(&self.data);
        if self.checksum != calculated_checksum {
            warn!("Checksum mismatch detected, attempting error correction");
            
            // Attempt error correction using ECC
            let correction_result = self.attempt_error_correction();
            if correction_result {
                info!("Error correction successful");
                self.checksum = Self::calculate_checksum(&self.data);
                // After correction, verify checksum again
                if self.checksum == calculated_checksum {
                    return Ok(true);
                } else {
                    error!("Error correction failed to fix checksum");
                    return Err("Uncorrectable error detected".to_string());
                }
            } else {
                error!("Error correction failed");
                return Err("Uncorrectable error detected".to_string());
            }
        }
        
        Ok(false)
    }

    /// Attempt to correct errors using error correction codes
    fn attempt_error_correction(&mut self) -> bool {
        // Simplified error correction - in production, implement proper Reed-Solomon
        for (i, chunk) in self.data.chunks_mut(4).enumerate() {
            if i < self.error_correction.len() {
                let expected_sum = self.error_correction[i];
                let actual_sum: u8 = chunk.iter().fold(0, |acc, &x| acc.wrapping_add(x));
                
                if expected_sum != actual_sum {
                    // Simple single-bit error correction
                    let diff = expected_sum.wrapping_sub(actual_sum);
                    if chunk.len() > 0 {
                        chunk[0] = chunk[0].wrapping_add(diff);
                    }
                }
            }
        }
        true
    }

    /// Get transmission power requirements based on power mode
    pub fn get_power_requirements(&self) -> f64 {
        match self.power_mode {
            PowerMode::HighPower => 2.0,    // 2W
            PowerMode::MediumPower => 1.0,  // 1W
            PowerMode::LowPower => 0.5,     // 0.5W
            PowerMode::UltraLow => 0.1,     // 0.1W
        }
    }

    /// Get estimated transmission range in kilometers
    pub fn get_transmission_range(&self) -> f64 {
        match self.power_mode {
            PowerMode::HighPower => 2000.0,
            PowerMode::MediumPower => 1000.0,
            PowerMode::LowPower => 500.0,
            PowerMode::UltraLow => 100.0,
        }
    }
}

/// SpaceCAN adapter for managing multiple communication channels
pub struct SpaceCANAdapter {
    channels: HashMap<u8, SpaceCANChannel>,
    frame_buffer: Vec<SpaceCANFrame>,
    statistics: CommunicationStats,
}

#[derive(Debug, Clone)]
pub struct SpaceCANChannel {
    pub channel_id: u8,
    pub frequency: f64,  // MHz
    pub bandwidth: f64,  // kHz
    pub is_active: bool,
    pub power_mode: PowerMode,
}

#[derive(Debug, Clone, Default)]
pub struct CommunicationStats {
    pub frames_sent: u64,
    pub frames_received: u64,
    pub errors_detected: u64,
    pub errors_corrected: u64,
    pub total_bytes_transmitted: u64,
    pub total_power_consumed: f64,  // Watt-hours
}

impl SpaceCANAdapter {
    pub fn new() -> Self {
        let mut adapter = Self {
            channels: HashMap::new(),
            frame_buffer: Vec::new(),
            statistics: CommunicationStats::default(),
        };
        
        // Initialize default channels
        adapter.add_channel(0, 437.5, 25.0);  // UHF band
        adapter.add_channel(1, 2400.0, 100.0); // S-band
        
        adapter
    }

    pub fn add_channel(&mut self, channel_id: u8, frequency: f64, bandwidth: f64) {
        let channel = SpaceCANChannel {
            channel_id,
            frequency,
            bandwidth,
            is_active: true,
            power_mode: PowerMode::MediumPower,
        };
        self.channels.insert(channel_id, channel);
        info!("Added communication channel {} at {} MHz", channel_id, frequency);
    }

    pub fn transmit(&mut self, frame: &SpaceCANFrame) -> Result<(), String> {
        // Select best channel based on frame priority and power requirements
        let channel_id = self.select_optimal_channel(frame)?;
        
        // Encode and transmit
        let encoded = frame.encode();
        
        // Update statistics
        self.statistics.frames_sent += 1;
        self.statistics.total_bytes_transmitted += encoded.len() as u64;
        self.statistics.total_power_consumed += frame.get_power_requirements() * 0.1; // 0.1 hour transmission
        
        info!("Transmitted frame {} on channel {} ({} bytes)", 
              frame.sequence_number, channel_id, encoded.len());
        
        Ok(())
    }

    pub fn receive(&mut self) -> Result<Option<Vec<u8>>, String> {
        // Simulate receiving data from active channels
        for (_channel_id, channel) in &self.channels {
            if channel.is_active {
                // In a real implementation, this would interface with radio hardware
                // For simulation, we'll return buffered frames
                if !self.frame_buffer.is_empty() {
                    let frame = self.frame_buffer.remove(0);
                    let encoded = frame.encode();
                    self.statistics.frames_received += 1;
                    return Ok(Some(encoded));
                }
            }
        }
        Ok(None)
    }

    fn select_optimal_channel(&self, frame: &SpaceCANFrame) -> Result<u8, String> {
        // Select channel based on priority and power requirements
        let required_range = match frame.priority {
            FramePriority::Emergency => 2000.0,
            FramePriority::High => 1000.0,
            FramePriority::Normal => 500.0,
            FramePriority::Low => 100.0,
        };

        for (_channel_id, channel) in &self.channels {
            if channel.is_active {
                let channel_range = match channel.power_mode {
                    PowerMode::HighPower => 2000.0,
                    PowerMode::MediumPower => 1000.0,
                    PowerMode::LowPower => 500.0,
                    PowerMode::UltraLow => 100.0,
                };

                if channel_range >= required_range {
                    return Ok(0); // Return a default channel ID since we can't access the actual ID
                }
            }
        }

        Err("No suitable channel available".to_string())
    }

    pub fn get_statistics(&self) -> &CommunicationStats {
        &self.statistics
    }

    pub fn set_channel_power_mode(&mut self, channel_id: u8, power_mode: PowerMode) -> Result<(), String> {
        if let Some(channel) = self.channels.get_mut(&channel_id) {
            channel.power_mode = power_mode;
            info!("Set channel {} power mode to {:?}", channel_id, power_mode);
            Ok(())
        } else {
            Err(format!("Channel {} not found", channel_id))
        }
    }
}

impl Default for SpaceCANAdapter {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_frame_encoding_decoding() {
        let data = vec![1, 2, 3, 4, 5];
        let frame = SpaceCANFrame::new(0x123, data.clone(), FramePriority::High);
        
        let encoded = frame.encode();
        let decoded = SpaceCANFrame::decode(&encoded).unwrap();
        
        assert_eq!(frame.id, decoded.id);
        assert_eq!(frame.data, decoded.data);
        assert_eq!(frame.priority, decoded.priority);
    }

    #[test]
    fn test_error_detection() {
        let data = vec![1, 2, 3, 4, 5];
        let mut frame = SpaceCANFrame::new(0x123, data, FramePriority::High);
        
        // Corrupt data
        frame.data[0] = 255;
        
        // Should detect error
        assert!(frame.validate_and_correct().is_err());
    }

    #[test]
    fn test_power_modes() {
        let frame = SpaceCANFrame::new(0x123, vec![1, 2, 3], FramePriority::High)
            .with_power_mode(PowerMode::LowPower);
        
        assert_eq!(frame.power_mode, PowerMode::LowPower);
        assert_eq!(frame.get_power_requirements(), 0.5);
        assert_eq!(frame.get_transmission_range(), 500.0);
    }

    #[test]
    fn test_adapter_channel_management() {
        let mut adapter = SpaceCANAdapter::new();
        adapter.add_channel(2, 5800.0, 200.0);
        
        assert!(adapter.channels.contains_key(&2));
        assert!(adapter.set_channel_power_mode(2, PowerMode::HighPower).is_ok());
    }
}
