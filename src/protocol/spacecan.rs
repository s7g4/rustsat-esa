

use crate::error::RustSatError;
use defmt::{error, info, warn, Format};
use heapless::{FnvIndexMap, Vec as HeaplessVec};

/// Frame priority levels for CubeSat communications
#[derive(Debug, Clone, Copy, PartialEq, Eq, Format)]
pub enum FramePriority {
    Emergency = 0, // Life-critical systems
    High = 1,      // Mission-critical telemetry
    Normal = 2,    // Regular data transmission
    Low = 3,       // Housekeeping data
}

/// Power transmission modes for energy-efficient communication
#[derive(Debug, Clone, Copy, PartialEq, Eq, Format)]
pub enum PowerMode {
    HighPower,   // Maximum range, high energy consumption
    MediumPower, // Balanced range and energy
    LowPower,    // Energy-efficient, reduced range
    UltraLow,    // Emergency mode, minimal energy
}

/// Enhanced SpaceCAN frame with CubeSat-specific features
#[derive(Debug, Clone)]
pub struct SpaceCANFrame {
    pub id: u32,
    pub data: HeaplessVec<u8, 256>,
    pub dlc: u8,
    pub priority: FramePriority,
    pub power_mode: PowerMode,
    pub timestamp_ticks: u64,
    pub sequence_number: u16,
    pub checksum: u32,
    pub error_correction: HeaplessVec<u8, 64>,
}

impl SpaceCANFrame {
    pub fn new(
        id: u32,
        data: HeaplessVec<u8, 256>,
        priority: FramePriority,
        ticks: u64,
        seq: u16,
    ) -> Self {
        let dlc = data.len() as u8;
        let checksum = Self::calculate_checksum(&data);
        let error_correction = Self::generate_error_correction(&data);

        Self {
            id,
            data,
            dlc,
            priority,
            power_mode: PowerMode::MediumPower,
            timestamp_ticks: ticks,
            sequence_number: seq,
            checksum,
            error_correction,
        }
    }

    pub fn with_power_mode(mut self, power_mode: PowerMode) -> Self {
        self.power_mode = power_mode;
        self
    }

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

    fn generate_error_correction(data: &[u8]) -> HeaplessVec<u8, 64> {
        let mut ecc = HeaplessVec::new();
        for chunk in data.chunks(4) {
            let sum: u8 = chunk.iter().fold(0, |acc, &x| acc.wrapping_add(x));
            let _ = ecc.push(sum); // ignore push failure on full capacity
        }
        ecc
    }

    pub fn encode(&self) -> HeaplessVec<u8, 512> {
        let mut encoded = HeaplessVec::new();

        let _ = encoded.extend_from_slice(&self.id.to_be_bytes());
        let _ = encoded.push(self.dlc);
        let _ = encoded.push(self.priority as u8);
        let _ = encoded.push(self.power_mode as u8);

        let _ = encoded.extend_from_slice(&self.sequence_number.to_be_bytes());
        let _ = encoded.extend_from_slice(&self.timestamp_ticks.to_be_bytes());
        let _ = encoded.extend_from_slice(&self.data);
        let _ = encoded.extend_from_slice(&self.checksum.to_be_bytes());

        let _ = encoded.push(self.error_correction.len() as u8);
        let _ = encoded.extend_from_slice(&self.error_correction);

        encoded
    }

    pub fn decode(bytes: &[u8]) -> Result<Self, RustSatError> {
        if bytes.len() < 21 {
            return Err(RustSatError::SpaceCanError);
        }

        let mut offset = 0;
        let id = u32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]);
        offset += 4;

        let dlc = bytes[offset];
        offset += 1;

        let priority = match bytes[offset] {
            0 => FramePriority::Emergency,
            1 => FramePriority::High,
            2 => FramePriority::Normal,
            3 => FramePriority::Low,
            _ => return Err(RustSatError::SpaceCanError),
        };
        offset += 1;

        let power_mode = match bytes[offset] {
            0 => PowerMode::HighPower,
            1 => PowerMode::MediumPower,
            2 => PowerMode::LowPower,
            3 => PowerMode::UltraLow,
            _ => return Err(RustSatError::SpaceCanError),
        };
        offset += 1;

        let sequence_number = u16::from_be_bytes([bytes[offset], bytes[offset + 1]]);
        offset += 2;

        let timestamp_ticks = u64::from_be_bytes([
            bytes[offset],
            bytes[offset + 1],
            bytes[offset + 2],
            bytes[offset + 3],
            bytes[offset + 4],
            bytes[offset + 5],
            bytes[offset + 6],
            bytes[offset + 7],
        ]);
        offset += 8;

        if offset + dlc as usize + 5 > bytes.len() {
            return Err(RustSatError::SpaceCanError);
        }

        let mut data = HeaplessVec::new();
        let _ = data.extend_from_slice(&bytes[offset..offset + dlc as usize]);
        offset += dlc as usize;

        let checksum = u32::from_be_bytes([
            bytes[offset],
            bytes[offset + 1],
            bytes[offset + 2],
            bytes[offset + 3],
        ]);
        offset += 4;

        let calculated_checksum = Self::calculate_checksum(&data);
        if checksum != calculated_checksum {
            return Err(RustSatError::SpaceCanError);
        }

        let ecc_len = bytes[offset] as usize;
        offset += 1;

        if offset + ecc_len > bytes.len() {
            return Err(RustSatError::SpaceCanError);
        }

        let mut error_correction = HeaplessVec::new();
        let _ = error_correction.extend_from_slice(&bytes[offset..offset + ecc_len]);

        Ok(Self {
            id,
            data,
            dlc,
            priority,
            power_mode,
            timestamp_ticks,
            sequence_number,
            checksum,
            error_correction,
        })
    }
}

pub struct SpaceCANAdapter {
    channels: FnvIndexMap<u8, SpaceCANChannel, 4>,
    frame_buffer: HeaplessVec<SpaceCANFrame, 16>,
}

#[derive(Debug, Clone, Format)]
pub struct SpaceCANChannel {
    pub channel_id: u8,
    pub frequency: f64,
    pub bandwidth: f64,
    pub is_active: bool,
    pub power_mode: PowerMode,
}

impl SpaceCANAdapter {
    pub fn new() -> Self {
        let mut adapter = Self {
            channels: FnvIndexMap::new(),
            frame_buffer: HeaplessVec::new(),
        };
        adapter.add_channel(0, 437.5, 25.0);
        adapter.add_channel(1, 2400.0, 100.0);
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
        let _ = self.channels.insert(channel_id, channel);
        info!("Added communication channel {}", channel_id);
    }

    pub fn transmit(&mut self, frame: &SpaceCANFrame) -> Result<(), RustSatError> {
        let channel_id = self.select_optimal_channel(frame)?;
        let encoded = frame.encode();
        info!(
            "Transmitted frame seq={} on ch={}",
            frame.sequence_number, channel_id
        );
        Ok(())
    }

    pub fn receive(&mut self) -> Result<Option<HeaplessVec<u8, 512>>, RustSatError> {
        if !self.frame_buffer.is_empty() {
            let frame = self.frame_buffer.pop().unwrap();
            let encoded = frame.encode();
            return Ok(Some(encoded));
        }
        Ok(None)
    }

    fn select_optimal_channel(&self, _frame: &SpaceCANFrame) -> Result<u8, RustSatError> {
        for (id, channel) in self.channels.iter() {
            if channel.is_active {
                return Ok(*id);
            }
        }
        Err(RustSatError::SpaceCanError)
    }
}

impl Default for SpaceCANAdapter {
    fn default() -> Self {
        Self::new()
    }
}
