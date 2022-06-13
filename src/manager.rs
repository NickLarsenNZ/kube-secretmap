use std::{time::Duration, sync::Arc};

use chrono::{DateTime, Utc};
use crd::SecretMap;
use futures::{future::BoxFuture, FutureExt, StreamExt};
use kube::{api::ListParams, runtime::{controller::Action, Controller, events::Reporter}, Client, Api, ResourceExt, Resource, core::object::HasSpec};
use serde::Serialize;
use thiserror::Error;
use tokio::{sync::RwLock, time::Instant};
use tracing::*;

use crate::finalizer;

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
            // .owns(secrets, ListParams::default().labels(label_selector)) // todo: we can limit to owned secrets, but then how do we also watch shared secrets?
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
    #[error("SecretMap resources must have a namespace")]
    MissingNamespace,

    #[error("Kubernetes API Error: {0}")]
    KubeError(#[from] kube::Error),

    // #[error("Serialization Error: {0}")]
    // SerializationError(#[source] serde_json::Error),
}

pub enum ManagerAction {
    /// Create subresources, and add them as finalizers to the parent resource (if owned)
    Create,
    /// If owned, delete subresources and remove the applicable entry from the parent's finalalizers list
    Delete,
    /// The resource is in a desired state, no action required
    NoOp,
}

#[instrument(skip(secret_map, ctx), fields(trace_id))]
async fn reconcile(secret_map: Arc<SecretMap>, ctx: Arc<ContextData>) -> Result<Action, ManagerError> {
    // Setup tracing, and span
    use opentelemetry::trace::TraceContextExt as _; // opentelemetry::Context -> opentelemetry::trace::Span
    use tracing_opentelemetry::OpenTelemetrySpanExt as _; // tracing::Span to opentelemetry::Context
    let trace_id = Span::current().context().span().span_context().trace_id();
    Span::current().record("trace_id", &field::display(&trace_id));
    let _start = Instant::now(); // to record reconciliation duration


    // todo: Increase the reconciliations counter
    // ctx.metrics.reconciliations.inc();

    let name = secret_map.name();
    let namespace =  secret_map.namespace().ok_or(ManagerError::MissingNamespace)?;

    // do the needful
    ctx.state.write().await.last_event = Utc::now();
    let client = ctx.client.clone();
    let _reporter = ctx.state.read().await.reporter.clone();

    // todo: once the reconciliation loop is understood, need to make a flowchart of the cases to cover

    match determine_action(&secret_map) {
        ManagerAction::Create => {
            info!("doing create");
            // if owned:
            // - add a finalizer
            // - create the secret
            if secret_map.spec().secret_already_exists.is_none() {
                // mark the owned resources so we can clen up resources before the parent is deleted
                finalizer::add(client, name.as_str(), namespace.as_str()).await?; // todo: actually, we should always add finalizers so we can clean up secrets in shared resources (perhaps based on explicit configuration)
                warn!("to implement create for owned secret");
            } else {
            // else:
            // - append to existing secret
                warn!("to implement create for shared secret")

            }
            Ok(Action::requeue(Duration::from_secs(30)))
        },
        ManagerAction::Delete => {
            info!("doing delete");
            // if owned:
            // - delete the secret
            // - delete the finalizer
            if secret_map.spec().secret_already_exists.is_none() {
                warn!("to implement delete for owned secret");

                // mark the owned resources so we can clen up resources before the parent is deleted
                finalizer::delete(client, name.as_str(), namespace.as_str()).await?;
            } else {
                // else:
                // - ?
                warn!("this probably never gets seen")

            }
            Ok(Action::await_change())
        }
        ManagerAction::NoOp => {
            info!("noop");
            Ok(Action::requeue(Duration::from_secs(30)))
        },
    }

}

#[instrument(skip(_ctx))]
fn error_policy(error: &ManagerError, _ctx: Arc<ContextData>) -> Action {
    warn!("reconcile failed: {:?}", error);

    // increment the failure counter
    // ctx.metrics.failures.inc();
    // ctx.get_ref().metrics.failures.inc();

    // Reqeue after some time
    Action::requeue(Duration::from_secs(5 * 60))

}

// todo: rewrite this for clarity. It was mostly copied from https://github.com/Pscheidl/rust-kubernetes-operator-example/blob/fbf5ce138b05a952c0fa3595d1181766bbc82be7/src/main.rs#L141-L154
fn determine_action(resource: &SecretMap) -> ManagerAction {
    // If there is a deletion timestamp, we need to cleanup
    // Not sure if we need to worry about subresource deletion here
    let deletion = resource.meta().deletion_timestamp.is_some();

    // Creation should add finalizers (subresources to be cleaned up)
    let creation = resource.meta()
        .finalizers
        .as_ref()
        .map_or(true, |finalizers| finalizers.is_empty());

    match (deletion, creation) {
        (true, _) => ManagerAction::Delete,
        (_, true) => ManagerAction::Create,
        _ => ManagerAction::NoOp,
    }

}
