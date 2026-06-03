#![allow(missing_docs)]
use core::sync::atomic::{AtomicU32, Ordering};

/// Core metrics structure designed for zero-allocation telemetry polling.
#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct PerformanceMetrics {
    pub message_count: u32,
    pub error_count: u32,
    pub total_latency_ms: u32,
    pub system_ticks: u32,
}

/// Global atomic metrics collector for zero-overhead performance tracking.
/// Uses lock-free atomics to prevent Priority Inversion and deadlocks on RTOS.
#[derive(Debug)]
pub struct MetricsCollector {
    pub message_count: AtomicU32,
    pub error_count: AtomicU32,
    pub total_latency_ms: AtomicU32,
    pub system_ticks: AtomicU32,
}

impl MetricsCollector {
    pub const fn new() -> Self {
        Self {
            message_count: AtomicU32::new(0),
            error_count: AtomicU32::new(0),
            total_latency_ms: AtomicU32::new(0),
            system_ticks: AtomicU32::new(0),
        }
    }

    pub fn record_message(&self) {
        self.message_count.fetch_add(1, Ordering::SeqCst);
    }

    pub fn record_error(&self) {
        self.error_count.fetch_add(1, Ordering::SeqCst);
    }

    pub fn record_latency(&self, latency_ms: u32) {
        self.total_latency_ms
            .fetch_add(latency_ms, Ordering::SeqCst);
    }

    pub fn record_tick(&self, ticks: u32) {
        self.system_ticks.fetch_add(ticks, Ordering::SeqCst);
    }

    pub fn get_metrics(&self) -> PerformanceMetrics {
        PerformanceMetrics {
            message_count: self.message_count.load(Ordering::SeqCst),
            error_count: self.error_count.load(Ordering::SeqCst),
            total_latency_ms: self.total_latency_ms.load(Ordering::SeqCst),
            system_ticks: self.system_ticks.load(Ordering::SeqCst),
        }
    }

    pub fn reset_metrics(&self) {
        self.message_count.store(0, Ordering::SeqCst);
        self.error_count.store(0, Ordering::SeqCst);
        self.total_latency_ms.store(0, Ordering::SeqCst);
        self.system_ticks.store(0, Ordering::SeqCst);
    }
}

// Global static metrics collector. No lazy_static or Mutex required.
pub static GLOBAL_METRICS: MetricsCollector = MetricsCollector::new();

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
    ($latency_ms:expr) => {
        $crate::metrics::GLOBAL_METRICS.record_latency($latency_ms);
    };
}

impl Default for MetricsCollector {
    fn default() -> Self {
        Self::new()
    }
}
