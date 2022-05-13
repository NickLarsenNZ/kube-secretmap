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

    for crd in crd_list {
        let dest_path = Path::new(&OUT_DIR).join(format!("{}.crd.yml", crd.spec.names.kind));

        fs::write(
            &dest_path,
            serde_yaml::to_string(&crd).unwrap()
        ).unwrap();
    }

}
