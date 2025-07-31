// Example application demonstrating RustSat-ESA protocol usage

use rustsat_esa::protocol::spacecan::{SpaceCANFrame, FramePriority};
use rustsat_esa::protocol::network::MeshNetwork;
use rustsat_esa::cubesat::CubeSatProtocol;
use rustsat_esa::telemetry::TelemetryProcessor;
use rustsat_esa::security::CryptoModule;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("RustSat-ESA Basic Demo");
    println!("======================");

    // Initialize SpaceCAN frame
    println!("\nTesting SpaceCAN Protocol...");
    let test_data = vec![1, 2, 3, 4, 5, 6, 7, 8];
    let sc_frame = SpaceCANFrame::new(0x123, test_data.clone(), FramePriority::High);
    let encoded = sc_frame.encode();
    println!("SpaceCAN Frame created with ID: 0x{:X}", sc_frame.id);
    println!("Frame encoded: {} bytes", encoded.len());
    
    // Test decoding
    match SpaceCANFrame::decode(&encoded) {
        Ok(decoded) => {
            println!("Frame decoded successfully");
            println!("Data matches: {}", decoded.data == test_data);
        },
        Err(e) => println!("✗ Decode failed: {}", e),
    }

    // Setup mesh network
    println!("\nTesting Mesh Network...");
    let _network = MeshNetwork::new();
    println!("Mesh network initialized");
    
    // Initialize CubeSat protocol
    println!("\nTesting CubeSat Protocol...");
    let _cubesat = CubeSatProtocol::new(1);
    println!("CubeSat protocol initialized");

    // Test telemetry
    println!("\nTesting Telemetry System...");
    let _telemetry = TelemetryProcessor::new();
    println!("Telemetry processor initialized");

    // Test security
    println!("\nTesting Security Module...");
    let mut crypto = CryptoModule::new();
    let test_message = b"Hello from CubeSat!";
    
    match crypto.encrypt(test_message) {
        Ok(encrypted) => {
            println!("Message encrypted: {} bytes", encrypted.len());
            
            match crypto.decrypt(&encrypted) {
                Ok(decrypted) => {
                    println!("Message decrypted successfully");
                    println!("Data matches: {}", decrypted == test_message);
                },
                Err(e) => println!("✗ Decryption failed: {}", e),
            }
        },
        Err(e) => println!("✗ Encryption failed: {}", e),
    }

    println!("\nBasic demo completed successfully!");
    println!("Try running: cargo run --example comprehensive_demo");
    
    Ok(())
}
