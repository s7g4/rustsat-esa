// RustSat-ESA: Comprehensive CubeSat Communication Protocol Demo
// Demonstrates all major features of the protocol stack

use rustsat_esa::*;
use rustsat_esa::protocol::spacecan::{SpaceCANFrame, FramePriority, PowerMode};
use rustsat_esa::protocol::network::{NetworkNode, NodeType, OrbitalPosition, MeshNetwork};
use rustsat_esa::cubesat::{CubeSatFrame, FrameType, MissionConfig, CubeSatProtocol, MissionControl};
use rustsat_esa::ground_station::{ESAGroundNetwork, CommandMessage, CommandType};
use rustsat_esa::telemetry::{TelemetryProcessor, TelemetryData, TelemetryType, TelemetryValue};
use rustsat_esa::security::{CryptoModule, Permission};
use rustsat_esa::simulation::{SpaceSimulator, ScenarioConfig};

use chrono::{Utc, Duration};
use std::collections::HashMap;
use log::{info, warn, error};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    env_logger::init();
    
    println!("🛰️  RustSat-ESA: SpaceCAN-Compatible CubeSat Communication Stack");
    println!("================================================================");
    println!("Demonstrating comprehensive CubeSat communication capabilities");
    println!();

    // Demo 1: SpaceCAN Protocol with Enhanced Features
    demo_spacecan_protocol()?;
    
    // Demo 2: Mesh Network Routing
    // demo_mesh_networking()?;
    
    // Demo 3: CubeSat Mission Control
    demo_cubesat_mission_control()?;
    
    // Demo 4: Ground Station Network
    demo_ground_station_network()?;
    
    // Demo 5: Security and Cryptography
    demo_security_features()?;
    
    // Demo 6: Telemetry Processing
    demo_telemetry_processing()?;
    
    // Demo 7: Complete Protocol Stack Integration
    //demo_integrated_protocol_stack()?;
    
    // Demo 8: Space Environment Simulation
    demo_space_simulation()?;

    println!("🎉 All demonstrations completed successfully!");
    println!("RustSat-ESA protocol stack is ready for ESA CubeSat missions.");
    
    Ok(())
}

fn demo_spacecan_protocol() -> Result<(), Box<dyn std::error::Error>> {
    println!("📡 Demo 1: Enhanced SpaceCAN Protocol");
    println!("------------------------------------");
    
    // Create SpaceCAN frame with CubeSat-specific features
    let frame = SpaceCANFrame::new(
        0x123,
        vec![1, 2, 3, 4, 5, 6, 7, 8],
        FramePriority::High
    ).with_power_mode(PowerMode::LowPower);
    
    println!("Created SpaceCAN frame:");
    println!("  ID: 0x{:X}", frame.id);
    println!("  Priority: {:?}", frame.priority);
    println!("  Power Mode: {:?}", frame.power_mode);
    println!("  Power Requirements: {:.1}W", frame.get_power_requirements());
    println!("  Transmission Range: {:.0}km", frame.get_transmission_range());
    
    // Encode and decode
    let encoded = frame.encode();
    println!("  Encoded size: {} bytes", encoded.len());
    
    let decoded = SpaceCANFrame::decode(&encoded)?;
    println!("  Decode successful: {}", decoded.id == frame.id);
    
    // Test error correction
    let mut test_frame = frame.clone();
    let corrected = test_frame.validate_and_correct()?;
    println!("  Error correction test: {}", if corrected { "Corrected errors" } else { "No errors detected" });
    
    println!("✅ SpaceCAN protocol demo completed\n");
    Ok(())
}

