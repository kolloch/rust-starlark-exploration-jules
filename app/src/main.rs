use lib::add;
use tracing::{info, instrument};

#[instrument]
fn main() {
    // Set up the tracing subscriber.
    tracing_subscriber::fmt::init();

    info!("Hello, world!");
    let result = add(2, 2);
    info!(result, "2 + 2 = {}", result);
}
