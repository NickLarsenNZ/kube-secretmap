use std::collections::BTreeMap;

use kube::{CustomResource};
use schemars::JsonSchema;
use serde::{Serialize, Deserialize};

pub mod provider;
pub mod examples;

#[derive(CustomResource, Serialize, Deserialize, Debug, PartialEq, Clone, JsonSchema)]
#[kube(
    group = "example.com",
    version = "v1aplha1",
    kind = "SecretMap",
    plural = "secretmaps",
    derive = "PartialEq",
    namespaced,
    status = "SecretMapStatus",

)]
#[serde(rename_all = "camelCase")]
pub struct SecretMapSpec {
    /// Name of Secret to create (or use if overwriteExisting.matchLabels are specified)
    #[validate(length(min = 3))]
    pub secret_name: String,

    // Labels to add to the secret created (not used if the secret already exists)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<Labels>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret_already_exists: Option<SecretAlreadyExists>,

    // Mappings of resulting secret key to seceret provider reference
    pub mappings: BTreeMap<String, MappingConfig>
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct SecretMapStatus {
    //stats: Stats
    // for each mapping, show last check, and last change, last error
}

type Labels = std::collections::BTreeMap<String, String>;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct SecretAlreadyExists {
    /// Labels used to match an existing `Secret`.
    /// Note: all labels listed here must be present on the existing `Secret`.
    match_labels: Labels
}

// todo: move this to the provider module
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub enum MappingConfig {
    /// Set a static string value. Useful for non-sensitive values that must be part of the same secret.
    Static(String),
    /// AWS SSM Parameter Store or Secrets Manager
    Aws(provider::aws::AwsProviderConfig),
    /// GCP Secret Manager
    Gcp(String),
}

// #[cfg(test)]
// mod tests {
//     use kube::CustomResourceExt;
//     use super::*;

//     #[test]
//     fn test() {
//         assert!(false);

//         let crd = SecretMap::new();
//     }

// }
