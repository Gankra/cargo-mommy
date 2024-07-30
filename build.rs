//! Code that statically parses responses.json and adds it to the codebase~
//!
//! This allows the binary to only include what it needs, both perf-wise
//! and oh-god-i-found-these-strings-in-this-binary-wise.
//!
//! How to add a new mood, simply add a new entry to the `moods` object in
//! `responses.json`. Make sure to add `vars` entries for any new variables you
//! introduce.
//!
//! If your new mood or variable include... "spicy" terms, make sure to set an
//! explicit `spiciness`.

#![allow(dead_code)]

use std::env;
use std::fs;
use std::path::Path;

// Mommy needs to use some code that's part of the final binary in her build
// script, too~
#[path = "src/json.rs"]
mod json;
#[path = "src/template.rs"]
mod template;

const RESPONSES: &str = include_str!("./responses.json");

// This needs to exist so Var::load() compiles, but don't worry, mommy will never
// call that from build.rs~
const CONFIG: template::Config<'static> = template::Config {
    vars: &[],
    moods: &[],
    mood: !0,
    emote: !0,
    pronoun: !0,
    role: !0,
};

fn main() {
    let out_dir = &env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(out_dir).join("responses.rs");

    let bump = bumpalo::Bump::new();
    let config = json::Config::parse("mommy", RESPONSES)
        .unwrap()
        .build("mommy", &bump)
        .unwrap();

    fs::write(
        dest_path,
        format!(
            "const CONFIG: template::Config<'static> = {};",
            config.const_string()
        ),
    )
    .unwrap();

    println!("cargo:rerun-if-changed=responses.json");
}
