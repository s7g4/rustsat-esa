# 📖 RustSat-ESA Usage Guide

This guide provides detailed instructions for using all features of the RustSat-ESA communication protocol stack.

## Table of Contents

1. [Getting Started](#getting-started)
2. [Examples](#examples)
3. [Command Line Interface](#command-line-interface)
4. [Web Dashboard](#web-dashboard)
5. [Configuration](#configuration)
6. [Testing](#testing)
7. [API Usage](#api-usage)
8. [Troubleshooting](#troubleshooting)

## Getting Started

### System Requirements

- **Operating System**: Linux, macOS, or Windows
- **Rust**: Version 1.70 or later
- **Memory**: Minimum 4GB RAM recommended
- **Storage**: 500MB free space for build artifacts

### Installation Steps

1. **Install Rust** (if not already installed):
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   source ~/.cargo/env
   ```

2. **Clone the repository**:
   ```bash
   git clone https://github.com/s7g4/rustsat-esa.git
   cd rustsat-esa
   ```

3. **Build the project**:
   ```bash
   # Debug build (faster compilation)
   cargo build
   
   # Release build (optimized performance)
   cargo build --release
   ```

4. **Verify installation**:
   ```bash
   cargo run --example demo
   ```

## Examples

### 1. Basic Demo (`demo.rs`)

**Purpose**: Demonstrates core functionality of all major components.

**Usage**:
```bash
cargo run --example demo
```

**What it does**:
- Tests SpaceCAN frame encoding/decoding
- Initializes mesh network
- Creates CubeSat protocol instance
- Tests telemetry processing
- Demonstrates encryption/decryption

**Expected runtime**: ~2-3 seconds

### 2. Comprehensive Demo (`comprehensive_demo.rs`)

**Purpose**: Full system demonstration with orbital simulation.

**Usage**:
```bash
cargo run --example comprehensive_demo
```

**What it does**:
- Sets up complete satellite constellation
- Runs orbital mechanics simulation
- Simulates inter-satellite communication
- Processes telemetry data
- Demonstrates ground station contacts

**Expected runtime**: ~30-60 seconds

### 3. Protocol Benchmarks (`protocol_benchmarks.rs`)

**Purpose**: Performance testing and benchmarking.

**Usage**:
```bash
# Debug mode
cargo run --example protocol_benchmarks

# Release mode (recommended for accurate benchmarks)
cargo run --release --example protocol_benchmarks
```

**What it measures**:
- SpaceCAN encoding/decoding performance
- Network routing speed
- Encryption/decryption throughput
- Memory usage patterns

**Expected runtime**: ~10-15 seconds

## Command Line Interface

The CLI tool (`simple-cli`) provides interactive access to system features.

### Available Commands

#### Demo Command
```bash
cargo run --bin simple-cli demo [--quiet]
```
- Runs basic functionality demonstration
- `--quiet`: Reduces output verbosity

#### Test Command
```bash
cargo run --bin simple-cli test [COMPONENT]
```
- Tests individual components
- Available components: `spacecan`, `network`, `security`, `telemetry`

#### Config Command
```bash
cargo run --bin simple-cli config [--create|--validate] [FILE]
```
- `--create`: Creates a sample configuration file
- `--validate`: Validates existing configuration
- `FILE`: Path to configuration file (default: `config.json`)

#### Help Command
```bash
cargo run --bin simple-cli --help
```
- Shows all available commands and options

### CLI Examples

```bash
# Run basic demo quietly
cargo run --bin simple-cli demo --quiet

# Test only the SpaceCAN protocol
cargo run --bin simple-cli test spacecan

# Create a sample configuration
cargo run --bin simple-cli config --create mission.json

# Validate configuration
cargo run --bin simple-cli config --validate mission.json
```

## Web Dashboard

The web dashboard provides a real-time interface for mission monitoring.

### Opening the Dashboard

```bash
# Method 1: Direct file opening
open dashboard.html

# Method 2: Using a web browser
firefox dashboard.html
chrome dashboard.html

# Method 3: Using Python's built-in server
python3 -m http.server 8000
# Then open http://localhost:8000/dashboard.html
```

### Dashboard Features

1. **Satellite Tracking**
   - Real-time position display
   - Orbital trajectory visualization
   - Ground track plotting

2. **Communication Status**
   - Link quality indicators
   - Message throughput graphs
   - Error rate monitoring

3. **Telemetry Display**
   - System health metrics
   - Power consumption graphs
   - Temperature monitoring

4. **Mission Control**
   - Command scheduling
   - Mission timeline
   - Alert notifications

### Dashboard Controls

- **Zoom**: Mouse wheel or +/- buttons
- **Pan**: Click and drag
- **Reset View**: Double-click
- **Toggle Layers**: Use checkboxes in control panel

## Configuration

### Configuration File Format

RustSat-ESA uses JSON configuration files. Here's a complete example:

```json
{
  "mission": {
    "name": "Demo Mission",
    "description": "Demonstration of RustSat-ESA capabilities",
    "duration_hours": 24,
    "start_time": "2024-01-01T00:00:00Z",
    "satellite_count": 3,
    "mission_type": "technology_demonstration"
  },
  "satellites": [
    {
      "id": 1,
      "name": "DemoSat-1",
      "initial_position": {
        "latitude": 0.0,
        "longitude": 0.0,
        "altitude": 400.0
      },
      "power_budget": 10.0,
      "communication_range": 2000.0
    }
  ],
  "network": {
    "mesh_enabled": true,
    "encryption_enabled": true,
    "max_hops": 5,
    "routing_algorithm": "dijkstra",
    "heartbeat_interval": 30
  },
  "ground_stations": [
    {
      "id": 100,
      "name": "ESA Kourou",
      "latitude": 5.236,
      "longitude": -52.768,
      "elevation": 16.0,
      "antenna_gain": 20.0,
      "frequency_band": "S-band"
    }
  ],
  "security": {
    "encryption_algorithm": "AES256",
    "key_rotation_interval": 3600,
    "authentication_required": true,
    "certificate_validation": true
  },
  "telemetry": {
    "sampling_rate_hz": 1.0,
    "compression_enabled": true,
    "priority_channels": ["power", "thermal", "attitude"],
    "downlink_schedule": "automatic"
  },
  "simulation": {
    "time_acceleration": 60.0,
    "orbital_period_minutes": 90,
    "space_weather_enabled": true,
    "atmospheric_drag_enabled": true,
    "solar_radiation_pressure": true
  }
}
```

### Configuration Validation

Validate your configuration file:

```bash
cargo run --bin simple-cli config --validate your-config.json
```

### Environment Variables

You can override configuration using environment variables:

```bash
# Set log level
export RUST_LOG=debug

# Override satellite count
export RUSTSAT_SATELLITE_COUNT=5

# Set configuration file path
export RUSTSAT_CONFIG_PATH=/path/to/config.json

# Run with environment overrides
cargo run --example comprehensive_demo
```

## Testing

### Unit Tests

```bash
# Run all unit tests
cargo test

# Run tests for specific module
cargo test protocol::spacecan

# Run tests with output
cargo test -- --nocapture

# Run tests in parallel
cargo test --jobs 4
```

### Integration Tests

```bash
# Run all integration tests
cargo test --test integration_tests

# Run specific integration test
cargo test test_end_to_end_communication_flow

# Run integration tests with release optimizations
cargo test --release --test integration_tests
```

### Test Coverage

Generate test coverage report:

```bash
# Install coverage tool
cargo install cargo-tarpaulin

# Generate coverage report
cargo tarpaulin --out Html

# Open coverage report
open tarpaulin-report.html
```

### Performance Testing

```bash
# Run benchmarks
cargo run --release --example protocol_benchmarks

# Profile memory usage
cargo run --example comprehensive_demo --features memory-profiling

# Profile CPU usage
perf record cargo run --release --example comprehensive_demo
perf report
```

## API Usage

### Basic API Examples

#### Creating and Using SpaceCAN Frames

```rust
use rustsat_esa::protocol::spacecan::{SpaceCANFrame, FramePriority};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a frame with telemetry data
    let telemetry_data = vec![0x01, 0x02, 0x03, 0x04];
    let frame = SpaceCANFrame::new(0x200, telemetry_data, FramePriority::High);
    
    // Encode for transmission
    let encoded = frame.encode();
    println!("Encoded frame: {} bytes", encoded.len());
    
    // Decode received frame
    let decoded = SpaceCANFrame::decode(&encoded)?;
    println!("Decoded frame ID: 0x{:X}", decoded.id);
    
    Ok(())
}
```

#### Setting Up Mesh Network

```rust
use rustsat_esa::protocol::network::MeshNetwork;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut network = MeshNetwork::new();
    
    // Add nodes to the network
    // (Node addition methods would be available in full API)
    
    // Route a message
    let message = b"Hello from satellite 1";
    network.route_message(1, 2, message)?;
    
    Ok(())
}
```

#### Using Security Features

```rust
use rustsat_esa::security::CryptoModule;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let crypto = CryptoModule::new();
    
    // Encrypt sensitive command
    let command = b"DEPLOY_SOLAR_PANELS";
    let encrypted = crypto.encrypt(command)?;
    
    // Decrypt received command
    let decrypted = crypto.decrypt(&encrypted)?;
    assert_eq!(decrypted, command);
    
    println!("Encryption/decryption successful!");
    Ok(())
}
```

### Advanced API Usage

#### Custom Telemetry Processing

```rust
use rustsat_esa::telemetry::TelemetryProcessor;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let processor = TelemetryProcessor::new();
    
    // Process custom telemetry data
    // (Full API methods would be available)
    
    Ok(())
}
```

#### Orbital Simulation

```rust
use rustsat_esa::simulation::SpaceSimulator;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut simulator = SpaceSimulator::new();
    
    // Get current satellite positions
    let positions = simulator.get_satellite_positions();
    
    for (id, pos) in positions {
        println!("Satellite {}: lat={:.2}, lon={:.2}, alt={:.2}", 
                 id, pos.latitude, pos.longitude, pos.altitude);
    }
    
    Ok(())
}
```

## Troubleshooting

### Common Build Issues

#### Issue: Missing Dependencies
```
error: failed to resolve dependencies
```

**Solution**:
```bash
# Update Rust toolchain
rustup update

# Clean and rebuild
cargo clean
cargo build
```

#### Issue: Compilation Errors
```
error: could not compile `rustsat-esa`
```

**Solution**:
```bash
# Check Rust version
rustc --version

# Update to latest stable
rustup update stable
rustup default stable

# Try building again
cargo build
```

### Runtime Issues

#### Issue: Permission Denied
```
Error: Permission denied (os error 13)
```

**Solution**:
- The examples run in simulation mode by default
- No special permissions required
- If you see this error, check file permissions

#### Issue: Slow Performance
```
Demo taking too long to complete
```

**Solution**:
```bash
# Use release mode for better performance
cargo run --release --example demo

# Check system resources
top
htop
```

### Debug Information

Enable detailed logging:

```bash
# Set environment variable
export RUST_LOG=rustsat_esa=debug

# Run with debug output
cargo run --example demo 2>&1 | tee debug.log
```

### Getting Help

1. **Check the logs**: Look for error messages in the output
2. **Verify configuration**: Use `cargo run --bin simple-cli config --validate`
3. **Test components individually**: Use `cargo run --bin simple-cli test [component]`
4. **Check system resources**: Ensure adequate memory and CPU
5. **Update dependencies**: Run `cargo update`

### Performance Optimization

For best performance:

```bash
# Use release mode
cargo build --release

# Set CPU-specific optimizations
export RUSTFLAGS="-C target-cpu=native"
cargo build --release

# Run with optimizations
cargo run --release --example comprehensive_demo
```

---

This usage guide covers all major aspects of using RustSat-ESA. For additional help, refer to the main README.md or check the inline documentation in the source code.