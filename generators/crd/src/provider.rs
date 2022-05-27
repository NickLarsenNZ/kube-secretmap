pub mod aws {
    use schemars::JsonSchema;
    use serde::{Serialize, Deserialize};

    #[derive(Serialize, Deserialize, Debug, PartialEq, Clone, JsonSchema)]
    #[serde(rename_all = "camelCase")]
    pub struct AwsProviderConfig {
        /// AWS offers both SSM Parameter Store and Secrets Manager
        pub store: AwsSecretStore,

        /// Name of the secret (eg: /app/hello-world/db/password)
        pub name: String,

        /// Override the AWS Region (eg: us-west-1)
        #[serde(skip_serializing_if = "Option::is_none")]
        pub region: Option<String>
    }

    #[derive(Serialize, Deserialize, Debug, PartialEq, Clone, JsonSchema)]
    #[serde(rename_all = "camelCase")]
    pub enum AwsSecretStore {
        SsmParameterStore,
        SecretsManager,
    }

    impl Default for AwsSecretStore {
        fn default() -> Self {
            Self::SsmParameterStore
        }
    }
}
