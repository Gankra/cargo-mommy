use std::env;
use std::fs;
use std::path::Path;

const RESPONSES: &str = include_str!("./responses.json");

#[derive(serde::Deserialize)]
struct AllResponses {
    chill: Responses,
    thirsty: Responses,
    yikes: Responses,
}
#[derive(serde::Deserialize)]
struct Responses {
    positive: Vec<String>,
    negative: Vec<String>,
}

fn serialise_responses(name: &str, response: Responses) -> String {
    let response_positive = response.positive;
    let response_negative = response.negative;

    format!(
        r#"("{name}", &[
        &{response_positive:?},
        &{response_negative:?},
    ]),"#
    )
}

fn main() {
    let out_dir = &env::var("OUT_DIR").unwrap();
    let AllResponses {
        chill,
        thirsty,
        yikes,
    } = serde_json::from_str(RESPONSES).unwrap();
    let dest_path = Path::new(out_dir).join("responses.rs");

    let mut enabled_responses = String::new();

    enabled_responses += &serialise_responses("chill", chill);

    if cfg!(feature = "thirsty") {
        enabled_responses += &serialise_responses("thirsty", thirsty);
    }
    if cfg!(feature = "yikes") {
        enabled_responses += &serialise_responses("yikes", yikes);
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