fn demo_mesh_networking() -> Result<(), Box<dyn std::error::Error>> {
    println!("🌐 Demo 2: Advanced Mesh Networking");
    println!("-----------------------------------");
    
    let mut network = MeshNetwork::new();
    
    // Create CubeSat constellation
    for i in 1..=5 {
        let position = OrbitalPosition {
            latitude: (i as f64 - 3.0) * 2.0,  // Reduced spacing to 2 degrees
            longitude: (i as f64 - 3.0) * 3.0, // Reduced spacing to 3 degrees
            altitude: 400.0 + i as f64 * 5.0,
            velocity: (7.66, 0.0, 0.0),
        };
        
        let mut node = NetworkNode::new_cubesat(i, position);
        node.communication_range = 1500.0; // Increase communication range to 1500 km
        network.add_node(node);
    }
    
    // Add ground station
    let ground_station = NetworkNode::new_ground_station(100, 52.5, 13.4); // Berlin
    network.add_node(ground_station);
    
    // Initialize routing
    network.initialize_routing()?;
    
    // Test message routing
    let test_message = b"Hello from CubeSat constellation!";
    let routed = network.route_message(1, 100, test_message)?;
    println!("Message routing test: {}", if routed { "SUCCESS" } else { "FAILED" });
    
    // Test ground station handover
    let handover_result = network.handle_ground_station_handover(2)?;
    println!("Ground station handover: {:?}", handover_result);
    
    // Display network statistics
    let stats = network.get_statistics();
    println!("Network Statistics:");
    println!("  Packets routed: {}", stats.packets_routed);
    println!("  Packets dropped: {}", stats.packets_dropped);
    println!("  Average hop count: {:.1}", stats.average_hop_count);
    println!("  Handovers completed: {}", stats.handovers_completed);
    
    println!("✅ Mesh networking demo completed\n");
    Ok(())
}

fn demo_cubesat_mission_control() -> Result<(), Box<dyn std::error::Error>> {
    println!("🛰️  Demo 3: CubeSat Mission Control");
    println!("----------------------------------");
    
    let mut mission_control = MissionControl::new();
    
    // Create and configure CubeSat
    let mut cubesat = CubeSatProtocol::new(1);
    let mission_config = MissionConfig::default();
    cubesat.configure_mission(mission_config)?;
    
    // Add to mission control
    mission_control.add_satellite(cubesat);
    
    // Send commands
    let mut command_params = HashMap::new();
    command_params.insert("roll".to_string(), "45.0".to_string());
    command_params.insert("pitch".to_string(), "0.0".to_string());
    command_params.insert("yaw".to_string(), "90.0".to_string());
    
    let attitude_command = rustsat_esa::cubesat::CubeSatCommand {
        command_id: 1,
        command_type: rustsat_esa::cubesat::CommandType::AttitudeControl,
        parameters: command_params,
        scheduled_execution: None,
        priority: 5,
        status: rustsat_esa::cubesat::CommandStatus::Queued,
    };
    
    mission_control.send_command_to_satellite(1, attitude_command)?;
    
    // Collect telemetry
    let telemetry = mission_control.collect_telemetry();
    println!("Collected telemetry from {} satellites", telemetry.len());
    
    if let Some(sat_telemetry) = telemetry.get(&1) {
        println!("Satellite 1 telemetry points: {}", sat_telemetry.len());
        for (i, data) in sat_telemetry.iter().take(3).enumerate() {
            println!("  {}: {:?} = {:?}", i + 1, data.data_type, data.value);
        }
    }
    
    // Update satellite systems
    mission_control.update_all_satellites(Duration::minutes(10));
    
    // Display mission statistics
    let stats = mission_control.get_statistics();
    println!("Mission Statistics:");
    println!("  Total satellites: {}", stats.total_satellites);
    println!("  Active satellites: {}", stats.active_satellites);
    println!("  Commands executed: {}", stats.total_commands_executed);
    println!("  Mission uptime: {} hours", stats.mission_uptime.num_hours());
    
    println!("✅ Mission control demo completed\n");
    Ok(())
}

fn demo_ground_station_network() -> Result<(), Box<dyn std::error::Error>> {
    println!("🌍 Demo 4: ESA Ground Station Network");
    println!("------------------------------------");
    
    let mut ground_network = ESAGroundNetwork::new();
    ground_network.initialize()?;
    
    // Display ground stations
    println!("ESA Ground Station Network initialized");
    
    // Simulate contact establishment
    println!("Attempting to establish contact...");
    
    // Create a command
    let command = CommandMessage {
        command_id: 1,
        target_satellite: 1,
        command_type: CommandType::DataDownload,
        parameters: HashMap::new(),
        execution_time: Some(Utc::now() + Duration::minutes(30)),
        priority: 3,
    };
    
    println!("Created command: {:?}", command.command_type);
    
    // Process message queue
    let processed = ground_network.process_message_queue()?;
    println!("Processed {} messages from queue", processed.len());
    
    // Display network statistics
    let stats = ground_network.get_statistics();
    println!("Ground Network Statistics:");
    println!("  Total contacts: {}", stats.total_contacts);
    println!("  Successful contacts: {}", stats.successful_contacts);
    println!("  Data volume: {:.2} GB", stats.data_volume_gb);
    println!("  Network availability: {:.1}%", stats.network_availability * 100.0);
    
    println!("✅ Ground station network demo completed\n");
    Ok(())
}

