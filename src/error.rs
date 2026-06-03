use defmt::Format;

/// Core domain errors for the RustSat protocol stack.
/// Replaces dynamic String errors to ensure zero-allocation failure handling.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Format)]
pub enum RustSatError {
    /// Failure during frame encoding/decoding at the physical layer
    SpaceCanError,
    /// Routing failure or topology mismatch in the mesh network
    NetworkError,
    /// Encryption/Decryption failure or invalid authentication
    SecurityError,
    /// Telemetry packet parsing or validation failure
    TelemetryError,
    /// Data integrity or checksum failure
    DataCorruption,
    /// Invalid packet format or header structure
    InvalidFormat,
    /// Core initialization or state machine error
    SystemError,
    /// Configuration validation error
    ConfigError,
}
