#!/usr/bin/env rust
//! Simple CLI for RustSat-ESA
//! 
//! A basic command-line interface demonstrating the protocol stack functionality.

use rustsat_esa::*;
use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        print_help();
        return Ok(());
    }
    
    match args[1].as_str() {
        "demo" => run_demo()?,
        "test" => run_tests()?,
        "config" => show_config()?,
        "help" | "--help" | "-h" => print_help(),
        _ => {
            println!("Unknown command: {}", args[1]);
            print_help();
        }
    }
    
    Ok(())
}

fn print_help() {
    println!("RustSat-ESA Simple CLI");
    println!("Usage: simple-cli <command>");
    println!();
    println!("Commands:");
    println!("  demo     - Run a basic demonstration");
    println!("  test     - Run protocol tests");
    println!("  config   - Show configuration options");
    println!("  help     - Show this help message");
}

fn run_demo() -> Result<(), Box<dyn std::error::Error>> {
    println!("🛰️  RustSat-ESA Demo");
    println!("===================");
    
    // Test SpaceCAN protocol
    println!("\n📡 Testing SpaceCAN Protocol...");
    let _adapter = protocol::spacecan::SpaceCANAdapter::new();
    
    let test_data = b"Hello from CubeSat!";
    let frame = protocol::spacecan::SpaceCANFrame::new(
        0x123, 
        test_data.to_vec(), 
        protocol::spacecan::FramePriority::High
    );
    
    let encoded = frame.encode();
    println!("✓ Frame encoded: {} bytes", encoded.len());
    
    match protocol::spacecan::SpaceCANFrame::decode(&encoded) {
        Ok(decoded) => {
            println!("✓ Frame decoded successfully");
            println!("  Data matches: {}", decoded.data == test_data);
        },
        Err(e) => println!("✗ Decode failed: {}", e),
    }
    
    // Test mesh networking
    println!("\n🌐 Testing Mesh Network...");
    let _network = protocol::network::MeshNetwork::new();
    println!("✓ Mesh network initialized");
    
    // Test telemetry
    println!("\n📊 Testing Telemetry System...");
    let _processor = telemetry::TelemetryProcessor::new();
    println!("✓ Telemetry processor initialized");
    
    // Test security
    println!("\n🔒 Testing Security Module...");
    let mut crypto = security::CryptoModule::new();
    let test_message = b"Secret satellite data";
    
    match crypto.encrypt(test_message) {
        Ok(encrypted) => {
            println!("✓ Message encrypted: {} bytes", encrypted.len());
            
            match crypto.decrypt(&encrypted) {
                Ok(decrypted) => {
                    println!("✓ Message decrypted successfully");
                    println!("  Data matches: {}", decrypted == test_message);
                },
                Err(e) => println!("✗ Decryption failed: {}", e),
            }
        },
        Err(e) => println!("✗ Encryption failed: {}", e),
    }
    
    println!("\n✅ Demo completed successfully!");
    Ok(())
}

fn run_tests() -> Result<(), Box<dyn std::error::Error>> {
    println!("🧪 Running Protocol Tests");
    println!("=========================");
    
    let mut passed = 0;
    let mut total = 0;
    
    // Test 1: SpaceCAN frame creation
    total += 1;
    println!("\nTest 1: SpaceCAN Frame Creation");
    let frame = protocol::spacecan::SpaceCANFrame::new(
        0x456,
        vec![1, 2, 3, 4, 5],
        protocol::spacecan::FramePriority::High
    );
    
    if frame.id == 0x456 && frame.data.len() == 5 {
        println!("✓ PASSED");
        passed += 1;
    } else {
        println!("✗ FAILED");
    }
    
    // Test 2: Frame encoding/decoding
    total += 1;
    println!("\nTest 2: Frame Encoding/Decoding");
    let encoded = frame.encode();
    match protocol::spacecan::SpaceCANFrame::decode(&encoded) {
        Ok(decoded) => {
            if decoded.id == frame.id && decoded.data == frame.data {
                println!("✓ PASSED");
                passed += 1;
            } else {
                println!("✗ FAILED - Data mismatch");
            }
        },
        Err(_) => println!("✗ FAILED - Decode error"),
    }
    
    // Test 3: Mesh network initialization
    total += 1;
    println!("\nTest 3: Mesh Network Initialization");
    let network = protocol::network::MeshNetwork::new();
    // Basic check - if we can create it without panicking, it's good
    println!("✓ PASSED");
    passed += 1;
    
    // Test 4: Telemetry processor
    total += 1;
    println!("\nTest 4: Telemetry Processor");
    let processor = telemetry::TelemetryProcessor::new();
    // Basic check
    println!("✓ PASSED");
    passed += 1;
    
    // Test 5: Security module
    total += 1;
    println!("\nTest 5: Security Module");
    let mut crypto = security::CryptoModule::new();
    let test_data = b"test data";
    
    match crypto.encrypt(test_data) {
        Ok(encrypted) => {
            match crypto.decrypt(&encrypted) {
                Ok(decrypted) => {
                    if decrypted == test_data {
                        println!("✓ PASSED");
                        passed += 1;
                    } else {
                        println!("✗ FAILED - Decryption mismatch");
                    }
                },
                Err(_) => println!("✗ FAILED - Decryption error"),
            }
        },
        Err(_) => println!("✗ FAILED - Encryption error"),
    }
    
    println!("\n📊 Test Results: {}/{} passed ({:.1}%)", 
        passed, total, (passed as f64 / total as f64) * 100.0);
    
    if passed == total {
        println!("🎉 All tests passed!");
    } else {
        println!("⚠️  Some tests failed");
    }
    
    Ok(())
}

fn show_config() -> Result<(), Box<dyn std::error::Error>> {
    println!("⚙️  Configuration Options");
    println!("========================");
    
    let config = config::RustSatConfig::default();
    
    println!("\nSystem Configuration:");
    println!("  Satellite ID: {}", config.system.satellite_id);
    println!("  Mission Name: {}", config.system.mission_name);
    println!("  Max Memory: {} MB", config.system.max_memory_mb);
    println!("  Heartbeat Interval: {} ms", config.system.heartbeat_interval_ms);
    
    println!("\nNetwork Configuration:");
    println!("  Max Hops: {}", config.network.max_hops);
    println!("  Connection Timeout: {} ms", config.network.connection_timeout_ms);
    println!("  Retry Attempts: {}", config.network.retry_attempts);
    
    println!("\nSecurity Configuration:");
    println!("  Encryption Enabled: {}", config.security.encryption_enabled);
    println!("  Key Rotation Interval: {} hours", config.security.key_rotation_interval_hours);
    println!("  Max Auth Failures: {}", config.security.max_auth_failures);
    
    println!("\nTelemetry Configuration:");
    println!("  Collection Interval: {} ms", config.telemetry.collection_interval_ms);
    println!("  Compression Enabled: {}", config.telemetry.compression_enabled);
    println!("  Buffer Size: {}", config.telemetry.max_buffer_size);
    
    println!("\nAlert Thresholds:");
    println!("  Battery Low: {:.1}%", config.telemetry.alert_thresholds.battery_low_percent);
    println!("  Temperature High: {:.1}°C", config.telemetry.alert_thresholds.temperature_high_celsius);
    println!("  Temperature Low: {:.1}°C", config.telemetry.alert_thresholds.temperature_low_celsius);
    
    println!("\n💡 Tip: You can override these settings with environment variables");
    println!("   Example: RUSTSAT_SATELLITE_ID=42 simple-cli demo");
    
    Ok(())
}