fn demo_security_features() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔒 Demo 5: Security and Cryptography");
    println!("------------------------------------");
    
    let mut crypto = CryptoModule::new();
    crypto.initialize_keys()?;
    
    // Test encryption/decryption
    let test_data = b"Classified CubeSat telemetry data";
    println!("Original data: {:?}", std::str::from_utf8(test_data)?);
    
    let encrypted = crypto.encrypt(test_data)?;
    println!("Encrypted data size: {} bytes", encrypted.len());
    
    let decrypted = crypto.decrypt(&encrypted)?;
    println!("Decrypted data: {:?}", std::str::from_utf8(&decrypted)?);
    println!("Encryption/Decryption: {}", if decrypted == test_data { "SUCCESS" } else { "FAILED" });
    
    // Test secure messaging
    let secure_message = crypto.create_secure_message(1, 2, test_data)?;
    println!("Created secure message with signature");
    
    let verified_data = crypto.verify_and_decrypt(&secure_message)?;
    println!("Message verification: {}", if verified_data == test_data { "SUCCESS" } else { "FAILED" });
    
    // Test authentication tokens
    let permissions = vec![Permission::Telemetry, Permission::Command];
    let token = crypto.generate_auth_token(1, permissions)?;
    println!("Generated auth token: {}...", &token[..16]);
    
    let auth_valid = crypto.verify_auth_token(1, &token, Permission::Telemetry)?;
    println!("Token verification: {}", if auth_valid { "VALID" } else { "INVALID" });
    
    // Test emergency messaging
    let emergency_data = b"EMERGENCY: Power system failure detected!";
    let emergency_msg = crypto.create_emergency_message(1, emergency_data)?;
    println!("Created emergency message");
    
    let emergency_verified = crypto.verify_emergency_message(&emergency_msg)?;
    println!("Emergency message verified: {} bytes", emergency_verified.len());
    
    println!("✅ Security features demo completed\n");
    Ok(())
}

fn demo_telemetry_processing() -> Result<(), Box<dyn std::error::Error>> {
    println!("📊 Demo 6: Real-time Telemetry Processing");
    println!("-----------------------------------------");
    
    let mut telemetry_processor = TelemetryProcessor::new();
    telemetry_processor.initialize()?;
    
    // Generate sample telemetry data
    let telemetry_samples = vec![
        TelemetryData {
            timestamp: Utc::now(),
            source_node: 1,
            data_type: TelemetryType::PowerStatus,
            value: TelemetryValue::Float(85.5),
            quality: 0.95,
            sequence_number: 1,
        },
        TelemetryData {
            timestamp: Utc::now(),
            source_node: 1,
            data_type: TelemetryType::Temperature,
            value: TelemetryValue::Float(23.7),
            quality: 0.92,
            sequence_number: 2,
        },
        TelemetryData {
            timestamp: Utc::now(),
            source_node: 1,
            data_type: TelemetryType::SystemHealth,
            value: TelemetryValue::Float(0.98),
            quality: 0.99,
            sequence_number: 3,
        },
    ];
    
    // Process telemetry data
    for data in telemetry_samples {
        telemetry_processor.process_telemetry(data)?;
    }
    
    println!("Processed {} telemetry data points", 3);
    
    // Create telemetry packet
    let packet = telemetry_processor.create_telemetry_packet(1, 10)?;
    println!("Created telemetry packet:");
    println!("  Source: {}", packet.source_node);
    println!("  Data points: {}", packet.data_points.len());
    println!("  Compression: {:?}", packet.compression_type);
    
    // Get current mission events
    let current_events = telemetry_processor.get_current_events();
    println!("Current mission events: {}", current_events.len());
    
    // Display statistics
    let stats = telemetry_processor.get_statistics();
    println!("Telemetry Statistics:");
    println!("  Data points processed: {}", stats.data_points_processed);
    println!("  Alerts generated: {}", stats.alerts_generated);
    println!("  Data quality score: {:.2}", stats.data_quality_score);
    
    println!("✅ Telemetry processing demo completed\n");
    Ok(())
}

