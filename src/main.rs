use std::error::Error;
use std::{thread};
use std::time::Duration;

use k8s_openapi::api::core::v1::Node;
use kube::{Client, Api, api::ListParams};


#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {

    let kubernetes_client = Client::try_default().await.expect("can't make a k8s client");
    let nodes: Api<Node> = Api::all(kubernetes_client);

    match nodes.list(&ListParams::default().limit(1)).await {
        Ok(nodes) => {
            if let Some(node)= nodes.items.get(0) {
                if let Some(name) = node.metadata.name.clone() {
                    println!("node: {}", name);
                }
            }
        },
        Err(e) => println!("{}", e)
    }

    println!("Sleeping...");
    loop {
        thread::sleep(Duration::from_secs(15));
    }

}
