use defmt::Format;
use heapless::Vec;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Format)]
pub enum FrameType {
    Telemetry = 0x01,
    Command = 0x02,
    Acknowledgment = 0x03,
    Emergency = 0x04,
    Beacon = 0x05,
}

#[derive(Debug, Clone)]
pub struct CubeSatFrame {
    pub frame_type: FrameType,
    pub payload: Vec<u8, 256>,
    pub timestamp_ticks: u64,
    pub source_id: u32,
    pub destination_id: u32,
    pub sequence_number: u16,
    pub acknowledgment_required: bool,
}

impl CubeSatFrame {
    pub fn new(
        frame_type: FrameType,
        payload: Vec<u8, 256>,
        source_id: u32,
        destination_id: u32,
        timestamp_ticks: u64,
    ) -> Self {
        Self {
            frame_type,
            payload,
            timestamp_ticks,
            source_id,
            destination_id,
            sequence_number: 0,
            acknowledgment_required: false,
        }
    }
}
