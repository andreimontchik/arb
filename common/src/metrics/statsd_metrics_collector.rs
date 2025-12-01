use {
    super::{MetricKey, MetricsCollector},
    anyhow::Result,
    cadence::{BufferedUdpMetricSink, Gauged, Metered, StatsdClient, Timed},
    log::info,
    serde::Deserialize,
    serde_json::Value,
    std::{net::UdpSocket, time::Duration},
};

#[derive(Deserialize)]
pub struct StatsdMetricsConfig {
    host: String,
    port: u16,
    prefix: String,
}

impl StatsdMetricsConfig {
    pub fn new(json: Value) -> Self {
        serde_json::from_value(json).unwrap()
    }
}

pub struct StatsdMetricsCollector {
    client: StatsdClient,
}

impl MetricsCollector for StatsdMetricsCollector {
    fn new(config: serde_json::Value) -> Self {
        info!("Creating StatsdMetricsCollector. Config: `{}`", config);
        let config = StatsdMetricsConfig::new(config);

        let socket = UdpSocket::bind("0.0.0.0:0").unwrap();
        // let sink = UdpMetricSink::from((config.host, config.port), socket).unwrap();
        let sink = BufferedUdpMetricSink::from((config.host, config.port), socket).unwrap();
        /* Turns out that the combination of BufferedUdpMetricSink and QueingmetricsSink is not very suitable when the metrics
           volume is high. It panics with the `channel full` exception that is probably coming off the crossmeam library.
                let queuing_sink = QueuingMetricSinkBuilder::new()
                    .with_capacity(config.queue_size)
                    .with_error_handler(|e| {
                        eprintln!("Failed to send metrics!: {:?}", e);
                    })
                    .build(sink);
        */
        // let client = StatsdClient::from_sink(&config.prefix, queuing_sink);
        let client = StatsdClient::from_sink(&config.prefix, sink);
        StatsdMetricsCollector { client }
    }

    fn try_gauge(&self, key: &dyn MetricKey, value: f64) -> Result<()> {
        self.client.gauge(key.to_str(), value)?;
        Ok(())
    }

    fn try_duration(&self, key: &dyn MetricKey, value: Duration) -> Result<()> {
        self.client.meter(key.to_str(), value.as_micros() as u64)?;
        Ok(())
    }

    fn try_time_sec(&self, key: &dyn MetricKey, value: u64) -> Result<()> {
        self.client.time(key.to_str(), value)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::metrics::statsd_metrics_collector::StatsdMetricsConfig;

    #[test]
    fn test_metrics_configuration() {
        let host = "test-host";
        let port = 1234u16;
        let prefix = "test-prefix";

        let config_str = format!(
            r#"
            {{
                "host": "{}",
                "port": {},
                "prefix": "{}"
            }}"#,
            host, port, prefix
        );
        let config = StatsdMetricsConfig::new(serde_json::from_str(&config_str).unwrap());
        assert_eq!(config.host, host);
        assert_eq!(config.port, port);
        assert_eq!(config.prefix, prefix);
    }
}
