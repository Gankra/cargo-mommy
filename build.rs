//! Code that statically parses responses.kdl and adds it to the codebase~
//!
//! This allows the binary to only include what it needs, both perf-wise
//! and oh-god-i-found-these-strings-in-this-binary-wise.
//!
//! How to add a new mood, simply add a new entry to the `moods` object in
//! `responses.kdl`. Make sure to add `vars` entries for any new variables you
//! introduce.
//!
//! If your new mood or variable include... "spicy" terms, make sure to set an
//! explicit `spiciness`.

use std::collections::BTreeMap;
use std::env;
use std::fmt::Write;
use std::fs;
use std::ops::Range;
use std::path::Path;

use knuffel::ast::SpannedNode;
use knuffel::decode::Context;
use knuffel::errors::DecodeError;
use knuffel::traits::ErrorSpan;
use knuffel::Decode;
use regex::Regex;

const RESPONSES: &str = include_str!("./responses.kdl");

#[derive(PartialEq, Eq, PartialOrd, Ord, knuffel::DecodeScalar, Copy, Clone)]
// TODO: #[serde(rename_all = "snake_case")]
enum Spiciness {
    Chill,
    Thirsty,
    Yikes,
}

impl Spiciness {
    const CONFIGURED: Spiciness = if cfg!(feature = "yikes") {
        Self::Yikes
    } else if cfg!(feature = "thirsty") {
        Self::Thirsty
    } else {
        Self::Chill
    };
}

impl Default for Spiciness {
    fn default() -> Self {
        Self::Chill
    }
}

#[derive(knuffel::Decode)]
struct Mood {
    #[knuffel(child, unwrap(arguments))]
    positive: Vec<String>,
    #[knuffel(child, unwrap(arguments))]
    negative: Vec<String>,
    #[knuffel(child, unwrap(arguments))]
    overflow: Vec<String>,

    #[knuffel(child, default, unwrap(argument))]
    spiciness: Spiciness,
}

#[derive(knuffel::Decode)]
struct Var {
    #[knuffel(child, unwrap(arguments))]
    defaults: Vec<String>,
    #[knuffel(child, unwrap(argument))]
    env_key: Option<String>,
    #[knuffel(child, default, unwrap(argument))]
    spiciness: Spiciness,

    // Mommy needs a way to reference variables by index when doing template
    // substitution. This type is the value of an ordered map, so we can just
    // stick an index in after parsing~
    index: usize,
}

#[derive(knuffel::Decode)]
struct Config {
    #[knuffel(child)]
    vars: KdlMap<Var>,
    #[knuffel(child)]
    moods: KdlMap<Mood>,
}

struct KdlMap<V>(BTreeMap<String, V>);
impl<V: Decode<S>, S: ErrorSpan> Decode<S> for KdlMap<V> {
    fn decode_node(node: &SpannedNode<S>, ctx: &mut Context<S>) -> Result<Self, DecodeError<S>> {
        let mut result = BTreeMap::<String, V>::new();
        for child in node.children() {
            let key = &child.node_name;
            let val = V::decode_node(child, ctx)?;
            result.insert(key.to_string(), val);
        }
        Ok(Self(result))
    }
}
impl<V> std::ops::Deref for KdlMap<V> {
    type Target = BTreeMap<String, V>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<V> std::ops::DerefMut for KdlMap<V> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

fn main() -> Result<(), miette::Report> {
    let out_dir = &env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(out_dir).join("responses.rs");

    let mut config: Config = knuffel::parse("responses.kdl", RESPONSES)?;
    let mut i = 0;
    let mut vars = String::new();
    for (name, var) in config.vars.iter_mut() {
        if var.spiciness > Spiciness::CONFIGURED {
            continue;
        }
        var.index = i;
        i += 1;

        let env_key = var
            .env_key
            .clone()
            .unwrap_or_else(|| format!("{}S", name.to_uppercase()));
        let defaults = &var.defaults;
        let _ = write!(
            vars,
            r#"Var {{ env_key: "{env_key}", defaults: &{defaults:?} }},"#
        );
    }

    let pattern = Regex::new(r"\{\w+\}").unwrap();
    let mut responses = String::new();
    for (name, mood) in &*config.moods {
        if mood.spiciness > Spiciness::CONFIGURED {
            continue;
        }

        let parse_response = |text: &str, out: &mut String| {
            let _ = write!(out, "&[");

            // Mommy has to the template on matches for `pattern`, and generate
            // an array of alternating Chunk::Text and Chunk::Var values.
            let mut prev = 0;
            for var in pattern.find_iter(text) {
                let var_name = &var.as_str()[1..var.len() - 1];
                let var_idx = match config.vars.get(var_name) {
                    Some(var) => {
                        assert!(
                            var.spiciness <= Spiciness::CONFIGURED,
                            "{{{var_name}}} is too spicy!"
                        );
                        var.index
                    }
                    None => panic!("undeclared variable {{{var_name}}}"),
                };

                let Range { start, end } = var.range();
                let prev_chunk = &text[prev..start];
                prev = end;

                let _ = write!(out, "Chunk::Text({prev_chunk:?}), Chunk::Var({var_idx}), ");
            }

            let _ = write!(out, "Chunk::Text({:?})],", &text[prev..],);
        };

        let _ = write!(responses, "Mood {{ name: {name:?}, positive: &[");
        for response in &mood.positive {
            parse_response(response, &mut responses)
        }
        let _ = write!(responses, "], negative: &[");
        for response in &mood.negative {
            parse_response(response, &mut responses)
        }
        let _ = write!(responses, "], overflow: &[");
        for response in &mood.overflow {
            parse_response(response, &mut responses)
        }
        let _ = write!(responses, "] }},");
    }

    // Mommy needs some hard-coded vars at a specific location~
    let mood_idx = config.vars["mood"].index;
    let emote_idx = config.vars["emote"].index;
    let pronoun_idx = config.vars["pronoun"].index;
    let role_idx = config.vars["role"].index;

    fs::write(
        dest_path,
        format!(
            r"
            static CONFIG: Config<'static> = Config {{
                vars: &[{vars}],
                moods: &[{responses}],
            }};
            static MOOD: &Var<'static> = &CONFIG.vars[{mood_idx}];
            static EMOTE: &Var<'static> = &CONFIG.vars[{emote_idx}];
            static PRONOUN: &Var<'static> = &CONFIG.vars[{pronoun_idx}];
            static ROLE: &Var<'static> = &CONFIG.vars[{role_idx}];
            "
        ),
    )
    .unwrap();

    println!("cargo:rerun-if-changed=responses.kdl");

    Ok(())
}
