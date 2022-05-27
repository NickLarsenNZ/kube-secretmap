use std::collections::BTreeMap;

use crate::MappingConfig;
use crate::SecretAlreadyExists;
use crate::SecretMap;
use crate::SecretMapSpec;

impl SecretMap {
    pub fn examples() -> Vec<Self> {
        vec![
            Self::new("static-example", SecretMapSpec {
                secret_name: String::from("example"),
                labels: Some(BTreeMap::from([
                    (String::from("kube-secrets-map.example.com/overwrite"), String::from("true")),
                ])),
                secret_already_exists: None,
                mappings: BTreeMap::from([
                    (String::from("DB_USERNAME"), MappingConfig::Static(String::from("postgres"))),
                ]),
            }),
            Self::new("aws-example-overwrite", SecretMapSpec {
                secret_name: "example".to_owned(),
                labels: None,
                secret_already_exists: Some(SecretAlreadyExists{
                    match_labels: BTreeMap::from([
                        (String::from("kube-secrets-map.example.com/overwrite"), String::from("true")),
                    ]),
                }),
                mappings: BTreeMap::from([
                    (String::from("DB_PASSWORD"), MappingConfig::Static(String::from("blah"))),
                ]),
            }),
        ]
    }
}
