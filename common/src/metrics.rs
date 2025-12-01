pub mod statsd_metrics_collector;

use {anyhow::Result, log::info, serde_json::Value, std::time::Duration};

pub trait MetricKey {
    fn to_str(&self) -> &str;
}

pub trait MetricsCollector {
    fn new(config: Value) -> Self;

    fn try_gauge(&self, key: &dyn MetricKey, value: f64) -> Result<()>;
    fn gauge(&self, key: &dyn MetricKey, value: f64) {
        let _ = self.try_gauge(key, value);
    }

    fn try_duration(&self, key: &dyn MetricKey, value: Duration) -> Result<()>;
    fn duration(&self, key: &dyn MetricKey, value: Duration) {
        let _ = self.try_duration(key, value);
    }

    fn try_time_sec(&self, key: &dyn MetricKey, value: u64) -> Result<()>;
    fn time_sec(&self, key: &dyn MetricKey, value: u64) {
        let _ = self.try_time_sec(key, value);
    }
}

pub struct NoopMetricsCollector {}

impl MetricsCollector for NoopMetricsCollector {
    fn new(_config: Value) -> Self {
        info!("Creating NoopMetricscollector");
        NoopMetricsCollector {}
    }

    fn try_gauge(&self, _key: &dyn MetricKey, _value: f64) -> Result<()> {
        Ok(())
    }

    fn try_duration(&self, _key: &dyn MetricKey, _value: Duration) -> Result<()> {
        Ok(())
    }

    fn try_time_sec(&self, _key: &dyn MetricKey, _value: u64) -> Result<()> {
        Ok(())
    }
}
