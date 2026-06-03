#![allow(missing_docs)]
use crate::error::RustSatError;

use heapless::{FnvIndexMap, Vec};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TelemetryType {
    SystemHealth,
    PowerStatus,
    OrbitPosition,
    Communication,
    Payload,
    Temperature,
    Attitude,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct TelemetryData {
    pub timestamp_ticks: u64,
    pub source_node: u32,
    pub data_type: TelemetryType,
    pub value: TelemetryValue,
    pub quality: f32, // 0.0 to 1.0
    pub sequence_number: u64,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TelemetryValue {
    Float(f32),
    Integer(i32),
    Boolean(bool),
    Vector3D(f32, f32, f32),
}

#[derive(Debug, Clone)]

pub struct TelemetryBuffer {
    pub packet_id: u32,
    pub source_node: u32,
    pub timestamp_ticks: u64,
    pub data_points: Vec<TelemetryData, 16>,
    pub priority: u8,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct AlertThreshold {
    pub min_value: Option<f32>,
    pub max_value: Option<f32>,
    pub rate_of_change_limit: Option<f32>,
}

pub struct TelemetryProcessor {
    telemetry_buffer: heapless::Deque<TelemetryData, 64>,
    alert_thresholds: FnvIndexMap<TelemetryType, AlertThreshold, 8>,
    pub alerts_generated: u32,
    pub packet_sequence: u32,
}

impl TelemetryProcessor {
    pub fn new() -> Self {
        Self {
            telemetry_buffer: heapless::Deque::new(),
            alert_thresholds: FnvIndexMap::new(),
            alerts_generated: 0,
            packet_sequence: 0,
        }
    }

    pub fn initialize(&mut self) -> Result<(), RustSatError> {
        #[cfg(feature = "defmt")]
        defmt::info!("Initializing telemetry processor");
        self.setup_default_thresholds();
        Ok(())
    }

    fn setup_default_thresholds(&mut self) {
        self.alert_thresholds
            .insert(
                TelemetryType::PowerStatus,
                AlertThreshold {
                    min_value: Some(20.0),
                    max_value: None,
                    rate_of_change_limit: Some(-5.0),
                },
            )
            .expect("infallible: static map size");

        self.alert_thresholds
            .insert(
                TelemetryType::Temperature,
                AlertThreshold {
                    min_value: Some(-40.0),
                    max_value: Some(85.0),
                    rate_of_change_limit: Some(10.0),
                },
            )
            .expect("infallible: static map size");
    }

    pub fn process_telemetry(&mut self, data: TelemetryData) -> Result<(), RustSatError> {
        #[cfg(feature = "defmt")]
        defmt::debug!("Processing telemetry data");

        if data.quality < 0.5 {
            #[cfg(feature = "defmt")]
            defmt::warn!("Low quality telemetry data received");
        }

        self.check_alerts(&data)?;

        if self.telemetry_buffer.is_full() {
            // O(1) ring buffer logic via Deque
            self.telemetry_buffer
                .pop_front()
                .expect("infallible: buffer is full");
        }
        self.telemetry_buffer
            .push_back(data)
            .expect("infallible: space was just made");

        Ok(())
    }

    fn check_alerts(&mut self, data: &TelemetryData) -> Result<(), RustSatError> {
        if let Some(threshold) = self.alert_thresholds.get(&data.data_type) {
            let value = match &data.value {
                TelemetryValue::Float(v) => *v,
                TelemetryValue::Integer(v) => *v as f32,
                _ => return Ok(()),
            };

            let mut alert_triggered = false;

            if let Some(min_val) = threshold.min_value {
                if value < min_val {
                    alert_triggered = true;
                }
            }

            if let Some(max_val) = threshold.max_value {
                if value > max_val {
                    alert_triggered = true;
                }
            }

            if alert_triggered {
                #[cfg(feature = "defmt")]
                defmt::error!("Telemetry alert triggered!");
                self.alerts_generated += 1;
            }
        }
        Ok(())
    }

    /// Creates a telemetry packet from the current buffer.
    /// Note: This takes `&mut self` because it increments the internal packet sequence counter,
    /// even though the buffer read operation itself is immutable.
    pub fn create_telemetry_packet(
        &mut self,
        node_id: u32,
    ) -> Result<TelemetryBuffer, RustSatError> {
        let mut data_points = Vec::new();

        for data in self.telemetry_buffer.iter() {
            if data.source_node == node_id && !data_points.is_full() {
                data_points
                    .push(data.clone())
                    .expect("infallible: guarded by !is_full");
            }
        }

        if data_points.is_empty() {
            return Err(RustSatError::TelemetryError);
        }

        self.packet_sequence = self.packet_sequence.wrapping_add(1);

        Ok(TelemetryBuffer {
            packet_id: self.packet_sequence,
            source_node: node_id,
            timestamp_ticks: 0,
            data_points,
            priority: 1,
        })
    }

    pub fn log_transmission(&mut self, _destination: u32, _bytes_sent: usize) {
        #[cfg(feature = "defmt")]
        defmt::debug!("Transmission to node {}", _destination);
    }

    pub fn log_reception(&mut self, _bytes_received: usize) {
        #[cfg(feature = "defmt")]
        defmt::debug!("Reception logged");
    }
}

impl Default for TelemetryProcessor {
    fn default() -> Self {
        Self::new()
    }
}
