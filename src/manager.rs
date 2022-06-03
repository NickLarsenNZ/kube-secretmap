use std::{time::Duration, sync::Arc};

use chrono::{DateTime, Utc};
use crd::SecretMap;
use futures::{future::BoxFuture, FutureExt, StreamExt};
use kube::{api::ListParams, runtime::{controller::Action, Controller, events::Reporter}, Client, Api};
use serde::Serialize;
use thiserror::Error;
use tokio::{sync::RwLock, time::Instant};
use tracing::*;

/// State that we can expose (eg: web server)
#[derive(Clone, Serialize)]
pub struct State {
    #[serde(deserialize_with = "from_ts")]
    pub last_event: DateTime<Utc>,
    #[serde(skip)]
    pub reporter: Reporter,
}

impl State {
    pub fn new() -> Self {
        Self {
            last_event: Utc::now(),
            reporter: "secretmap-controller".into()
        }
    }
}

/// Data owned by the Manager
#[derive(Clone)]
pub struct Manager {
    /// In memory state
    // Note: it is here so we can expose the state.
    // The only reason we don't also expose metrics here too is because we are storing metrics in the default_registry
    state: Arc<RwLock<State>>,

}

impl Manager {
    // todo: Might want a Result, and do a pre-flight check to make sure the CRD is registered in the cluster, otherwise we get 404s
    pub async fn new() -> (Self, BoxFuture<'static, ()>) {
        let client = Client::try_default().await.expect("can't make a k8s client");
        // let metrics = Metrics::new();
        let state = Arc::new(RwLock::new(State::new()));
        let context = Arc::new(ContextData {
            client: client.clone(),
            // metrics.clone(),
            state: state.clone(),
        });

        let secret_maps: Api<SecretMap> = Api::all(client);
        let controller = Controller::new(secret_maps, ListParams::default())
            .run(reconcile, error_policy, context)
            .filter_map(|x| async move { std::result::Result::ok(x) })
            .for_each(|_| futures::future::ready(()))
            .boxed();

        (Self { state }, controller)

    }

    // todo: impl metrics(&self) -> Vec<MetricFamily> for our metrics server to use

    // todo: impl state(&self) -> State for our metrics server to use
}

/// This is the context that the reconciler is given so it can use the client, update metrics, and the state (to get the reporter, and update the last_event timestamp)
struct ContextData {
    client: Client,
    state: Arc<RwLock<State>>,
    // metrics: Metrics,
}

#[derive(Error, Debug)]
pub enum ManagerError {
    // #[error("Kubernetes API Error: {0}")]
    // KubernetesApiError(#[source] kube::Error),

    // #[error("Serialization Error: {0}")]
    // SerializationError(#[source] serde_json::Error),
}

#[instrument(skip(_secret_map, ctx), fields(trace_id))]
async fn reconcile(_secret_map: Arc<SecretMap>, ctx: Arc<ContextData>) -> Result<Action, ManagerError> {
    // Setup tracing, and span
    use opentelemetry::trace::TraceContextExt as _; // opentelemetry::Context -> opentelemetry::trace::Span
    use tracing_opentelemetry::OpenTelemetrySpanExt as _; // tracing::Span to opentelemetry::Context
    let trace_id = Span::current().context().span().span_context().trace_id();
    Span::current().record("trace_id", &field::display(&trace_id));
    let _start = Instant::now(); // to record reconciliation duration


    // todo: Increase the reconciliations counter
    // ctx.metrics.reconciliations.inc();

    // do the needful
    ctx.state.write().await.last_event = Utc::now();
    let _client = ctx.client.clone();
    let _reporter = ctx.state.read().await.reporter.clone();

    // update the status of the resource
    info!("would have done some reconciliation");

    // No event received, check again in 30 seconds
    Ok(Action::requeue(Duration::from_secs(30)))
}

#[instrument(skip(_ctx))]
fn error_policy(error: &ManagerError, _ctx: Arc<ContextData>) -> Action {
    warn!("reconcile failed: {:?}", error);
    info!("I am here");

    // increment the failure counter
    //ctx.get_ref().metrics.failures.inc();

    // Reqeue after some time
    Action::requeue(Duration::from_secs(5 * 60))

}
