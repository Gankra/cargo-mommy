use std::env;
use std::fmt;
use std::fmt::Write;

use fastrand::Rng;

#[derive(Copy, Clone)]
pub struct Config<'a> {
    pub vars: &'a [Var<'a>],
    pub moods: &'a [Mood<'a>],

    pub mood: usize,
    pub emote: usize,
    pub pronoun: usize,
    pub role: usize,
}

impl Config<'_> {
    /// Returns the special MOODS variable.
    pub fn mood(&self) -> &Var {
        &self.vars[self.mood]
    }

    /// Returns the special EMOTES variable.
    pub fn emote(&self) -> &Var {
        &self.vars[self.emote]
    }

    /// Returns the special PRONOUNS variable.
    pub fn pronoun(&self) -> &Var {
        &self.vars[self.pronoun]
    }

    /// Returns the special ROLES variable.
    pub fn role(&self) -> &Var {
        &self.vars[self.role]
    }

    /// Returns a valid Rust expression in a string representing this
    /// config's data.
    #[allow(unused)]
    pub fn const_string(&self) -> String {
        format!("{:#?}", DebugByToConst(self))
    }

    /// Applies a template by resolving `Chunk::Var`s against `self.vars`.
    pub fn apply_template(
        &self,
        true_role: &str,
        chunks: &[Chunk],
        rng: &Rng,
    ) -> Result<String, String> {
        let mut out = String::new();
        for chunk in chunks {
            match chunk {
                Chunk::Text(text) => out.push_str(text),
                Chunk::Var(i) => out.push_str(&self.vars[*i].load(true_role, rng)?),
            }
        }
        Ok(out)
    }
}

impl ToConst for Config<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("crate::template::Config")
            .field("vars", &DebugByToConst(&self.vars))
            .field("moods", &DebugByToConst(&self.moods))
            .field("mood", &self.mood)
            .field("emote", &self.emote)
            .field("pronoun", &self.pronoun)
            .field("role", &self.role)
            .finish()
    }
}

#[derive(Copy, Clone)]
pub struct Mood<'a> {
    pub name: &'a str,
    // Each of mommy's response templates is an alternating sequence of
    // Text and Var chunks; Text is literal text that should be printed as-is;
    // Var is an index into mommy's CONFIG.vars table~
    pub positive: &'a [&'a [Chunk<'a>]],
    pub negative: &'a [&'a [Chunk<'a>]],
    pub overflow: &'a [&'a [Chunk<'a>]],
}

impl ToConst for Mood<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("crate::template::Mood")
            .field("name", &self.name)
            .field("positive", &DebugByToConst(&self.positive))
            .field("negative", &DebugByToConst(&self.negative))
            .field("overflow", &DebugByToConst(&self.overflow))
            .finish()
    }
}

#[derive(Copy, Clone)]
pub enum Chunk<'a> {
    Text(&'a str),
    Var(usize),
}

impl ToConst for Chunk<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Text(x) => write!(f, "crate::template::Chunk::Text({x:?})"),
            Self::Var(x) => write!(f, "crate::template::Chunk::Var({x:?})"),
        }
    }
}

#[derive(Copy, Clone)]
pub struct Var<'a> {
    pub env_key: &'a str,
    pub defaults: &'a [&'a str],
}

impl ToConst for Var<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("crate::template::Var")
            .field("env_key", &self.env_key)
            .field("defaults", &DebugByToConst(&self.defaults))
            .finish()
    }
}

impl Var<'_> {
    /// Loads this variable and selects one of the possible values for it;
    /// produces an in-character error message on failure.
    pub fn load(&self, true_role: &str, rng: &Rng) -> Result<String, String> {
        // try to load custom settings from env vars~
        let var = env::var(self.env(true_role));
        let split;

        // parse the custom settings or use the builtins~
        let choices = match var.as_deref() {
            Ok("") => &[],
            Ok(value) => {
                split = value.split('/').collect::<Vec<_>>();
                split.as_slice()
            }
            Err(_) => self.defaults,
        };

        if choices.is_empty() {
            // If there's no ROLES set, default to mommy's true nature~
            if self.env_key == "ROLES" {
                return Ok(true_role.to_owned());
            }

            // Otherwise, report an error~
            let role = crate::CONFIG.role().load(true_role, rng)?;
            return Err(format!(
                "{role} needs at least one value for {}~",
                self.env_key
            ));
        }

        // now select a choice from the options~
        Ok(choices[rng.usize(..choices.len())].to_owned())
    }

    /// Gets the name of the env var to load~
    pub fn env(&self, true_role: &str) -> String {
        // Normally we'd load from CARGO_MOMMYS_*
        // but if cargo-mommy is cargo-daddy, we should load CARGO_DADDYS_* instead~
        // If we have multiple words in our role, we must also be careful with spaces~
        let screaming_role = true_role.to_ascii_uppercase().replace(' ', "_");
        format!("CARGO_{screaming_role}S_{}", self.env_key)
    }
}

/// This is some nonsense mommy needs to convert responses into something she
/// can bake into her binary for you~
trait ToConst {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result;
}

struct DebugByToConst<'a, T>(&'a T);
impl<T: ToConst> fmt::Debug for DebugByToConst<'_, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        ToConst::fmt(self.0, f)
    }
}

impl ToConst for usize {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

impl ToConst for &str {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

impl<T: ToConst> ToConst for &[T] {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_char('&')?;
        f.debug_list()
            .entries(self.iter().map(DebugByToConst))
            .finish()
    }
}
