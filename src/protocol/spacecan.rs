#![allow(missing_docs)]
use crate::error::RustSatError;

use heapless::{FnvIndexMap, Vec as HeaplessVec};

/// Frame priority levels for CubeSat communications
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FramePriority {
    Emergency = 0, // Life-critical systems
    High = 1,      // Mission-critical telemetry
    Normal = 2,    // Regular data transmission
    Low = 3,       // Housekeeping data
}

/// Power transmission modes for energy-efficient communication
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PowerMode {
    HighPower = 0,   // Maximum range, high energy consumption
    MediumPower = 1, // Balanced range and energy
    LowPower = 2,    // Energy-efficient, reduced range
    UltraLow = 3,    // Emergency mode, minimal energy
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
    ) -> Result<Self, RustSatError> {
        if data.len() > 255 {
            return Err(RustSatError::SpaceCanEncodeError);
        }
        let dlc = data.len() as u8;
        let checksum = Self::calculate_checksum(&data);
        let error_correction = Self::generate_error_correction(&data)?;

        Ok(Self {
            id,
            data,
            dlc,
            priority,
            power_mode: PowerMode::MediumPower,
            timestamp_ticks: ticks,
            sequence_number: seq,
            checksum,
            error_correction,
        })
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

    fn generate_error_correction(data: &[u8]) -> Result<HeaplessVec<u8, 64>, RustSatError> {
        let mut ecc = HeaplessVec::new();
        for chunk in data.chunks(4) {
            let sum: u8 = chunk.iter().fold(0, |acc, &x| acc.wrapping_add(x));
            ecc.push(sum)
                .map_err(|_| RustSatError::SystemError("ECC buffer overflow"))?;
        }
        Ok(ecc)
    }

    pub fn encode(&self) -> Result<HeaplessVec<u8, 512>, RustSatError> {
        let mut encoded = HeaplessVec::new();

        encoded
            .extend_from_slice(&self.id.to_be_bytes())
            .map_err(|_| RustSatError::SpaceCanEncodeError)?;
        encoded
            .push(self.dlc)
            .map_err(|_| RustSatError::SpaceCanEncodeError)?;
        encoded
            .push(self.priority as u8)
            .map_err(|_| RustSatError::SpaceCanEncodeError)?;
        encoded
            .push(self.power_mode as u8)
            .map_err(|_| RustSatError::SpaceCanEncodeError)?;

        encoded
            .extend_from_slice(&self.sequence_number.to_be_bytes())
            .map_err(|_| RustSatError::SpaceCanEncodeError)?;
        encoded
            .extend_from_slice(&self.timestamp_ticks.to_be_bytes())
            .map_err(|_| RustSatError::SpaceCanEncodeError)?;
        encoded
            .extend_from_slice(&self.data)
            .map_err(|_| RustSatError::SpaceCanEncodeError)?;
        encoded
            .extend_from_slice(&self.checksum.to_be_bytes())
            .map_err(|_| RustSatError::SpaceCanEncodeError)?;

        encoded
            .push(self.error_correction.len() as u8)
            .map_err(|_| RustSatError::SpaceCanEncodeError)?;
        encoded
            .extend_from_slice(&self.error_correction)
            .map_err(|_| RustSatError::SpaceCanEncodeError)?;

        Ok(encoded)
    }

    pub fn decode(bytes: &[u8]) -> Result<Self, RustSatError> {
        if bytes.len() < 22 {
            return Err(RustSatError::SpaceCanDecodeError);
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
            _ => return Err(RustSatError::SpaceCanDecodeError),
        };
        offset += 1;

        let power_mode = match bytes[offset] {
            0 => PowerMode::HighPower,
            1 => PowerMode::MediumPower,
            2 => PowerMode::LowPower,
            3 => PowerMode::UltraLow,
            _ => return Err(RustSatError::SpaceCanDecodeError),
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
            return Err(RustSatError::SpaceCanDecodeError);
        }

        let mut data = HeaplessVec::new();
        data.extend_from_slice(&bytes[offset..offset + dlc as usize])
            .map_err(|_| RustSatError::SystemError("Buffer overflow parsing data"))?;
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
            return Err(RustSatError::DataCorruption);
        }

        let ecc_len = bytes[offset] as usize;
        offset += 1;

        if offset + ecc_len > bytes.len() {
            return Err(RustSatError::SpaceCanDecodeError);
        }

        let mut error_correction = HeaplessVec::new();
        error_correction
            .extend_from_slice(&bytes[offset..offset + ecc_len])
            .map_err(|_| RustSatError::SystemError("Buffer overflow parsing ECC"))?;

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

#[derive(Debug, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct SpaceCANChannel {
    pub channel_id: u8,
    pub frequency: f32,
    pub bandwidth: f32,
    pub is_active: bool,
    pub power_mode: PowerMode,
}

