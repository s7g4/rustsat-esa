# ⚡ RustSat-ESA Quick Start Guide

Get up and running with RustSat-ESA in under 5 minutes!

## 🚀 Installation (2 minutes)

### Prerequisites
- **Rust 1.70+** - [Install here](https://rustup.rs/) if you don't have it

### Quick Install
```bash
# 1. Clone the repository
git clone https://github.com/s7g4/rustsat-esa.git
cd rustsat-esa

# 2. Build the project
cargo build --release

# 3. Run the demo
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

## 🎯 Quick Commands (1 minute)

### Essential Commands
```bash
# Basic demo
cargo run --example demo

# Full system demo
cargo run --example comprehensive_demo

# Performance benchmarks
cargo run --release --example protocol_benchmarks

# CLI tool
cargo run --bin simple-cli demo

# Run tests
cargo test
```

### Web Dashboard
```bash
# Open the dashboard
open dashboard.html
# or
firefox dashboard.html
```

## 🔧 Quick Configuration (1 minute)

### Create Basic Config
```bash
# Generate sample config
cargo run --bin simple-cli config --create mission.json
```

### Basic Configuration File (`mission.json`)
```json
{
  "mission": {
    "name": "My Mission",
    "satellite_count": 3,
    "duration_hours": 24
  },
  "network": {
    "mesh_enabled": true,
    "encryption_enabled": true
  },
  "simulation": {
    "time_acceleration": 60.0
  }
}
```

## 📚 Quick API Usage (1 minute)

### SpaceCAN Protocol
```rust
use rustsat_esa::protocol::spacecan::{SpaceCANFrame, FramePriority};

// Create and encode a frame
let frame = SpaceCANFrame::new(0x123, vec![1,2,3,4], FramePriority::High);
let encoded = frame.encode();

// Decode received frame
let decoded = SpaceCANFrame::decode(&encoded)?;
```

### Security Module
```rust
use rustsat_esa::security::CryptoModule;

// Encrypt/decrypt data
let crypto = CryptoModule::new();
let encrypted = crypto.encrypt(b"secret data")?;
let decrypted = crypto.decrypt(&encrypted)?;
```

### Mesh Network
```rust
use rustsat_esa::protocol::network::MeshNetwork;

// Initialize network
let network = MeshNetwork::new();
// Route messages between satellites
// (Full API available in documentation)
```

## 🧪 Quick Testing

### Run All Tests
```bash
cargo test
```

### Test Specific Component
```bash
cargo test protocol::spacecan
```

### Performance Test
```bash
cargo run --release --example protocol_benchmarks
```

## 🔍 Quick Troubleshooting

### Build Issues
```bash
# Update Rust and dependencies
rustup update
cargo update
cargo clean
cargo build
```

### Performance Issues
```bash
# Use release mode for better performance
cargo run --release --example demo
```

### Debug Information
```bash
# Enable debug logging
export RUST_LOG=debug
cargo run --example demo
```

## 📖 Next Steps

### Learn More
- Read the full [README.md](README.md) for detailed information
- Check [USAGE.md](USAGE.md) for comprehensive usage guide
- Explore [FEATURES.md](FEATURES.md) for complete feature list

### Try Advanced Features
```bash
# Full system simulation
cargo run --example comprehensive_demo

# CLI with different options
cargo run --bin simple-cli test spacecan
cargo run --bin simple-cli config --validate mission.json

# Performance analysis
cargo run --release --example protocol_benchmarks
```

### Explore the Code
```bash
# Project structure
ls -la src/

# Key modules
src/protocol/spacecan.rs    # SpaceCAN protocol
src/protocol/network.rs     # Mesh networking
src/cubesat/mod.rs         # CubeSat operations
src/security/mod.rs        # Security & encryption
src/simulation/mod.rs      # Space simulation
```

## 🎯 Common Use Cases

### 1. Protocol Testing
```bash
# Test SpaceCAN frames
cargo run --bin simple-cli test spacecan

# Test network routing
cargo run --bin simple-cli test network

# Test encryption
cargo run --bin simple-cli test security
```

### 2. Mission Simulation
```bash
# Run 24-hour mission simulation
cargo run --example comprehensive_demo

# Custom configuration
cargo run --bin simple-cli config --create my-mission.json
# Edit my-mission.json as needed
cargo run --example comprehensive_demo
```

### 3. Performance Analysis
```bash
# Benchmark all components
cargo run --release --example protocol_benchmarks

# Profile specific operations
cargo run --release --example demo
```

### 4. Development
```bash
# Watch for changes during development
cargo install cargo-watch
cargo watch -x "run --example demo"

# Format code
cargo fmt

# Check for issues
cargo clippy
```

## 🆘 Need Help?

### Quick Fixes
- **Build errors**: Run `cargo clean && cargo build`
- **Slow performance**: Use `cargo run --release`
- **Permission errors**: Examples run in simulation mode (no special permissions needed)

### Get Support
- Check [USAGE.md](USAGE.md) for detailed usage instructions
- Look at [TROUBLESHOOTING.md](TROUBLESHOOTING.md) for common issues
- Create an issue on GitHub for bugs or questions

## ✅ Success Checklist

After following this guide, you should be able to:
- [ ] Build the project successfully
- [ ] Run the basic demo
- [ ] See the web dashboard
- [ ] Run tests
- [ ] Use the CLI tool
- [ ] Create basic configurations

**Congratulations! You're now ready to explore RustSat-ESA! 🛰️**

---

**Total time**: ~5 minutes  
**Next**: Read [USAGE.md](USAGE.md) for comprehensive usage guide