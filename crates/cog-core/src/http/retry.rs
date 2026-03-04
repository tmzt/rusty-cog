use std::time::Duration;

/// Retry configuration for HTTP requests.
#[derive(Debug, Clone)]
pub struct RetryConfig {
    /// Maximum retries for any error (total).
    pub max_retries: u32,
    /// Maximum retries specifically for 429 (rate limited).
    pub max_429_retries: u32,
    /// Maximum retries specifically for 5xx (server error).
    pub max_5xx_retries: u32,
    /// Base delay for exponential backoff.
    pub base_delay: Duration,
    /// Maximum delay cap.
    pub max_delay: Duration,
    /// Jitter factor (0.0 - 1.0).
    pub jitter: f64,
}

impl Default for RetryConfig {
    fn default() -> Self {
        Self {
            max_retries: 3,
            max_429_retries: 3,
            max_5xx_retries: 1,
            base_delay: Duration::from_millis(500),
            max_delay: Duration::from_secs(30),
            jitter: 0.25,
        }
    }
}

impl RetryConfig {
    /// Calculate delay for a given attempt (0-indexed).
    pub fn delay_for_attempt(&self, attempt: u32) -> Duration {
        let base_ms = self.base_delay.as_millis() as f64;
        let delay_ms = base_ms * 2.0f64.powi(attempt as i32);
        let max_ms = self.max_delay.as_millis() as f64;
        let capped = delay_ms.min(max_ms);

        // Add jitter
        let jitter_range = capped * self.jitter;
        let jitter = (rand::random::<f64>() - 0.5) * 2.0 * jitter_range;
        let final_ms = (capped + jitter).max(0.0);

        Duration::from_millis(final_ms as u64)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_config() {
        let config = RetryConfig::default();
        assert_eq!(config.max_retries, 3);
        assert_eq!(config.max_429_retries, 3);
        assert_eq!(config.max_5xx_retries, 1);
    }

    #[test]
    fn exponential_backoff_increases() {
        let config = RetryConfig {
            jitter: 0.0,
            ..Default::default()
        };
        let d0 = config.delay_for_attempt(0);
        let d1 = config.delay_for_attempt(1);
        let d2 = config.delay_for_attempt(2);
        assert!(d1 > d0, "delay should increase: {d0:?} < {d1:?}");
        assert!(d2 > d1, "delay should increase: {d1:?} < {d2:?}");
    }

    #[test]
    fn delay_capped_at_max() {
        let config = RetryConfig {
            jitter: 0.0,
            max_delay: Duration::from_secs(1),
            ..Default::default()
        };
        let d = config.delay_for_attempt(100);
        assert!(d <= Duration::from_secs(1));
    }
}
