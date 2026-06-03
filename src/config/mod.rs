#![allow(missing_docs)]
use crate::error::RustSatError;
use core::time::Duration;

use heapless::String;

#[derive(Debug, Clone)]
pub struct RustSatConfig {
    pub system: SystemConfig,
    pub network: NetworkConfig,
    pub security: SecurityConfig,
    pub telemetry: TelemetryConfig,
    #[cfg(feature = "simulation")]
    pub simulation: SimulationConfig,
    pub logging: LoggingConfig,
}

#[derive(Debug, Clone)]
pub struct SystemConfig {
    pub satellite_id: u32,
    pub mission_name: String<32>,
    pub max_memory_mb: u64,
    pub max_cpu_percent: f32,
    pub heartbeat_interval_ms: u64,
    pub watchdog_timeout_ms: u64,
}

#[derive(Debug, Clone)]
pub struct NetworkConfig {
    pub max_hops: u8,
    pub routing_update_interval_ms: u64,
    pub connection_timeout_ms: u64,
    pub retry_attempts: u32,
    pub retry_backoff_ms: u64,
    pub mesh_discovery_interval_ms: u64,
    pub ground_station_priority: u8,
}

#[derive(Debug, Clone)]
pub struct SecurityConfig {
    pub encryption_enabled: bool,
    pub key_rotation_interval_hours: u64,
    pub max_auth_failures: u32,
    pub auth_timeout_ms: u64,
    pub emergency_bypass_enabled: bool,
}

#[derive(Debug, Clone)]
pub struct TelemetryConfig {
    pub collection_interval_ms: u64,
    pub compression_enabled: bool,
    pub max_buffer_size: usize,
    pub alert_thresholds: AlertThresholds,
    pub data_retention_hours: u64,
}

#[derive(Debug, Clone)]
pub struct AlertThresholds {
    pub battery_low_percent: f32,
    pub temperature_high_celsius: f32,
    pub temperature_low_celsius: f32,
    pub memory_usage_percent: f32,
    pub signal_strength_low: f32,
}

#[cfg(feature = "simulation")]
#[derive(Debug, Clone)]
pub struct SimulationConfig {
    pub time_acceleration: f32,
    pub orbital_perturbations: bool,
    pub space_weather_enabled: bool,
    pub ground_station_visibility: bool,
    pub realistic_delays: bool,
}

#[derive(Debug, Clone)]
pub struct LoggingConfig {
    pub level: String<16>,
    pub console_enabled: bool,
}

impl Default for RustSatConfig {
    fn default() -> Self {
        Self {
            system: SystemConfig {
                satellite_id: 1,
                mission_name: "RustSat-Demo".try_into().unwrap_or_default(),
                max_memory_mb: 2, // 2MB is realistic for Cortex-M3
                max_cpu_percent: 80.0,
                heartbeat_interval_ms: 1000,
                watchdog_timeout_ms: 5000,
            },
            network: NetworkConfig {
                max_hops: 5,
                routing_update_interval_ms: 30000,
                connection_timeout_ms: 5000,
                retry_attempts: 3,
                retry_backoff_ms: 1000,
                mesh_discovery_interval_ms: 60000,
                ground_station_priority: 10,
            },
            security: SecurityConfig {
                encryption_enabled: true,
                key_rotation_interval_hours: 24,
                max_auth_failures: 3,
                auth_timeout_ms: 10000,
                emergency_bypass_enabled: false,
            },
            telemetry: TelemetryConfig {
                collection_interval_ms: 5000,
                compression_enabled: true,
                max_buffer_size: 10000,
                alert_thresholds: AlertThresholds {
                    battery_low_percent: 20.0,
                    temperature_high_celsius: 60.0,
                    temperature_low_celsius: -20.0,
                    memory_usage_percent: 90.0,
                    signal_strength_low: 0.3,
                },
                data_retention_hours: 72,
            },
            #[cfg(feature = "simulation")]
            simulation: SimulationConfig {
                time_acceleration: 1.0,
                orbital_perturbations: true,
                space_weather_enabled: true,
                ground_station_visibility: true,
                realistic_delays: true,
            },
            logging: LoggingConfig {
                level: "info".try_into().unwrap_or_default(),
                console_enabled: true,
            },
        }
    }
}

impl RustSatConfig {
    pub fn validate(&self) -> Result<(), RustSatError> {
        if self.system.satellite_id == 0 {
            return Err(RustSatError::ConfigError("satellite_id cannot be 0"));
        }

        if self.system.max_memory_mb < 1 {
            return Err(RustSatError::ConfigError(
                "max_memory_mb must be at least 1",
            ));
        }

        if self.network.max_hops == 0 || self.network.retry_attempts == 0 {
            return Err(RustSatError::ConfigError("network hops or retry 0"));
        }

        let thresholds = &self.telemetry.alert_thresholds;
        if thresholds.battery_low_percent < 0.0 || thresholds.battery_low_percent > 100.0 {
            return Err(RustSatError::ConfigError("battery percent out of bounds"));
        }

        if thresholds.temperature_high_celsius <= thresholds.temperature_low_celsius {
            return Err(RustSatError::ConfigError("temperature min >= max"));
        }

        Ok(())
    }

    /// Host/Simulation helper for retrieving the configured heartbeat interval.
    pub fn get_heartbeat_interval(&self) -> Duration {
        Duration::from_millis(self.system.heartbeat_interval_ms)
    }

    /// Host/Simulation helper for retrieving the configured routing update interval.
    pub fn get_routing_update_interval(&self) -> Duration {
        Duration::from_millis(self.network.routing_update_interval_ms)
    }

    /// Host/Simulation helper for retrieving the configured telemetry interval.
    pub fn get_telemetry_interval(&self) -> Duration {
        Duration::from_millis(self.telemetry.collection_interval_ms)
    }

    pub fn is_battery_low(&self, level_percent: f32) -> bool {
        level_percent < self.telemetry.alert_thresholds.battery_low_percent
    }

    pub fn is_temperature_critical(&self, temp_celsius: f32) -> bool {
        temp_celsius > self.telemetry.alert_thresholds.temperature_high_celsius
            || temp_celsius < self.telemetry.alert_thresholds.temperature_low_celsius
    }

    pub fn is_signal_weak(&self, strength: f32) -> bool {
        strength < self.telemetry.alert_thresholds.signal_strength_low
    }
}
