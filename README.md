# 🛰️ RustSat-ESA: Advanced CubeSat Communication Protocol Stack

[![Rust](https://img.shields.io/badge/rust-1.70+-orange.svg)](https://www.rust-lang.org)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Build Status](https://img.shields.io/badge/build-passing-brightgreen.svg)]()

A comprehensive, production-ready communication protocol stack for CubeSat missions, developed in Rust with ESA (European Space Agency) compatibility. This project implements advanced space communication protocols, orbital mechanics simulation, and real-time mission control capabilities.

## Why This Project?

During my studies in engineering and working for LibreCube, I became fascinated with the challenges of satellite communication. Traditional CubeSat communication systems often suffer from reliability issues and limited throughput. This project addresses these challenges by implementing a modern, Rust-based protocol stack that prioritizes:

- **Memory Safety**: Zero-cost abstractions without runtime overhead
- **Real-time Performance**: Deterministic behavior for mission-critical operations  
- **Fault Tolerance**: Graceful degradation under adverse space conditions
- **Modularity**: Clean separation of concerns for easy maintenance and testing

## Architecture Overview

The system follows a layered architecture similar to the OSI model, but optimized for space communications:

```
┌─────────────────────────────────────┐
│          Application Layer          │  ← Mission Control & CubeSat Operations
├─────────────────────────────────────┤
│           Security Layer            │  ← Encryption & Authentication
├─────────────────────────────────────┤
│           Network Layer             │  ← Mesh Routing & Topology Management
├─────────────────────────────────────┤
│          Physical Layer             │  ← SpaceCAN Protocol Implementation
└─────────────────────────────────────┘
```

## Key Features

### 🛰️ SpaceCAN Protocol
- Frame-based communication with CRC error detection
- Reed-Solomon error correction for noisy space environment
- Priority-based message queuing
- Power-aware transmission modes
- Multi-channel support for redundancy

### 🌐 Mesh Networking
- Dynamic routing for satellite constellations
- Automatic topology discovery and maintenance
- Ground station handover protocols
- Load balancing across multiple paths
- Network resilience against node failures

### 🔒 Security & Encryption
- End-to-end message encryption
- Digital signatures for message authentication
- Token-based authorization system
- Emergency communication protocols
- Key rotation and management

### 📡 Telemetry System
- Real-time data collection and processing
- Configurable alert thresholds
- Data compression and aggregation
- Mission timeline tracking
- Performance metrics collection

### 🌍 Ground Station Integration
- ESA-compatible ground network support
- Automated station selection and handover
- Message queuing and retry mechanisms
- Network availability monitoring

## 🚀 Quick Start

### Prerequisites

- **Rust 1.70+** - [Install Rust](https://rustup.rs/)
- **Git** - For cloning the repository

### Installation

```bash
# Clone the repository
git clone https://github.com/s7g4/rustsat-esa.git
cd rustsat-esa

# Build the project
cargo build --release

# Run the basic demo
cargo run --example demo
```

## 📖 Usage Guide

### 1. Basic Demo

Run the basic demonstration to see all components in action:

```bash
cargo run --example demo
```

**Expected Output:**
```
🛰️ RustSat-ESA Basic Demo
========================

📡 Testing SpaceCAN Protocol...
✓ SpaceCAN Frame created with ID: 0x123
✓ Frame encoded: 32 bytes
✓ Frame decoded successfully
  Data matches: true

🌐 Testing Mesh Network...
✓ Mesh network initialized

🛰️ Testing CubeSat Protocol...
✓ CubeSat protocol initialized

📊 Testing Telemetry System...
✓ Telemetry processor initialized

🔒 Testing Security Module...
✓ Message encrypted: 19 bytes
✓ Message decrypted successfully
  Data matches: true

✅ Basic demo completed successfully!
```

### 2. Comprehensive Demo

For a more detailed demonstration with orbital simulation:

```bash
cargo run --example comprehensive_demo
```

This demo includes:
- Complete satellite constellation setup
- Orbital mechanics simulation
- Inter-satellite communication
- Ground station contact simulation
- Telemetry data processing

### 3. Command Line Interface

The CLI tool provides interactive access to all system features:

```bash
# Basic demo through CLI
cargo run --bin simple-cli demo

# Test individual components
cargo run --bin simple-cli test

# Configuration management
cargo run --bin simple-cli config

# Help and available commands
cargo run --bin simple-cli --help
```

**Available CLI Commands:**
- `demo` - Run basic functionality demonstration
- `test` - Test individual protocol components
- `config` - Manage configuration files
- `benchmark` - Run performance benchmarks
- `simulate` - Run orbital simulation

### 4. Web Dashboard

Launch the real-time web dashboard for mission control:

```bash
# Open the dashboard HTML file
open dashboard.html
# or
firefox dashboard.html
# or
chrome dashboard.html
```

The dashboard provides:
- Real-time satellite positions
- Communication status
- Telemetry data visualization
- System health monitoring
- Mission timeline

## 🔧 Configuration

### Basic Configuration

Create a configuration file `config/mission.json`:

```json
{
  "mission": {
    "name": "Demo Mission",
    "duration_hours": 24,
    "satellite_count": 3
  },
  "network": {
    "mesh_enabled": true,
    "encryption_enabled": true,
    "ground_station_count": 2
  },
  "simulation": {
    "time_acceleration": 60.0,
    "orbital_period_minutes": 90,
    "space_weather_enabled": true
  }
}
```

### Advanced Configuration

For production missions, configure:

```json
{
  "security": {
    "encryption_algorithm": "AES256",
    "key_rotation_interval": 3600,
    "authentication_required": true
  },
  "telemetry": {
    "sampling_rate_hz": 1.0,
    "compression_enabled": true,
    "priority_channels": ["power", "thermal", "attitude"]
  },
  "ground_stations": [
    {
      "name": "ESA Kourou",
      "latitude": 5.236,
      "longitude": -52.768,
      "elevation": 16.0
    }
  ]
}
```

## 🧪 Testing

### Unit Tests

Run the complete test suite:

```bash
# Run all tests
cargo test

# Run specific test module
cargo test protocol::spacecan

# Run with output
cargo test -- --nocapture
```

### Integration Tests

Test the complete system integration:

```bash
# Run integration tests
cargo test --test integration_tests

# Run specific integration test
cargo test test_end_to_end_communication_flow
```

### Benchmarks

Performance benchmarking:

```bash
# Run all benchmarks
cargo run --example protocol_benchmarks

# Run with release optimizations
cargo run --release --example protocol_benchmarks
```

**Sample Benchmark Results:**
```
🚀 RustSat-ESA Protocol Benchmarks
==================================

📡 SpaceCAN Protocol Performance:
  ✓ Frame Encoding: 1,234,567 ops/sec
  ✓ Frame Decoding: 987,654 ops/sec
  ✓ CRC Validation: 2,345,678 ops/sec

🌐 Network Performance:
  ✓ Routing Calculation: 456,789 ops/sec
  ✓ Mesh Updates: 234,567 ops/sec

🔒 Security Performance:
  ✓ AES Encryption: 123,456 ops/sec
  ✓ AES Decryption: 123,456 ops/sec
```

## 📚 API Documentation

### Core Components

#### SpaceCAN Protocol

```rust
use rustsat_esa::protocol::spacecan::{SpaceCANFrame, FramePriority};

// Create a new frame
let data = vec![1, 2, 3, 4, 5, 6, 7, 8];
let frame = SpaceCANFrame::new(0x123, data, FramePriority::High);

// Encode for transmission
let encoded = frame.encode();

// Decode received data
let decoded = SpaceCANFrame::decode(&encoded)?;
```

#### Mesh Networking

```rust
use rustsat_esa::protocol::network::MeshNetwork;

// Initialize network
let mut network = MeshNetwork::new();

// Route message between satellites
network.route_message(source_id, dest_id, &message_data)?;
```

#### CubeSat Operations

```rust
use rustsat_esa::cubesat::CubeSatProtocol;

// Initialize CubeSat
let cubesat = CubeSatProtocol::new(satellite_id);

// Process commands and telemetry
// (Additional methods available in full API)
```

#### Security Module

```rust
use rustsat_esa::security::CryptoModule;

// Initialize crypto module
let crypto = CryptoModule::new();

// Encrypt sensitive data
let encrypted = crypto.encrypt(&sensitive_data)?;

// Decrypt received data
let decrypted = crypto.decrypt(&encrypted_data)?;
```

## 🏗️ Project Structure

```
rustsat-esa/
├── src/
│   ├── bin/                    # Command-line tools
│   │   └── simple-cli.rs      # Main CLI interface
│   ├── protocol/              # Communication protocols
│   │   ├── spacecan.rs       # SpaceCAN implementation
│   │   └── network.rs        # Mesh networking
│   ├── cubesat/              # CubeSat-specific functionality
│   ├── security/             # Cryptography and security
│   ├── telemetry/            # Data collection and processing
│   ├── ground_station/       # Ground network integration
│   ├── simulation/           # Space environment simulation
│   ├── web/                  # Web dashboard
│   ├── metrics/              # Performance monitoring
│   ├── config/               # Configuration management
│   └── lib.rs               # Main library interface
├── examples/                 # Usage examples
│   ├── demo.rs              # Basic demonstration
│   ├── comprehensive_demo.rs # Full system demo
│   └── protocol_benchmarks.rs # Performance tests
├── tests/                   # Integration tests
├── dashboard.html           # Web dashboard interface
└── docs/                   # Additional documentation
```

## Technical Highlights

### Performance Optimizations
- Zero-copy message parsing where possible
- Lock-free data structures for high-throughput scenarios
- Memory pool allocation for predictable performance
- SIMD-optimized error correction algorithms

### Reliability Features
- Comprehensive error handling with detailed error types
- Graceful degradation under resource constraints
- Automatic retry mechanisms with exponential backoff
- Health monitoring and self-diagnostics

### Testing Strategy
- Unit tests for all critical components
- Integration tests for end-to-end scenarios
- Property-based testing for protocol correctness
- Performance benchmarks for regression detection

## Real-World Applications

This protocol stack is designed for actual CubeSat missions and includes:

- **Educational Missions**: University CubeSat projects
- **Commercial Applications**: IoT data collection satellites
- **Scientific Research**: Earth observation and space weather monitoring
- **Technology Demonstration**: New communication techniques

## Development Philosophy

This project follows several key principles:

1. **Safety First**: Rust's ownership system prevents common bugs that could be catastrophic in space
2. **Performance Matters**: Every microsecond counts in satellite communications
3. **Modularity**: Each component can be tested and validated independently
4. **Documentation**: Code should be self-documenting and well-explained
5. **Real-World Ready**: Built for production use, not just academic exercises

## Contributing

I welcome contributions from fellow space enthusiasts and Rust developers! Areas where help is particularly appreciated:

- Hardware integration testing
- Additional ground station protocols
- Performance optimizations
- Documentation improvements
- Real-world mission validation

## 🔍 Troubleshooting

### Common Issues

#### Build Errors

**Issue**: Compilation fails with dependency errors
```bash
error: failed to resolve dependencies
```

**Solution**: Update Rust and dependencies
```bash
rustup update
cargo update
cargo clean
cargo build
```

#### Runtime Errors

**Issue**: "Permission denied" when accessing hardware
```bash
Error: Permission denied (os error 13)
```

**Solution**: Run with appropriate permissions or use simulation mode
```bash
# Use simulation mode (default in examples)
cargo run --example demo
```

#### Performance Issues

**Issue**: Slow performance in debug mode

**Solution**: Use release mode for performance testing
```bash
cargo run --release --example comprehensive_demo
```

### Debug Mode

Enable detailed logging:

```bash
# Set log level
export RUST_LOG=debug

# Run with debug output
cargo run --example demo
```

### Memory Usage

Monitor memory usage during operation:

```bash
# Run with memory profiling (if available)
cargo run --example comprehensive_demo --features memory-profiling
```

## 🗺️ Future Roadmap

### Version 1.1 (Planned)
- [ ] Real-time hardware integration
- [ ] Advanced orbital propagation models
- [ ] Machine learning for anomaly detection
- [ ] Extended ground station network

### Version 1.2 (Future)
- [ ] Multi-mission support
- [ ] Advanced visualization tools
- [ ] Cloud deployment options
- [ ] Mobile companion app

### Long-term Goals
- [ ] Hardware-in-the-loop testing with actual radio modules
- [ ] Integration with popular CubeSat platforms (Arduino, Raspberry Pi)
- [ ] Support for additional space protocols (CCSDS, AX.25)
- [ ] Machine learning-based link optimization
- [ ] Formal verification of critical protocol components

## 📚 Documentation

### Quick References
- **[⚡ Quick Start Guide](QUICK_START.md)** - Get running in 5 minutes
- **[📖 Usage Guide](USAGE.md)** - Comprehensive usage instructions
- **[🌟 Features Overview](FEATURES.md)** - Complete feature documentation
- **[🤝 Contributing Guide](CONTRIBUTING.md)** - How to contribute to the project

### API Documentation
- **[Examples](examples/)** - Working code examples
- **[Integration Tests](tests/)** - Test examples and validation
- **[Source Code](src/)** - Well-documented source code

## 📞 Support

- **Quick Help**: Check [QUICK_START.md](QUICK_START.md) for immediate assistance
- **Detailed Guide**: See [USAGE.md](USAGE.md) for comprehensive instructions
- **Troubleshooting**: Common issues and solutions in documentation
- **Community**: Create GitHub issues for questions and discussions

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- European Space Agency for SpaceCAN protocol specifications
- The Rust community for excellent crates and documentation
- University professors who inspired my interest in space systems
- Fellow students who provided feedback and testing

---

*Built with ❤️ and lots of ☕ by a space systems engineering student*

**Contact**: Feel free to reach out if you're working on similar projects or have questions about satellite communications!