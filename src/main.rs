use manager::Manager;
use tracing::{info, warn};
use tracing_subscriber::layer::SubscriberExt;

mod metrics_server;
mod manager;
mod provider;
mod finalizer;

#[tokio::main]
async fn main() {

    // Create a stdout logging layer
    let logger = tracing_subscriber::fmt::layer();

    // Allow setting RUST_LOG level, or fallback to some level
    let fallback = "info";
    let env_filter = tracing_subscriber::EnvFilter::try_from_default_env()
        .or_else(|_| tracing_subscriber::EnvFilter::try_new(fallback))
        .unwrap();

    // Create a collector?
    let subscriber = tracing_subscriber::Registry::default()
        .with(logger) // .with(layer) Requires tracing_subscriber::layer::SubscriberExt
        .with(env_filter);

    // Initialize tracing
    tracing::subscriber::set_global_default(subscriber).expect("initialize tracing subscriber");

    // Start the controller
    let (_manager, controller) = Manager::new().await;


    // finish up when the first one completes
    tokio::select! {
        _ = controller => warn!("controller exited"),
        _ = metrics_server::run("0.0.0.0:8080") => info!("metrics server shutdown"), // todo: pass Metics from the controller manager as actix_web app_data
    }

}
