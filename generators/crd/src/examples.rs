use std::collections::BTreeMap;

use crate::MappingConfig;
use crate::SecretAlreadyExists;
use crate::SecretMap;
use crate::SecretMapSpec;
use crate::provider::aws::AwsProviderConfig;
use crate::provider::aws::AwsSecretStore;

impl SecretMap {
    pub fn examples() -> Vec<Self> {
        vec![
            Self::new("static-example", SecretMapSpec {
                secret_name: String::from("static-example"),
                labels: Some(BTreeMap::from([
                    (String::from("app.kubernetes.io/component"), String::from("database")),
                ])),
                secret_already_exists: None,
                mappings: BTreeMap::from([
                    (String::from("DB_USERNAME"), MappingConfig::Static(String::from("postgres"))),
                ]),
            }),
            Self::new("static-example-overwrite", SecretMapSpec {
                secret_name: String::from("existing"),
                labels: None,
                secret_already_exists: Some(SecretAlreadyExists{
                    match_labels: BTreeMap::from([
                        (String::from("kube-secrets-map.example.com/overwrite"), String::from("true")),
                    ]),
                }),
                mappings: BTreeMap::from([
                    (String::from("DB_USERNAME"), MappingConfig::Static(String::from("postgres"))),
                ]),
            }),
            Self::new("aws-example", SecretMapSpec {
                secret_name: "aws-example".to_owned(),
                labels: Some(BTreeMap::from([
                    (String::from("app.kubernetes.io/component"), String::from("database")),
                ])),
                secret_already_exists: None,
                mappings: BTreeMap::from([
                    (String::from("DB_PASSWORD"), MappingConfig::Aws(AwsProviderConfig {
                        region:Some(String::from("ap-southeast-2")),
                        store: AwsSecretStore::default(),
                        name: String::from("/app/hello-world/db/password"),
                    })),
                ]),
            }),
        ]
    }
}
