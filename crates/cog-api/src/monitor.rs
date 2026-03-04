use cog_ndjson::protocol::CogEvent;
use std::collections::HashMap;
use std::time::Duration;

/// Default polling intervals per service.
pub fn default_interval(service: &str) -> Duration {
    match service {
        "gmail" => Duration::from_secs(30),
        "drive" => Duration::from_secs(60),
        "calendar" => Duration::from_secs(60),
        "keep" => Duration::from_secs(120),
        _ => Duration::from_secs(60),
    }
}

/// Monitor state for a single service.
#[derive(Debug)]
struct ServiceMonitor {
    service: String,
    interval: Duration,
    cursor: Option<String>,
    last_check: Option<std::time::Instant>,
}

/// Manages polling monitors for subscribed services.
pub struct MonitorManager {
    monitors: async_lock::Mutex<HashMap<String, ServiceMonitor>>,
    event_tx: async_channel::Sender<CogEvent>,
    event_rx: async_channel::Receiver<CogEvent>,
}

impl MonitorManager {
    pub fn new() -> Self {
        let (event_tx, event_rx) = async_channel::unbounded();
        Self {
            monitors: async_lock::Mutex::new(HashMap::new()),
            event_tx,
            event_rx,
        }
    }

    /// Subscribe to monitor events for the given services.
    pub async fn subscribe(&self, services: &[String], interval_override: Option<Duration>) {
        let mut monitors = self.monitors.lock().await;
        for service in services {
            let interval = interval_override.unwrap_or_else(|| default_interval(service));
            monitors.insert(
                service.clone(),
                ServiceMonitor {
                    service: service.clone(),
                    interval,
                    cursor: None,
                    last_check: None,
                },
            );
            tracing::info!("subscribed to {service} monitor (interval: {interval:?})");
        }
    }

    /// Unsubscribe from monitor events.
    pub async fn unsubscribe(&self, services: &[String]) {
        let mut monitors = self.monitors.lock().await;
        if services.is_empty() {
            monitors.clear();
            tracing::info!("unsubscribed from all monitors");
        } else {
            for service in services {
                monitors.remove(service);
                tracing::info!("unsubscribed from {service} monitor");
            }
        }
    }

    /// Get the event receiver for consuming monitor events.
    pub fn events(&self) -> async_channel::Receiver<CogEvent> {
        self.event_rx.clone()
    }

    /// Get current subscription status.
    pub async fn status(&self) -> Vec<(String, Duration, Option<String>)> {
        let monitors = self.monitors.lock().await;
        monitors
            .values()
            .map(|m| (m.service.clone(), m.interval, m.cursor.clone()))
            .collect()
    }

    /// Run the monitor polling loop.
    ///
    /// This should be spawned as a background task.
    pub async fn run(&self) {
        loop {
            smol::Timer::after(Duration::from_secs(1)).await;

            let monitors = self.monitors.lock().await;
            for monitor in monitors.values() {
                let should_poll = match monitor.last_check {
                    None => true,
                    Some(last) => last.elapsed() >= monitor.interval,
                };

                if should_poll {
                    // TODO: Poll the appropriate service API
                    // TODO: Emit CogEvent via event_tx
                    let _ = &self.event_tx;
                }
            }
        }
    }
}
