use tracing::{subscriber, Subscriber};
use tracing_log::LogTracer;

pub fn init_subscriber(subscriber: impl Subscriber + Send + Sync) {
    LogTracer::init().expect("Failed to set logger");
    subscriber::set_global_default(subscriber).expect("Failed to set tracing subscriber");
}
