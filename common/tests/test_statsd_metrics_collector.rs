#[cfg(test)]
mod tests {
    use {
        common::metrics::{
            statsd_metrics_collector::StatsdMetricsCollector, MetricKey, MetricsCollector,
        },
        rand::Rng,
        serde_json::Value,
        std::{
            thread,
            time::{Duration, Instant, SystemTime, UNIX_EPOCH},
        },
    };
    enum TestMetricKey {
        TestDuration,
        TestGauge,
        TestTime,
    }

    impl MetricKey for TestMetricKey {
        fn to_str(&self) -> &str {
            match self {
                TestMetricKey::TestDuration => "test-duration-us",
                TestMetricKey::TestGauge => "test-gauge-us",
                TestMetricKey::TestTime => "test-time-sec",
            }
        }
    }

    #[test]
    #[ignore]
    fn test_send_metrics() {
        let config_str = format!(
            r#"
            {{
                "host": "76.16.6.193",
                "port": 18125,
                "prefix": "test-plugin"
            }}"#,
        );
        let config: Value = serde_json::from_str(&config_str).unwrap();
        let collector = StatsdMetricsCollector::new(config);

        let mut rng = rand::thread_rng();

        for i in 0..1_000_000 {
            thread::sleep(Duration::from_micros(1));
            let start = Instant::now();
            let time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() as f64;
            //                * rng.gen_range(0.0..1.0);
            collector
                .try_duration(&TestMetricKey::TestDuration, start.elapsed())
                .unwrap();
            collector
                .try_gauge(&TestMetricKey::TestGauge, i as f64 * rng.gen_range(0.0..1.0))
                .unwrap();
            collector
                .try_time_sec(&TestMetricKey::TestTime, time as u64)
                .unwrap();
        }
    }
}
