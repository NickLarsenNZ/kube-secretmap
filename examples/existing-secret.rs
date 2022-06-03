use std::collections::BTreeMap;

use k8s_openapi::{api::core::v1::Secret};
use kube::api::{PatchParams, Patch};

#[tokio::main]
async fn main() {
    // This will just use the default context in ~/.kube/config or whatever is set in env vars.
    let client = kube::Client::try_default().await.expect("can't make a k8s client");

    // Get the SecretName from the SecretMap
    // This would be read from the reconcile param `resource: Arc<SecretMap>`, eg: `resource.spec.secret_name`
    // Note: I have purposefully made it Some().expect to mimic how it would really work.
    let secret_name = Some(String::from("existing-example")).expect("actually use ok_or(Error::Something(\"blah\"))");

    // This would be read from the reconcile param `resource: Arc<SecretMap>`
    let matchLabels = ();

    // Secrets will be created in the same namespace as the SecretMap
    let namespace_of_secretmap = "default";

    // Lookup the secrets in the same namespace as the SecretMap
    let secrets: kube::Api<Secret> = kube::Api::namespaced(client.clone(), namespace_of_secretmap);


    // Find the secret
    // Otherwise log event and reque (or drop I guess, since it is a fatal issue like permissions)
    let s = secrets
        .get_opt(secret_name.as_str())
        .await
        .expect("actually use ok_or(Error::Something(\"blah\"))");
        // .ok_or()
        // .expect("use a questionmark to bubble up the Err case")

    // If it doesn't exist, we shoud log an event and requeue (for a richer status message). Maybe update the Status object for the secret
    let thing = match s {
        Some(s) => {Ok(s)},
        None => {
            // Log event
            //Requeue
            Err(())
        }
    }.expect("todo: bubble up with ?");

    // Does it exist yet?
    let s = secrets.list(&kube::api::ListParams {
        label_selector: todo!(),
        field_selector: todo!(),
        timeout: todo!(),
        bookmarks: todo!(),
        limit: todo!(),
        continue_token: todo!(),
    }).await.expect("actually use ok_or(Error::Something(\"blah\"))");
    // // Get the Owner Reference
    // // This would be read from the reconcile param `resource: Arc<SecretMap>`, eg: `resource.controller_owner_ref(&()).unwrap();`
    // oref = OwnerReference {
    //     api_version: todo!(),
    //     block_owner_deletion: todo!(),
    //     controller: todo!(),
    //     kind: todo!(),
    //     name: todo!(),
    //     uid: todo!()
    // }

    // The completely new secret
    let new_secret = Secret {
        metadata: kube::core::ObjectMeta {
            name: Some(secret_name.clone()),
            // owner_references: Some(vec![oref]),
            ..kube::core::ObjectMeta::default()
        },
        // Probably need this for binary blobs
        // data: Some(BTreeMap::from([String, ByteString])),
        string_data: Some(BTreeMap::from([
            (String::from("MY_SECRET"), String::from("my secret string"))
        ])),
        ..Default::default()
    };

    // Setup the API Client for doing operations on Secret resources
    let secrets = kube::Api::<Secret>::namespaced(client, namespace_of_secretmap);

    // Upsert the Secret
    secrets.patch(
        new_secret
            .metadata.name
            .as_ref()
            .expect("actually use ok_or(Error::Something(\"blah\"))"),
        &PatchParams::apply("our-controller-name"),
        &Patch::Apply(&new_secret)
    ).await.expect("actually use map_err(Error::SomethingElse(\"blah\"");
}