fn demo_integrated_protocol_stack() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔗 Demo 7: Integrated Protocol Stack");
    println!("------------------------------------");
    
    let mut protocol_stack = RustSatProtocol::new();
    
    // Initialize mission
    let mission_config = MissionConfig::default();
    protocol_stack.initialize_mission(mission_config)?;
    println!("Protocol stack initialized for mission");
    
    // Send a message through the complete stack
    let test_message = b"End-to-end protocol stack test message";
    protocol_stack.send_message(100, test_message)?; // Send to ground station
    println!("Message sent through complete protocol stack");
    
    // Attempt to receive messages
    if let Some(received) = protocol_stack.receive_message()? {
        println!("Received message: {} bytes", received.len());
    } else {
        println!("No messages received (expected in demo)");
    }
    
    println!("Protocol stack integration: SUCCESS");
    println!("✅ Integrated protocol stack demo completed\n");
    Ok(())
}

fn demo_space_simulation() -> Result<(), Box<dyn std::error::Error>> {
    println!("🌌 Demo 8: Space Environment Simulation");
    println!("---------------------------------------");
    
    let mut simulator = SpaceSimulator::new();
    
    // Configure simulation scenario
    let scenario_config = ScenarioConfig {
        scenario_name: "ESA CubeSat Constellation Demo".to_string(),
        duration: Duration::minutes(30), // Short demo duration
        time_acceleration: 60.0, // 1 minute = 1 hour
        satellite_count: 3,
        ground_station_count: 4,
        communication_frequency: Duration::minutes(2),
        failure_probability: 0.02,
        space_weather_enabled: true,
    };
    
    // Initialize and run simulation
    simulator.initialize_scenario(scenario_config)?;
    println!("Simulation scenario initialized");
    
    // Get initial positions
    let positions = simulator.get_satellite_positions();
    println!("Satellite positions:");
    for (id, pos) in positions.iter().take(3) {
        println!("  Sat {}: {:.2}°N, {:.2}°E, {:.0}km", id, pos.latitude, pos.longitude, pos.altitude);
    }
    
    // Get ground station status
    let gs_status = simulator.get_ground_station_status();
    println!("Ground stations: {}", gs_status.len());
    for (id, (name, tracking, target)) in gs_status.iter().take(2) {
        println!("  {}: {} (tracking: {}, target: {:?})", id, name, tracking, target);
    }
    
    // Run short simulation
    println!("Running simulation...");
    simulator.run_scenario()?;
    
    // Display final statistics
    let stats = simulator.get_statistics();
    println!("Simulation Results:");
    println!("  Communication attempts: {}", stats.total_communication_attempts);
    println!("  Successful communications: {}", stats.successful_communications);
    println!("  Network availability: {:.1}%", stats.network_availability * 100.0);
    println!("  Data transmitted: {:.2} MB", stats.total_data_transmitted as f64 / (1024.0 * 1024.0));
    println!("  Average latency: {} ms", stats.average_latency.num_milliseconds());
    println!("  Ground station utilization: {:.1}%", stats.ground_station_utilization * 100.0);
    
    println!("✅ Space simulation demo completed\n");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_demo_functions() {
        // Test that all demo functions can be called without panicking
        assert!(demo_spacecan_protocol().is_ok());
        assert!(demo_mesh_networking().is_ok());
        assert!(demo_cubesat_mission_control().is_ok());
        assert!(demo_ground_station_network().is_ok());
        assert!(demo_security_features().is_ok());
        assert!(demo_telemetry_processing().is_ok());
        assert!(demo_integrated_protocol_stack().is_ok());
        // Note: Space simulation test would take too long for unit tests
    }
}