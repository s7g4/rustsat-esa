use core::sync::atomic::{AtomicU32, Ordering};
use defmt::Format;

/// Core metrics structure designed for zero-allocation telemetry polling.
#[derive(Debug, Clone, Default, Format)]
pub struct PerformanceMetrics {
    pub message_count: u32,
    pub error_count: u32,
    pub total_latency_ms: u32,
    pub system_ticks: u32,
}

/// Global atomic metrics collector for zero-overhead performance tracking.
/// Uses lock-free atomics to prevent Priority Inversion and deadlocks on RTOS.
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
        self.message_count.fetch_add(1, Ordering::Relaxed);
    }

    pub fn record_error(&self) {
        self.error_count.fetch_add(1, Ordering::Relaxed);
    }

    pub fn record_latency(&self, latency_ms: u32) {
        self.total_latency_ms.fetch_add(latency_ms, Ordering::Relaxed);
    }

    pub fn record_tick(&self, ticks: u32) {
        self.system_ticks.fetch_add(ticks, Ordering::Relaxed);
    }

    pub fn get_metrics(&self) -> PerformanceMetrics {
        PerformanceMetrics {
            message_count: self.message_count.load(Ordering::Relaxed),
            error_count: self.error_count.load(Ordering::Relaxed),
            total_latency_ms: self.total_latency_ms.load(Ordering::Relaxed),
            system_ticks: self.system_ticks.load(Ordering::Relaxed),
        }
    }

    pub fn reset_metrics(&self) {
        self.message_count.store(0, Ordering::Relaxed);
        self.error_count.store(0, Ordering::Relaxed);
        self.total_latency_ms.store(0, Ordering::Relaxed);
        self.system_ticks.store(0, Ordering::Relaxed);
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
        $crate::metrics::GLOBAL_METRICS.record_latency_ms($latency_ms);
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_metrics_collection() {
        let collector = MetricsCollector::new();

        collector.record_message();
        collector.record_message();
        collector.record_error();
        collector.record_latency_ms(50);
        collector.record_latency_ms(100);

        let metrics = collector.get_metrics();

        assert_eq!(metrics.message_count, 2);
        assert_eq!(metrics.error_count, 1);
        assert_eq!(metrics.total_latency_ms, 150);
    }
}
