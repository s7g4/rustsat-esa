// Web dashboard for real-time satellite monitoring
// This shows practical web development skills alongside embedded systems

use warp::Filter;
use serde_json::json;
use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use chrono::{DateTime, Utc};
use crate::telemetry::{TelemetryData, TelemetryProcessor};
use crate::simulation::SpaceSimulator;

#[derive(Clone)]
pub struct DashboardState {
    pub telemetry_processor: Arc<Mutex<TelemetryProcessor>>,
    pub simulator: Arc<Mutex<SpaceSimulator>>,
    pub active_satellites: Arc<Mutex<HashMap<u32, SatelliteStatus>>>,
}

#[derive(Clone, serde::Serialize)]
pub struct SatelliteStatus {
    pub id: u32,
    pub name: String,
    pub position: (f64, f64, f64), // lat, lon, alt
    pub battery_level: f64,
    pub temperature: f64,
    pub signal_strength: f64,
    pub last_contact: DateTime<Utc>,
    pub status: String,
}

impl DashboardState {
    pub fn new() -> Self {
        Self {
            telemetry_processor: Arc::new(Mutex::new(TelemetryProcessor::new())),
            simulator: Arc::new(Mutex::new(SpaceSimulator::new())),
            active_satellites: Arc::new(Mutex::new(HashMap::new())),
        }
    }
    
    pub fn update_satellite_status(&self, satellite_id: u32, status: SatelliteStatus) {
        if let Ok(mut satellites) = self.active_satellites.lock() {
            satellites.insert(satellite_id, status);
        }
    }
}

pub async fn start_dashboard(port: u16) -> Result<(), Box<dyn std::error::Error>> {
    let state = DashboardState::new();
    
    // Initialize with some demo satellites
    let demo_satellites = vec![
        SatelliteStatus {
            id: 1,
            name: "CubeSat-Alpha".to_string(),
            position: (45.0, 2.0, 408.0),
            battery_level: 0.85,
            temperature: 22.5,
            signal_strength: 0.92,
            last_contact: Utc::now(),
            status: "Operational".to_string(),
        },
        SatelliteStatus {
            id: 2,
            name: "CubeSat-Beta".to_string(),
            position: (-12.0, 45.0, 412.0),
            battery_level: 0.78,
            temperature: 18.3,
            signal_strength: 0.87,
            last_contact: Utc::now(),
            status: "Operational".to_string(),
        },
        SatelliteStatus {
            id: 3,
            name: "CubeSat-Gamma".to_string(),
            position: (67.0, -15.0, 395.0),
            battery_level: 0.92,
            temperature: 25.1,
            signal_strength: 0.95,
            last_contact: Utc::now(),
            status: "Operational".to_string(),
        },
    ];
    
    for sat in demo_satellites {
        state.update_satellite_status(sat.id, sat);
    }
    
    let state_filter = warp::any().map(move || state.clone());
    
    // API Routes
    let api_satellites = warp::path!("api" / "satellites")
        .and(warp::get())
        .and(state_filter.clone())
        .and_then(get_satellites);
    
    let api_telemetry = warp::path!("api" / "telemetry" / u32)
        .and(warp::get())
        .and(state_filter.clone())
        .and_then(get_telemetry);
    
    let api_status = warp::path!("api" / "status")
        .and(warp::get())
        .and(state_filter.clone())
        .and_then(get_system_status);
    
    // Static files
    let static_files = warp::path("static")
        .and(warp::fs::dir("web/static"));
    
    // Main dashboard page
    let dashboard = warp::path::end()
        .and(warp::get())
        .map(|| {
            warp::reply::html(include_str!("dashboard.html"))
        });
    
    let routes = api_satellites
        .or(api_telemetry)
        .or(api_status)
        .or(static_files)
        .or(dashboard)
        .with(warp::cors().allow_any_origin());
    
    println!("🌐 Starting web dashboard on http://localhost:{}", port);
    println!("   Open your browser to view real-time satellite data");
    
    warp::serve(routes)
        .run(([127, 0, 0, 1], port))
        .await;
    
    Ok(())
}

async fn get_satellites(state: DashboardState) -> Result<impl warp::Reply, warp::Rejection> {
    if let Ok(satellites) = state.active_satellites.lock() {
        let satellite_list: Vec<&SatelliteStatus> = satellites.values().collect();
        Ok(warp::reply::json(&satellite_list))
    } else {
        Ok(warp::reply::json(&json!({"error": "Unable to fetch satellites"})))
    }
}

async fn get_telemetry(satellite_id: u32, state: DashboardState) -> Result<impl warp::Reply, warp::Rejection> {
    // Generate some sample telemetry data
    let telemetry_data = json!({
        "satellite_id": satellite_id,
        "timestamp": Utc::now(),
        "data": {
            "temperature": 22.5 + (satellite_id as f64 * 0.5),
            "battery_voltage": 7.2 + (satellite_id as f64 * 0.1),
            "solar_panel_current": 0.85 + (satellite_id as f64 * 0.05),
            "attitude": [0.1, -0.2, 0.05],
            "position": [45.0 + satellite_id as f64, 2.0 + satellite_id as f64, 408.0]
        }
    });
    
    Ok(warp::reply::json(&telemetry_data))
}

async fn get_system_status(state: DashboardState) -> Result<impl warp::Reply, warp::Rejection> {
    let status = json!({
        "system_status": "Operational",
        "active_satellites": 3,
        "ground_stations": 2,
        "uptime": "72h 15m",
        "data_rate": "1.2 MB/s",
        "last_updated": Utc::now()
    });
    
    Ok(warp::reply::json(&status))
}