use lib::add;
use tracing::{info, instrument};
use tracing_subscriber::EnvFilter;

#[instrument]
fn main() {
    // Set up the tracing subscriber.
    // This uses the `RUST_LOG` environment variable to control logging.
    // For example, `RUST_LOG=info` will show info-level logs.
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    info!("Hello, world!");
    let result = add(2, 2);
    info!(result, "2 + 2 = {}", result);
}
