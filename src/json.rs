use std::collections::BTreeMap;
use std::ops::Range;

use bumpalo::Bump;
use regex::Regex;

#[derive(serde::Deserialize)]
pub struct Config {
    moods: BTreeMap<String, Mood>,
    vars: BTreeMap<String, Var>,
}

impl Config {
    /// Parses a JSON config; see `responses.json`.
    pub fn parse(data: &str) -> Self {
        let mut config: Self = serde_json::from_str(data).unwrap();
        for (i, (_, var)) in config.vars.iter_mut().enumerate() {
            var.index = i;
        }
        config
    }

    fn var_index(&self, name: &str) -> usize {
        self.vars[name].index
    }

    /// Builds a "real" config. This mostly consists of converting formatting
    /// strings into something simpler to interpolate strings into.
    pub fn build<'a>(&self, arena: &'a Bump) -> crate::template::Config<'a> {
        use crate::template::*;

        let mut vars = Vec::new();
        for (name, var) in &self.vars {
            if var.spiciness > Spiciness::CONFIGURED {
                continue;
            }

            let env_key = var
                .env_key
                .clone()
                .unwrap_or_else(|| format!("{}S", name.to_uppercase()));
            let defaults = var
                .defaults
                .iter()
                .map(|s| &*arena.alloc_str(s))
                .collect::<Vec<_>>();

            vars.push(Var {
                env_key: arena.alloc_str(&env_key),
                defaults: arena.alloc_slice_copy(&defaults),
            });
        }

        let pattern = Regex::new(r"\{\w+\}").unwrap();
        let mut moods = Vec::new();
        for (name, mood) in &self.moods {
            if mood.spiciness > Spiciness::CONFIGURED {
                continue;
            }

            let parse_response = |text: &str| -> &'a [Chunk<'a>] {
                let mut out = Vec::new();

                // Mommy has to the template on matches for `pattern`, and generate
                // an array of alternating Chunk::Text and Chunk::Var values.
                let mut prev = 0;
                for var in pattern.find_iter(text) {
                    let var_name = &var.as_str()[1..var.len() - 1];
                    let var_idx = match self.vars.get(var_name) {
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

                    out.push(Chunk::Text(arena.alloc_str(prev_chunk)));
                    out.push(Chunk::Var(var_idx));
                }

                out.push(Chunk::Text(arena.alloc_str(&text[prev..])));
                arena.alloc_slice_copy(&out)
            };

            moods.push(Mood {
                name: arena.alloc_str(name),
                positive: arena.alloc_slice_copy(
                    &mood
                        .positive
                        .iter()
                        .map(|s| parse_response(s))
                        .collect::<Vec<_>>(),
                ),
                negative: arena.alloc_slice_copy(
                    &mood
                        .negative
                        .iter()
                        .map(|s| parse_response(s))
                        .collect::<Vec<_>>(),
                ),
                overflow: arena.alloc_slice_copy(
                    &mood
                        .overflow
                        .iter()
                        .map(|s| parse_response(s))
                        .collect::<Vec<_>>(),
                ),
            });
        }

        Config {
            vars: arena.alloc_slice_copy(&vars),
            moods: arena.alloc_slice_copy(&moods),

            // Mommy needs some hard-coded vars at a specific location~
            mood: self.var_index("mood"),
            emote: self.var_index("emote"),
            pronoun: self.var_index("pronoun"),
            role: self.var_index("role"),
        }
    }
}

#[derive(serde::Deserialize)]
struct Mood {
    positive: Vec<String>,
    negative: Vec<String>,
    overflow: Vec<String>,

    #[serde(default)]
    spiciness: Spiciness,
}

#[derive(serde::Deserialize)]
struct Var {
    defaults: Vec<String>,
    #[serde(default)]
    env_key: Option<String>,

    #[serde(default)]
    spiciness: Spiciness,

    // Mommy needs a way to reference variables by index when doing template
    // substitution. This type is the value of an ordered map, so we can just
    // stick an index in after parsing~
    #[serde(skip)]
    index: usize,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
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
