#![allow(missing_docs)]
/// Core domain errors for the RustSat protocol stack.
/// Replaces dynamic String errors to ensure zero-allocation failure handling.
#[derive(Debug, Clone, Copy, PartialEq, Eq, core::hash::Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[non_exhaustive]
pub enum RustSatError {
    /// Failure during frame encoding at the physical layer
    SpaceCanEncodeError,
    /// Failure during frame decoding at the physical layer
    SpaceCanDecodeError,
    /// Routing failure or topology mismatch in the mesh network
    NetworkError(&'static str),
    /// Encryption/Decryption failure or invalid authentication
    SecurityError,
    /// Telemetry packet parsing or validation failure
    TelemetryError,
    /// Data integrity or checksum failure
    DataCorruption,
    /// Invalid packet format or header structure
    InvalidFormat,
    /// Core initialization or state machine error
    SystemError(&'static str),
    /// Configuration validation error
    ConfigError(&'static str),
}

impl core::fmt::Display for RustSatError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::SpaceCanEncodeError => write!(f, "SpaceCAN encode error"),
            Self::SpaceCanDecodeError => write!(f, "SpaceCAN decode error"),
            Self::NetworkError(ctx) => write!(f, "Network error: {}", ctx),
            Self::SecurityError => write!(f, "Security error"),
            Self::TelemetryError => write!(f, "Telemetry error"),
            Self::DataCorruption => write!(f, "Data corruption"),
            Self::InvalidFormat => write!(f, "Invalid format"),
            Self::SystemError(ctx) => write!(f, "System error: {}", ctx),
            Self::ConfigError(ctx) => write!(f, "Config error: {}", ctx),
        }
    }
}
