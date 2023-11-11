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

fn main() {
    let out_dir = &env::var("OUT_DIR").unwrap();
    let AllResponses {
        chill:
            Responses {
                positive: chill_positive,
                negative: chill_negative,
            },
        thirsty:
            Responses {
                positive: thirsty_positive,
                negative: thirsty_negative,
            },
        yikes:
            Responses {
                positive: yikes_positive,
                negative: yikes_negative,
            },
    } = serde_json::from_str(RESPONSES).unwrap();
    let dest_path = Path::new(out_dir).join("responses.rs");

    fs::write(
        dest_path,
        format!(
            r#"const RESPONSES: &[(&str, &[&[&str]])] = &[
    ("chill", &[
        &{chill_positive:?},
        &{chill_negative:?},
    ]),
    ("thirsty", &[
        &{thirsty_positive:?},
        &{thirsty_negative:?},
    ]),
    ("yikes", &[
        &{yikes_positive:?},
        &{yikes_negative:?},
    ]),
];"#
        ),
    )
    .unwrap();

    println!("cargo:rerun-if-changed=responses.json");
}
