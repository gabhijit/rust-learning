use openapiv3::OpenAPI;

fn main() {
    let data = include_str!("TS29571_CommonData.yaml.yaml");

    let spec: OpenAPI = serde_yaml::from_str(data).expect("failed to parse spec.");

    eprintln!("spec: {:#?}", spec);
}