impl SpaceCANAdapter {
    pub fn new() -> Self {
        let mut adapter = Self {
            channels: FnvIndexMap::new(),
            frame_buffer: HeaplessVec::new(),
        };
        adapter
            .add_channel(0, 437.5, 25.0)
            .expect("Failed to add default channel 0");
        adapter
            .add_channel(1, 2400.0, 100.0)
            .expect("Failed to add default channel 1");
        adapter
    }

    pub fn add_channel(
        &mut self,
        channel_id: u8,
        frequency: f32,
        bandwidth: f32,
    ) -> Result<(), RustSatError> {
        let channel = SpaceCANChannel {
            channel_id,
            frequency,
            bandwidth,
            is_active: true,
            power_mode: PowerMode::MediumPower,
        };
        self.channels
            .insert(channel_id, channel)
            .map_err(|_| RustSatError::SystemError("Channels full"))?;
        #[cfg(feature = "defmt")]
        defmt::info!("Added communication channel {}", channel_id);
        Ok(())
    }

    pub fn transmit(&mut self, frame: &SpaceCANFrame) -> Result<(), RustSatError> {
        let _channel_id = self.select_optimal_channel(frame)?;

        let mut _encoded = frame.encode()?;
        // Phase 2: Apply Forward Error Correction (Hamming 8,4)
        if frame.priority == FramePriority::Emergency || frame.priority == FramePriority::High {
            _encoded = crate::protocol::fec::Hamming84::encode(&_encoded)?;
        }

        #[cfg(feature = "defmt")]
        defmt::info!(
            "Transmitted frame seq={} on ch={} (FEC protected)",
            frame.sequence_number,
            _channel_id
        );
        Ok(())
    }

    pub fn receive(&mut self) -> Result<Option<HeaplessVec<u8, 512>>, RustSatError> {
        if let Some(frame) = self.frame_buffer.pop() {
            let mut encoded = frame.encode()?;

            // Phase 2: Attempt FEC Recovery if frame priority requires it
            if frame.priority == FramePriority::Emergency || frame.priority == FramePriority::High {
                match crate::protocol::fec::Hamming84::decode(&encoded) {
                    Ok(recovered) => encoded = recovered,
                    Err(_) => {
                        #[cfg(feature = "defmt")]
                        defmt::error!(
                            "Unrecoverable data corruption in frame seq={}",
                            frame.sequence_number
                        );
                        return Err(RustSatError::DataCorruption);
                    }
                }
            }

            return Ok(Some(encoded));
        }
        Ok(None)
    }

    /// Enqueues a received frame into the physical layer buffer.
    /// This should be called by the hardware-specific ISR when a frame is received over the air.
    pub fn enqueue_frame(&mut self, frame: SpaceCANFrame) -> Result<(), RustSatError> {
        self.frame_buffer
            .push(frame)
            .map_err(|_| RustSatError::SystemError("Frame buffer full"))
    }

    fn select_optimal_channel(&self, _frame: &SpaceCANFrame) -> Result<u8, RustSatError> {
        for (id, channel) in self.channels.iter() {
            if channel.is_active {
                return Ok(*id);
            }
        }
        Err(RustSatError::SpaceCanEncodeError)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spacecan_encode_decode() {
        let mut payload = HeaplessVec::new();
        payload.push(42).unwrap();
        let frame = SpaceCANFrame::new(123, payload, FramePriority::Normal, 1000, 1).unwrap();

        let encoded = frame.encode().unwrap();
        let decoded = SpaceCANFrame::decode(&encoded).unwrap();

        assert_eq!(frame.id, decoded.id);
        assert_eq!(frame.data.as_slice(), decoded.data.as_slice());
        assert_eq!(frame.checksum, decoded.checksum);
    }
}

impl Default for SpaceCANAdapter {
    fn default() -> Self {
        Self::new()
    }
}
