// Application state management with thread-safe access.

use rmx::prelude::*;
use std::sync::Arc;
use std::time::Instant;
use rmx::tokio::sync::RwLock;

// Global application state.
#[derive(Debug, Clone)]
pub struct AppState {
    inner: Arc<RwLock<AppStateInner>>,
}

#[derive(Debug)]
struct AppStateInner {
    pub config: crate::infrastructure::config::Config,
    pub start_time: Instant,
    pub metrics: Metrics,
    pub cache: Cache,
}

// Application metrics.
#[derive(Debug, Default)]
pub struct Metrics {
    pub requests_total: u64,
    pub requests_success: u64,
    pub requests_failed: u64,
    pub operations_total: u64,
    pub cache_hits: u64,
    pub cache_misses: u64,
}

// Simple LRU cache using ahash.
#[derive(Debug)]
pub struct Cache {
    data: rmx::ahash::AHashMap<String, CacheEntry>,
    max_size: usize,
    access_order: Vec<String>,
}

#[derive(Debug, Clone)]
struct CacheEntry {
    value: Vec<u8>,
    created_at: Instant,
    accessed_at: Instant,
    access_count: u64,
}

impl AppState {
    // Create new application state.
    pub fn new(config: crate::infrastructure::config::Config) -> Self {
        let max_cache_size = config.storage.max_cache_size_mb * 1024 * 1024;

        Self {
            inner: Arc::new(RwLock::new(AppStateInner {
                config,
                start_time: Instant::now(),
                metrics: Metrics::default(),
                cache: Cache::new(max_cache_size),
            })),
        }
    }

    // Get configuration.
    pub async fn config(&self) -> crate::infrastructure::config::Config {
        self.inner.read().await.config.clone()
    }

    // Update configuration.
    pub async fn update_config(&self, config: crate::infrastructure::config::Config) {
        self.inner.write().await.config = config;
    }

    // Get uptime in seconds.
    pub async fn uptime_secs(&self) -> u64 {
        self.inner.read().await.start_time.elapsed().as_secs()
    }

    // Increment metric.
    pub async fn increment_metric(&self, metric: MetricType) {
        let mut state = self.inner.write().await;
        match metric {
            MetricType::RequestTotal => state.metrics.requests_total += 1,
            MetricType::RequestSuccess => state.metrics.requests_success += 1,
            MetricType::RequestFailed => state.metrics.requests_failed += 1,
            MetricType::OperationTotal => state.metrics.operations_total += 1,
            MetricType::CacheHit => state.metrics.cache_hits += 1,
            MetricType::CacheMiss => state.metrics.cache_misses += 1,
        }
    }

    // Get metrics snapshot.
    pub async fn metrics_snapshot(&self) -> MetricsSnapshot {
        let state = self.inner.read().await;
        MetricsSnapshot {
            uptime_secs: state.start_time.elapsed().as_secs(),
            requests_total: state.metrics.requests_total,
            requests_success: state.metrics.requests_success,
            requests_failed: state.metrics.requests_failed,
            operations_total: state.metrics.operations_total,
            cache_hits: state.metrics.cache_hits,
            cache_misses: state.metrics.cache_misses,
            cache_size: state.cache.size(),
            cache_entries: state.cache.len(),
        }
    }

    // Cache operations.
    pub async fn cache_get(&self, key: &str) -> Option<Vec<u8>> {
        let mut state = self.inner.write().await;
        if let Some(value) = state.cache.get(key) {
            state.metrics.cache_hits += 1;
            Some(value)
        } else {
            state.metrics.cache_misses += 1;
            None
        }
    }

    pub async fn cache_set(&self, key: String, value: Vec<u8>) {
        let mut state = self.inner.write().await;
        state.cache.set(key, value);
    }

    pub async fn cache_remove(&self, key: &str) -> Option<Vec<u8>> {
        let mut state = self.inner.write().await;
        state.cache.remove(key)
    }

    pub async fn cache_clear(&self) {
        let mut state = self.inner.write().await;
        state.cache.clear();
    }
}

// Metric types.
#[derive(Debug, Clone, Copy)]
pub enum MetricType {
    RequestTotal,
    RequestSuccess,
    RequestFailed,
    OperationTotal,
    CacheHit,
    CacheMiss,
}

// Metrics snapshot for reporting.
#[derive(Debug, Clone)]
pub struct MetricsSnapshot {
    pub uptime_secs: u64,
    pub requests_total: u64,
    pub requests_success: u64,
    pub requests_failed: u64,
    pub operations_total: u64,
    pub cache_hits: u64,
    pub cache_misses: u64,
    pub cache_size: usize,
    pub cache_entries: usize,
}

impl Cache {
    fn new(max_size: usize) -> Self {
        Self {
            data: rmx::ahash::AHashMap::new(),
            max_size,
            access_order: Vec::new(),
        }
    }

    fn get(&mut self, key: &str) -> Option<Vec<u8>> {
        if let Some(entry) = self.data.get_mut(key) {
            entry.accessed_at = Instant::now();
            entry.access_count += 1;

            // Move to end of access order.
            if let Some(pos) = self.access_order.iter().position(|k| k == key) {
                self.access_order.remove(pos);
            }
            self.access_order.push(key.to_string());

            Some(entry.value.clone())
        } else {
            None
        }
    }

    fn set(&mut self, key: String, value: Vec<u8>) {
        let entry = CacheEntry {
            value,
            created_at: Instant::now(),
            accessed_at: Instant::now(),
            access_count: 1,
        };

        // Check if we need to evict.
        while self.size() + entry.value.len() > self.max_size && !self.access_order.is_empty() {
            // Remove least recently used.
            let lru_key = self.access_order.remove(0);
            self.data.remove(&lru_key);
        }

        self.data.insert(key.clone(), entry);
        self.access_order.push(key);
    }

    fn remove(&mut self, key: &str) -> Option<Vec<u8>> {
        if let Some(pos) = self.access_order.iter().position(|k| k == key) {
            self.access_order.remove(pos);
        }
        self.data.remove(key).map(|entry| entry.value)
    }

    fn clear(&mut self) {
        self.data.clear();
        self.access_order.clear();
    }

    fn size(&self) -> usize {
        self.data.values().map(|e| e.value.len()).sum()
    }

    fn len(&self) -> usize {
        self.data.len()
    }
}