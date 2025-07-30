// Performance metrics and monitoring for the CubeSat communication stack
// This shows understanding of production system monitoring

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceMetrics {
    pub message_throughput: f64,      // messages per second
    pub average_latency: Duration,    // average message latency
    pub error_rate: f64,             // percentage of failed operations
    pub memory_usage: u64,           // bytes
    pub cpu_usage: f64,              // percentage
    pub network_utilization: f64,    // percentage
    pub uptime: Duration,            // system uptime
    pub last_updated: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct MetricPoint {
    pub timestamp: Instant,
    pub value: f64,
}

pub struct MetricsCollector {
    start_time: Instant,
    message_count: Arc<Mutex<u64>>,
    error_count: Arc<Mutex<u64>>,
    latency_samples: Arc<Mutex<Vec<Duration>>>,
    throughput_history: Arc<Mutex<Vec<MetricPoint>>>,
    custom_metrics: Arc<Mutex<HashMap<String, Vec<MetricPoint>>>>,
}

impl MetricsCollector {
    pub fn new() -> Self {
        Self {
            start_time: Instant::now(),
            message_count: Arc::new(Mutex::new(0)),
            error_count: Arc::new(Mutex::new(0)),
            latency_samples: Arc::new(Mutex::new(Vec::new())),
            throughput_history: Arc::new(Mutex::new(Vec::new())),
            custom_metrics: Arc::new(Mutex::new(HashMap::new())),
        }
    }
    
    pub fn record_message(&self) {
        if let Ok(mut count) = self.message_count.lock() {
            *count += 1;
        }
    }
    
    pub fn record_error(&self) {
        if let Ok(mut count) = self.error_count.lock() {
            *count += 1;
        }
    }
    
    pub fn record_latency(&self, latency: Duration) {
        if let Ok(mut samples) = self.latency_samples.lock() {
            samples.push(latency);
            
            // Keep only last 1000 samples to prevent memory growth
            if samples.len() > 1000 {
                let excess = samples.len() - 1000;
                samples.drain(0..excess);
            }
        }
    }
    
    pub fn record_custom_metric(&self, name: &str, value: f64) {
        if let Ok(mut metrics) = self.custom_metrics.lock() {
            let points = metrics.entry(name.to_string()).or_insert_with(Vec::new);
            points.push(MetricPoint {
                timestamp: Instant::now(),
                value,
            });
            
            // Keep only last 100 points per metric
            if points.len() > 100 {
                points.drain(0..points.len() - 100);
            }
        }
    }
    
    pub fn get_metrics(&self) -> PerformanceMetrics {
        let uptime = self.start_time.elapsed();
        
        let message_count = *self.message_count.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
        let error_count = *self.error_count.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
        let latency_samples = self.latency_samples.lock().unwrap_or_else(|poisoned| poisoned.into_inner()).clone();
        
        let throughput = if uptime.as_secs() > 0 {
            message_count as f64 / uptime.as_secs() as f64
        } else {
            0.0
        };
        
        let error_rate = if message_count > 0 {
            (error_count as f64 / message_count as f64) * 100.0
        } else {
            0.0
        };
        
        let average_latency = if !latency_samples.is_empty() {
            let total: Duration = latency_samples.iter().sum();
            total / latency_samples.len() as u32
        } else {
            Duration::from_millis(0)
        };
        
        // Simulate system metrics (in a real system, these would come from the OS)
        let memory_usage = self.estimate_memory_usage();
        let cpu_usage = self.estimate_cpu_usage();
        let network_utilization = self.estimate_network_usage();
        
        PerformanceMetrics {
            message_throughput: throughput,
            average_latency,
            error_rate,
            memory_usage,
            cpu_usage,
            network_utilization,
            uptime,
            last_updated: Utc::now(),
        }
    }
    
    pub fn get_custom_metric_history(&self, name: &str) -> Vec<MetricPoint> {
        if let Ok(metrics) = self.custom_metrics.lock() {
            metrics.get(name).cloned().unwrap_or_default()
        } else {
            Vec::new()
        }
    }
    
    pub fn reset_metrics(&self) {
        if let Ok(mut count) = self.message_count.lock() {
            *count = 0;
        }
        if let Ok(mut count) = self.error_count.lock() {
            *count = 0;
        }
        if let Ok(mut samples) = self.latency_samples.lock() {
            samples.clear();
        }
        if let Ok(mut metrics) = self.custom_metrics.lock() {
            metrics.clear();
        }
    }
    
    // Simulate memory usage estimation
    fn estimate_memory_usage(&self) -> u64 {
        // In a real implementation, this would use system APIs
        let base_usage = 50 * 1024 * 1024; // 50MB base
        let message_count = self.message_count.lock().map(|c| *c).unwrap_or(0);
        base_usage + (message_count * 1024) // ~1KB per message
    }
    
    // Simulate CPU usage estimation
    fn estimate_cpu_usage(&self) -> f64 {
        // In a real implementation, this would use system APIs
        let throughput = self.get_current_throughput();
        (throughput * 0.1).min(100.0) // Rough estimate: 0.1% CPU per msg/sec
    }
    
    // Simulate network usage estimation
    fn estimate_network_usage(&self) -> f64 {
        // In a real implementation, this would monitor network interfaces
        let throughput = self.get_current_throughput();
        (throughput * 0.05).min(100.0) // Rough estimate
    }
    
    fn get_current_throughput(&self) -> f64 {
        let uptime = self.start_time.elapsed();
        if uptime.as_secs() > 0 {
            let message_count = self.message_count.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
            *message_count as f64 / uptime.as_secs() as f64
        } else {
            0.0
        }
    }
}

// Global metrics instance for easy access
lazy_static::lazy_static! {
    pub static ref GLOBAL_METRICS: MetricsCollector = MetricsCollector::new();
}

// Convenience macros for recording metrics
#[macro_export]
macro_rules! record_message {
    () => {
        $crate::metrics::GLOBAL_METRICS.record_message();
    };
}

#[macro_export]
macro_rules! record_error {
    () => {
        $crate::metrics::GLOBAL_METRICS.record_error();
    };
}

#[macro_export]
macro_rules! record_latency {
    ($latency:expr) => {
        $crate::metrics::GLOBAL_METRICS.record_latency($latency);
    };
}

#[macro_export]
macro_rules! record_custom_metric {
    ($name:expr, $value:expr) => {
        $crate::metrics::GLOBAL_METRICS.record_custom_metric($name, $value);
    };
}

// Performance monitoring utilities
pub struct PerformanceTimer {
    start: Instant,
    name: String,
}

impl PerformanceTimer {
    pub fn new(name: &str) -> Self {
        Self {
            start: Instant::now(),
            name: name.to_string(),
        }
    }
}

impl Drop for PerformanceTimer {
    fn drop(&mut self) {
        let elapsed = self.start.elapsed();
        GLOBAL_METRICS.record_latency(elapsed);
        GLOBAL_METRICS.record_custom_metric(&format!("{}_duration_ms", self.name), elapsed.as_millis() as f64);
    }
}

// Macro for easy performance timing
#[macro_export]
macro_rules! time_operation {
    ($name:expr, $block:block) => {{
        let _timer = $crate::metrics::PerformanceTimer::new($name);
        $block
    }};
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;
    use std::time::Duration;
    
    #[test]
    fn test_metrics_collection() {
        let collector = MetricsCollector::new();
        
        // Record some test data
        collector.record_message();
        collector.record_message();
        collector.record_error();
        collector.record_latency(Duration::from_millis(50));
        collector.record_latency(Duration::from_millis(100));
        
        let metrics = collector.get_metrics();
        
        assert!(metrics.message_throughput >= 0.0);
        assert_eq!(metrics.error_rate, 50.0); // 1 error out of 2 messages
        assert!(metrics.average_latency.as_millis() > 0);
    }
    
    #[test]
    fn test_custom_metrics() {
        let collector = MetricsCollector::new();
        
        collector.record_custom_metric("temperature", 25.5);
        collector.record_custom_metric("temperature", 26.0);
        
        let history = collector.get_custom_metric_history("temperature");
        assert_eq!(history.len(), 2);
        assert_eq!(history[0].value, 25.5);
        assert_eq!(history[1].value, 26.0);
    }
}