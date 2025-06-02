use metrics::{counter, histogram};

pub fn record_metrics(endpoint: &str, status: &str, duration: f64) {
    let full_counter_name = format!("requests_total{{endpoint=\"{}\",status=\"{}\"}}", endpoint, status);
    let full_histogram_name = format!("request_duration_seconds{{endpoint=\"{}\",status=\"{}\"}}", endpoint, status);

    let counter = counter!(full_counter_name);
    counter.increment(1);

    let histogram = histogram!(full_histogram_name);
    histogram.record(duration);
}
