use std::env;
use std::fs;
use std::path::Path;

const RESPONSES: &str = include_str!("./responses.json");

#[derive(serde::Deserialize)]
struct Responses {
    positive: Vec<String>,
    negative: Vec<String>,
}

fn main() {
    let out_dir = &env::var("OUT_DIR").unwrap();
    let responses: Responses = serde_json::from_str(RESPONSES).unwrap();
    let dest_path = Path::new(out_dir).join("responses.rs");

    fs::write(
        dest_path,
        format!(
            "const POSITIVE_RESPONSES: &[&str] = &{:?};\nconst NEGATIVE_RESPONSES: &[&str] = &{:?};",
            responses.positive, responses.negative
        ),
    )
    .unwrap();

    println!("cargo:rerun-if-changed=responses.json");
}
