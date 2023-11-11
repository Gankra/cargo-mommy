//! Code that statically parses responses.json and adds it to the codebase~
//!
//! This allows the binary to only include what it needs, both perf-wise
//! and oh-god-i-found-these-strings-in-this-binary-wise.
//!
//! How to add a new mood (look for all the places "thirsty" shows up, duplicate it):
//!
//! * responses.json: add the data
//! * Cargo.toml: add a default-on cfg for it
//! * build.rs: Add a serialize_responses call (behind that cfg)
//! * main.rs: If your mood needs access to yikes-exclusive substitutions like "DENIGRATING_TERM"
//!   then update the `cfg`'s in main.rs to enable them when your feature is there too

use std::env;
use std::fs;
use std::path::Path;

const RESPONSES: &str = include_str!("./responses.json");
type AllResponses = std::collections::BTreeMap<String, Responses>;
#[derive(serde::Deserialize)]
struct Responses {
    positive: Vec<String>,
    negative: Vec<String>,
}

fn serialise_responses(name: &str, responses: &AllResponses) -> String {
    let response = &responses[name];
    let response_positive = &response.positive;
    let response_negative = &response.negative;

    format!(
        r#"("{name}", &[
        &{response_positive:?},
        &{response_negative:?},
    ]),"#
    )
}

fn main() {
    let out_dir = &env::var("OUT_DIR").unwrap();
    let responses: AllResponses = serde_json::from_str(RESPONSES).unwrap();
    let dest_path = Path::new(out_dir).join("responses.rs");

    let mut enabled_responses = String::new();

    enabled_responses += &serialise_responses("chill", &responses);

    if cfg!(feature = "thirsty") {
        enabled_responses += &serialise_responses("thirsty", &responses);
    }
    if cfg!(feature = "yikes") {
        enabled_responses += &serialise_responses("yikes", &responses);
    }

    fs::write(
        dest_path,
        format!(
            r#"const RESPONSES: &[(&str, &[&[&str]])] = &[
                {enabled_responses}
            ];"#
        ),
    )
    .unwrap();

    println!("cargo:rerun-if-changed=responses.json");
}
