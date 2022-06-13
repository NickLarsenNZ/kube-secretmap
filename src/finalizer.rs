use crd::SecretMap;
use kube::api::{Patch, PatchParams};
use kube::{Api, Client, Error};
use serde_json::{json, Value};

// from: https://github.com/Pscheidl/rust-kubernetes-operator-example/blob/master/src/finalizer.rs

/// Adds a finalizer record into an `SecretMap` kind of resource. If the finalizer already exists,
/// this action has no effect.
///
/// # Arguments:
/// - `client` - Kubernetes client to modify the `SecretMap` resource with.
/// - `name` - Name of the `SecretMap` resource to modify. Existence is not verified
/// - `namespace` - Namespace where the `SecretMap` resource with given `name` resides.
///
/// Note: Does not check for resource's existence for simplicity.
pub async fn add(client: Client, name: &str, namespace: &str) -> Result<SecretMap, Error> {
    let api: Api<SecretMap> = Api::namespaced(client, namespace);
    let finalizer: Value = json!({
        "metadata": {
            "finalizers": ["secretmaps.example.com/finalizer"] // todo: fix the domain name
        }
    });

    let patch: Patch<&Value> = Patch::Merge(&finalizer);
    api.patch(name, &PatchParams::default(), &patch).await
}

/// Removes all finalizers from a `SecretMap` resource. If there are no finalizers already, this
/// action has no effect.
///
/// # Arguments:
/// - `client` - Kubernetes client to modify the `SecretMap` resource with.
/// - `name` - Name of the `SecretMap` resource to modify. Existence is not verified
/// - `namespace` - Namespace where the `SecretMap` resource with given `name` resides.
///
/// Note: Does not check for resource's existence for simplicity.
pub async fn delete(client: Client, name: &str, namespace: &str) -> Result<SecretMap, Error> {
    let api: Api<SecretMap> = Api::namespaced(client, namespace);
    let finalizer: Value = json!({
        "metadata": {
            "finalizers": null
        }
    });

    let patch: Patch<&Value> = Patch::Merge(&finalizer);
    api.patch(name, &PatchParams::default(), &patch).await
}
