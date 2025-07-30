// Configuration management for the RustSat-ESA system
// This demonstrates understanding of production configuration patterns

use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use std::time::Duration;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RustSatConfig {
    pub system: SystemConfig,
    pub network: NetworkConfig,
    pub security: SecurityConfig,
    pub telemetry: TelemetryConfig,
    pub simulation: SimulationConfig,
    pub logging: LoggingConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemConfig {
    pub satellite_id: u32,
    pub mission_name: String,
    pub max_memory_mb: u64,
    pub max_cpu_percent: f64,
    pub heartbeat_interval_ms: u64,
    pub watchdog_timeout_ms: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkConfig {
    pub max_hops: u8,
    pub routing_update_interval_ms: u64,
    pub connection_timeout_ms: u64,
    pub retry_attempts: u32,
    pub retry_backoff_ms: u64,
    pub mesh_discovery_interval_ms: u64,
    pub ground_station_priority: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityConfig {
    pub encryption_enabled: bool,
    pub key_rotation_interval_hours: u64,
    pub max_auth_failures: u32,
    pub auth_timeout_ms: u64,
    pub emergency_bypass_enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TelemetryConfig {
    pub collection_interval_ms: u64,
    pub compression_enabled: bool,
    pub max_buffer_size: usize,
    pub alert_thresholds: AlertThresholds,
    pub data_retention_hours: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertThresholds {
    pub battery_low_percent: f64,
    pub temperature_high_celsius: f64,
    pub temperature_low_celsius: f64,
    pub memory_usage_percent: f64,
    pub signal_strength_low: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimulationConfig {
    pub time_acceleration: f64,
    pub orbital_perturbations: bool,
    pub space_weather_enabled: bool,
    pub ground_station_visibility: bool,
    pub realistic_delays: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoggingConfig {
    pub level: String,
    pub file_enabled: bool,
    pub console_enabled: bool,
    pub max_file_size_mb: u64,
    pub max_files: u32,
}

impl Default for RustSatConfig {
    fn default() -> Self {
        Self {
            system: SystemConfig {
                satellite_id: 1,
                mission_name: "RustSat-Demo".to_string(),
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
                level: "info".to_string(),
                file_enabled: true,
                console_enabled: true,
                max_file_size_mb: 10,
                max_files: 5,
            },
        }
    }
}

impl RustSatConfig {
    pub fn load_from_file<P: AsRef<Path>>(path: P) -> Result<Self, ConfigError> {
        let content = fs::read_to_string(path)
            .map_err(|e| ConfigError::FileRead(e.to_string()))?;
        
        let config: RustSatConfig = serde_json::from_str(&content)
            .map_err(|e| ConfigError::ParseError(e.to_string()))?;
        
        config.validate()?;
        Ok(config)
    }
    
    pub fn save_to_file<P: AsRef<Path>>(&self, path: P) -> Result<(), ConfigError> {
        let content = serde_json::to_string_pretty(self)
            .map_err(|e| ConfigError::SerializeError(e.to_string()))?;
        
        fs::write(path, content)
            .map_err(|e| ConfigError::FileWrite(e.to_string()))?;
        
        Ok(())
    }
    
    pub fn load_from_env() -> Self {
        let mut config = Self::default();
        
        // Override with environment variables if present
        if let Ok(sat_id) = std::env::var("RUSTSAT_SATELLITE_ID") {
            if let Ok(id) = sat_id.parse() {
                config.system.satellite_id = id;
            }
        }
        
        if let Ok(mission_name) = std::env::var("RUSTSAT_MISSION_NAME") {
            config.system.mission_name = mission_name;
        }
        
        if let Ok(log_level) = std::env::var("RUSTSAT_LOG_LEVEL") {
            config.logging.level = log_level;
        }
        
        if let Ok(encryption) = std::env::var("RUSTSAT_ENCRYPTION_ENABLED") {
            config.security.encryption_enabled = encryption.to_lowercase() == "true";
        }
        
        config
    }
    
    pub fn validate(&self) -> Result<(), ConfigError> {
        // Validate system config
        if self.system.satellite_id == 0 {
            return Err(ConfigError::ValidationError("Satellite ID cannot be zero".to_string()));
        }
        
        if self.system.mission_name.is_empty() {
            return Err(ConfigError::ValidationError("Mission name cannot be empty".to_string()));
        }
        
        if self.system.max_memory_mb < 64 {
            return Err(ConfigError::ValidationError("Minimum memory requirement is 64MB".to_string()));
        }
        
        // Validate network config
        if self.network.max_hops == 0 {
            return Err(ConfigError::ValidationError("Max hops must be at least 1".to_string()));
        }
        
        if self.network.retry_attempts == 0 {
            return Err(ConfigError::ValidationError("Retry attempts must be at least 1".to_string()));
        }
        
        // Validate telemetry thresholds
        let thresholds = &self.telemetry.alert_thresholds;
        if thresholds.battery_low_percent < 0.0 || thresholds.battery_low_percent > 100.0 {
            return Err(ConfigError::ValidationError("Battery threshold must be between 0-100%".to_string()));
        }
        
        if thresholds.temperature_high_celsius <= thresholds.temperature_low_celsius {
            return Err(ConfigError::ValidationError("High temperature threshold must be greater than low threshold".to_string()));
        }
        
        // Validate logging config
        let valid_levels = ["trace", "debug", "info", "warn", "error"];
        if !valid_levels.contains(&self.logging.level.as_str()) {
            return Err(ConfigError::ValidationError(format!("Invalid log level: {}", self.logging.level)));
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
        temp_celsius > self.telemetry.alert_thresholds.temperature_high_celsius ||
        temp_celsius < self.telemetry.alert_thresholds.temperature_low_celsius
    }
    
    pub fn is_signal_weak(&self, strength: f64) -> bool {
        strength < self.telemetry.alert_thresholds.signal_strength_low
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
    #[error("Failed to read config file: {0}")]
    FileRead(String),
    
    #[error("Failed to write config file: {0}")]
    FileWrite(String),
    
    #[error("Failed to parse config: {0}")]
    ParseError(String),
    
    #[error("Failed to serialize config: {0}")]
    SerializeError(String),
    
    #[error("Configuration validation error: {0}")]
    ValidationError(String),
}

// Configuration builder for programmatic config creation
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
    
    pub fn mission_name<S: Into<String>>(mut self, name: S) -> Self {
        self.config.system.mission_name = name.into();
        self
    }
    
    pub fn encryption_enabled(mut self, enabled: bool) -> Self {
        self.config.security.encryption_enabled = enabled;
        self
    }
    
    pub fn log_level<S: Into<String>>(mut self, level: S) -> Self {
        self.config.logging.level = level.into();
        self
    }
    
    pub fn telemetry_interval_ms(mut self, interval: u64) -> Self {
        self.config.telemetry.collection_interval_ms = interval;
        self
    }
    
    pub fn build(self) -> Result<RustSatConfig, ConfigError> {
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
    use tempfile::NamedTempFile;
    
    #[test]
    fn test_default_config_is_valid() {
        let config = RustSatConfig::default();
        assert!(config.validate().is_ok());
    }
    
    #[test]
    fn test_config_builder() {
        let config = ConfigBuilder::new()
            .satellite_id(42)
            .mission_name("Test Mission")
            .encryption_enabled(false)
            .log_level("debug")
            .build()
            .unwrap();
        
        assert_eq!(config.system.satellite_id, 42);
        assert_eq!(config.system.mission_name, "Test Mission");
        assert!(!config.security.encryption_enabled);
        assert_eq!(config.logging.level, "debug");
    }
    
    #[test]
    fn test_config_file_roundtrip() {
        let original_config = RustSatConfig::default();
        let temp_file = NamedTempFile::new().unwrap();
        
        // Save config
        original_config.save_to_file(temp_file.path()).unwrap();
        
        // Load config
        let loaded_config = RustSatConfig::load_from_file(temp_file.path()).unwrap();
        
        // Compare (using JSON serialization for easy comparison)
        let original_json = serde_json::to_string(&original_config).unwrap();
        let loaded_json = serde_json::to_string(&loaded_config).unwrap();
        assert_eq!(original_json, loaded_json);
    }
    
    #[test]
    fn test_validation_errors() {
        let mut config = RustSatConfig::default();
        
        // Test invalid satellite ID
        config.system.satellite_id = 0;
        assert!(config.validate().is_err());
        
        // Reset and test empty mission name
        config = RustSatConfig::default();
        config.system.mission_name = String::new();
        assert!(config.validate().is_err());
        
        // Reset and test invalid log level
        config = RustSatConfig::default();
        config.logging.level = "invalid".to_string();
        assert!(config.validate().is_err());
    }
    
    #[test]
    fn test_alert_thresholds() {
        let config = RustSatConfig::default();
        
        assert!(config.is_battery_low(0.15)); // 15% < 20% threshold
        assert!(!config.is_battery_low(0.25)); // 25% > 20% threshold
        
        assert!(config.is_temperature_critical(65.0)); // > 60°C
        assert!(config.is_temperature_critical(-25.0)); // < -20°C
        assert!(!config.is_temperature_critical(25.0)); // Normal range
        
        assert!(config.is_signal_weak(0.2)); // < 0.3 threshold
        assert!(!config.is_signal_weak(0.5)); // > 0.3 threshold
    }
}