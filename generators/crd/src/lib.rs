use kube::{CustomResource};
use schemars::JsonSchema;
use serde::{Serialize, Deserialize};

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

    secret_already_exists: Option<SecretAlreadyExists>
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
    match_labels: Labels

}
