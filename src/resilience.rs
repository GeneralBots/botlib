
use std::future::Future;
use std::sync::atomic::{AtomicU32, AtomicU64, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::{RwLock, Semaphore, SemaphorePermit};
use tokio::time::{sleep, timeout};

#[derive(Debug, Clone)]
pub enum ResilienceError {
    Timeout { duration: Duration },
    CircuitOpen { until: Option<Duration> },
    RetriesExhausted {
        attempts: u32,
        last_error: String,
    },
    BulkheadFull { max_concurrent: usize },
    Operation(String),
}

impl std::fmt::Display for ResilienceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Timeout { duration } => {
                write!(f, "Operation timed out after {:?}", duration)
            }
            Self::CircuitOpen { until } => {
                if let Some(d) = until {
                    write!(f, "Circuit breaker open, retry after {:?}", d)
                } else {
                    write!(f, "Circuit breaker open")
                }
            }
            Self::RetriesExhausted {
                attempts,
                last_error,
            } => {
                write!(
                    f,
                    "All {} retry attempts exhausted. Last error: {}",
                    attempts, last_error
                )
            }
            Self::BulkheadFull { max_concurrent } => {
                write!(
                    f,
                    "Bulkhead full, max {} concurrent requests",
                    max_concurrent
                )
            }
            Self::Operation(msg) => write!(f, "Operation failed: {}", msg),
        }
    }
}

impl std::error::Error for ResilienceError {}


#[derive(Debug, Clone)]
pub struct RetryConfig {
    pub max_attempts: u32,
    pub initial_delay: Duration,
    pub max_delay: Duration,
    pub backoff_multiplier: f64,
    pub jitter_factor: f64,
    retryable: Option<Arc<dyn Fn(&str) -> bool + Send + Sync>>,
}

impl Default for RetryConfig {
    fn default() -> Self {
        Self {
            max_attempts: 3,
            initial_delay: Duration::from_millis(100),
            max_delay: Duration::from_secs(30),
            backoff_multiplier: 2.0,
            jitter_factor: 0.2,
            retryable: None,
        }
    }
}

impl RetryConfig {
    pub fn with_max_attempts(mut self, attempts: u32) -> Self {
        self.max_attempts = attempts.max(1);
        self
    }

    pub fn with_initial_delay(mut self, delay: Duration) -> Self {
        self.initial_delay = delay;
        self
    }

    pub fn with_max_delay(mut self, delay: Duration) -> Self {
        self.max_delay = delay;
        self
    }

    pub fn with_backoff_multiplier(mut self, multiplier: f64) -> Self {
        self.backoff_multiplier = multiplier.max(1.0);
        self
    }

    pub fn with_jitter(mut self, jitter: f64) -> Self {
        self.jitter_factor = jitter.clamp(0.0, 1.0);
        self
    }

    pub fn with_retryable<F>(mut self, predicate: F) -> Self
    where
        F: Fn(&str) -> bool + Send + Sync + 'static,
    {
        self.retryable = Some(Arc::new(predicate));
        self
    }

    pub fn aggressive() -> Self {
        Self {
            max_attempts: 5,
            initial_delay: Duration::from_millis(50),
            max_delay: Duration::from_secs(10),
            backoff_multiplier: 1.5,
            jitter_factor: 0.3,
            retryable: None,
        }
    }

    pub fn conservative() -> Self {
        Self {
            max_attempts: 2,
            initial_delay: Duration::from_millis(500),
            max_delay: Duration::from_secs(5),
            backoff_multiplier: 2.0,
            jitter_factor: 0.1,
            retryable: None,
        }
    }

    fn calculate_delay(&self, attempt: u32) -> Duration {
        let base_delay = self.initial_delay.as_secs_f64()
            * self.backoff_multiplier.powi(attempt.saturating_sub(1) as i32);

        let capped_delay = base_delay.min(self.max_delay.as_secs_f64());

        let jitter = if self.jitter_factor > 0.0 {
            let jitter_range = capped_delay * self.jitter_factor;
            let pseudo_random = ((attempt as f64 * 1.618033988749895) % 1.0) * 2.0 - 1.0;
            jitter_range * pseudo_random
        } else {
            0.0
        };

        Duration::from_secs_f64((capped_delay + jitter).max(0.001))
    }

    fn is_retryable(&self, error: &str) -> bool {
        if let Some(ref predicate) = self.retryable {
            predicate(error)
        } else {
