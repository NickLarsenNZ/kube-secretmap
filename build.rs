use std::fs;
use std::path::Path;

use kube::CustomResourceExt;

// Although build scripts aren't supposed to write files outside of the OUT_DIR environment variable
// we want to store the generated manifests in a more convenient location.
const OUT_DIR: &str = "manifests";

fn main() {

    // Add any new CRDs to this list to generate the CRD manifests
    let crd_list = vec![
        crd::SecretMap::crd(),
    ];

    // Add any examples to write
    let example_list: Vec<_> = vec![
        crd::SecretMap::examples()
    ].into_iter().flatten().collect();

    for crd in crd_list {
        let dest_path = Path::new(&OUT_DIR).join(format!("{}.crd.yml", crd.spec.names.kind));

        fs::write(
            &dest_path,
            serde_yaml::to_string(&crd).unwrap()
        ).unwrap();
    }

    for example in example_list {
        let dest_path = Path::new(&OUT_DIR).join(format!("examples/{}.yml", example.clone().metadata.name.expect("named example resource")));

        fs::write(
            &dest_path,
            serde_yaml::to_string(&example).unwrap()
        ).unwrap();
    }

}
