

use core::time::Duration;
use heapless::String;
use defmt::Format;
use crate::error::RustSatError;

#[derive(Debug, Clone)]
pub struct RustSatConfig {
    pub system: SystemConfig,
    pub network: NetworkConfig,
    pub security: SecurityConfig,
    pub telemetry: TelemetryConfig,
    pub simulation: SimulationConfig,
    pub logging: LoggingConfig,
}

#[derive(Debug, Clone)]
pub struct SystemConfig {
    pub satellite_id: u32,
    pub mission_name: String<32>,
    pub max_memory_mb: u64,
    pub max_cpu_percent: f64,
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
    pub battery_low_percent: f64,
    pub temperature_high_celsius: f64,
    pub temperature_low_celsius: f64,
    pub memory_usage_percent: f64,
    pub signal_strength_low: f64,
}

#[derive(Debug, Clone)]
pub struct SimulationConfig {
    pub time_acceleration: f64,
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
                max_memory_mb: 512,
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
            return Err(RustSatError::ConfigError);
        }

        if self.system.max_memory_mb < 64 {
            return Err(RustSatError::ConfigError);
        }

        if self.network.max_hops == 0 || self.network.retry_attempts == 0 {
            return Err(RustSatError::ConfigError);
        }

        let thresholds = &self.telemetry.alert_thresholds;
        if thresholds.battery_low_percent < 0.0 || thresholds.battery_low_percent > 100.0 {
            return Err(RustSatError::ConfigError);
        }

        if thresholds.temperature_high_celsius <= thresholds.temperature_low_celsius {
            return Err(RustSatError::ConfigError);
        }

        Ok(())
    }

    pub fn get_heartbeat_interval(&self) -> Duration {
        Duration::from_millis(self.system.heartbeat_interval_ms)
    }

    pub fn get_routing_update_interval(&self) -> Duration {
        Duration::from_millis(self.network.routing_update_interval_ms)
    }

    pub fn get_telemetry_interval(&self) -> Duration {
        Duration::from_millis(self.telemetry.collection_interval_ms)
    }

    pub fn is_battery_low(&self, level: f64) -> bool {
        level < self.telemetry.alert_thresholds.battery_low_percent / 100.0
    }

    pub fn is_temperature_critical(&self, temp_celsius: f64) -> bool {
        temp_celsius > self.telemetry.alert_thresholds.temperature_high_celsius
            || temp_celsius < self.telemetry.alert_thresholds.temperature_low_celsius
    }

    pub fn is_signal_weak(&self, strength: f64) -> bool {
        strength < self.telemetry.alert_thresholds.signal_strength_low
    }
}

pub struct ConfigBuilder {
    config: RustSatConfig,
}

impl ConfigBuilder {
    pub fn new() -> Self {
        Self {
            config: RustSatConfig::default(),
        }
    }

    pub fn satellite_id(mut self, id: u32) -> Self {
        self.config.system.satellite_id = id;
        self
    }

    pub fn encryption_enabled(mut self, enabled: bool) -> Self {
        self.config.security.encryption_enabled = enabled;
        self
    }

    pub fn build(self) -> Result<RustSatConfig, RustSatError> {
        self.config.validate()?;
        Ok(self.config)
    }
}

impl Default for ConfigBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config_is_valid() {
        let config = RustSatConfig::default();
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_config_builder() {
        let config = ConfigBuilder::new()
            .satellite_id(42)
            .encryption_enabled(false)
            .build()
            .unwrap(); // Allowed in tests, but we'll try to avoid it anyway.
        assert_eq!(config.system.satellite_id, 42);
        assert!(!config.security.encryption_enabled);
    }
}
