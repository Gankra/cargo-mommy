#![allow(clippy::let_and_return)]

use fastrand::Rng;

enum ResponseType {
    Positive,
    Negative,
}

fn main() {
    // Ideally mommy would use ExitCode but that's pretty new and mommy wants
    // to support more little ones~
    let code = real_main().unwrap_or_else(|e| {
        eprintln!("Error: {e:?}");
        -1
    });
    std::process::exit(code)
}

fn real_main() -> Result<i32, Box<dyn std::error::Error>> {
    let cargo = std::env::var("CARGO").unwrap_or_else(|_| "cargo".to_owned());
    let mut arg_iter = std::env::args().peekable();
    let _cargo = arg_iter.next();
    if arg_iter.peek().map_or(false, |arg| arg == "mommy") {
        let _mommy = arg_iter.next();
    }

    let mut cmd = std::process::Command::new(cargo);
    cmd.args(arg_iter);
    let status = cmd.status()?;
    let code = status.code().unwrap_or(-1);
    if is_quiet_mode_enabled(cmd.get_args()) {
        return Ok(code);
    }

    let response = if status.success() {
        select_response(ResponseType::Positive)
    } else {
        select_response(ResponseType::Negative)
    };

    match response {
        Ok(resp) => eprintln!("\x1b[1m{resp}\x1b[0m"),
        Err(resp) => eprintln!("\x1b[31m{resp}\x1b[0m"),
    }

    Ok(code)
}

fn is_quiet_mode_enabled(args: std::process::CommandArgs) -> bool {
    for arg in args.filter_map(std::ffi::OsStr::to_str) {
        match arg.as_bytes() {
            b"--" => break,
            b"--quiet" => return true,
            [b'-', b'-', ..] => {}
            [b'-', args @ ..] if args.contains(&b'q') => return true,
            _ => {}
        }
    }

    false
}

fn select_response(response_type: ResponseType) -> Result<String, String> {
    let rng = Rng::new();

    // Get mommy's options~

    // Choose what mood mommy is in~
    let mood = MOOD.load(&rng)?;

    let Some(group) = &CONFIG
        .moods
        .iter()
        .find(|group| group.name == mood)
    else {
        let supported_moods_str = CONFIG
            .moods
            .iter()
            .map(|group| group.name)
            .collect::<Vec<_>>()
            .join(", ");
        return Err(format!(
            "{role} doesn't know how to feel {mood}... {pronoun} moods are {supported_moods_str}",
            role = ROLE.load(&rng)?,
            pronoun = PRONOUN.load(&rng)?,
        ));
    };

    // Choose what mommy will say~
    let responses = match response_type {
        ResponseType::Positive => group.positive,
        ResponseType::Negative => group.negative,
    };
    let response = &responses[rng.usize(..responses.len())];

    // Apply options to the message template~
    let mut response = CONFIG.apply_template(response, &rng)?;

    // Let mommy show a little emote~
    let should_emote = rng.bool();
    if should_emote {
        if let Ok(emote) = EMOTE.load(&rng) {
            response.push(' ');
            response.push_str(&emote);
        }
    }

    // Done~!
    Ok(response)
}

// Mommy generates CONFIG and other global constants in build.rs~
include!(concat!(env!("OUT_DIR"), "/responses.rs"));

struct Config<'a> {
    vars: &'a [Var<'a>],
    moods: &'a [Mood<'a>],
}

impl Config<'_> {
    /// Applies a template by resolving `Chunk::Var`s against `self.vars`.
    fn apply_template(&self, chunks: &[Chunk], rng: &Rng) -> Result<String, String> {
        let mut out = String::new();
        for chunk in chunks {
            match chunk {
                Chunk::Text(text) => out.push_str(text),
                Chunk::Var(i) => out.push_str(&self.vars[*i].load(rng)?),
            }
        }
        Ok(out)
    }
}

struct Mood<'a> {
    name: &'a str,
    // Each of mommy's response templates is an alternating sequence of
    // Text and Var chunks; Text is literal text that should be printed as-is;
    // Var is an index into mommy's CONFIG.vars table~
    positive: &'a [&'a [Chunk<'a>]],
    negative: &'a [&'a [Chunk<'a>]],
}

enum Chunk<'a> {
    Text(&'a str),
    Var(usize),
}

struct Var<'a> {
    env_key: &'a str,
    defaults: &'a [&'a str],
}

impl Var<'_> {
    /// Loads this variable and selects one of the possible values for it;
    /// produces an in-character error message on failure.
    fn load(&self, rng: &Rng) -> Result<String, String> {
        let var = std::env::var(self.env_key);
        let split;

        let choices = match var.as_deref() {
            Ok("") => &[],
            Ok(value) => {
                split = value.split('/').collect::<Vec<_>>();
                split.as_slice()
            }
            Err(_) => self.defaults,
        };

        if choices.is_empty() {
            // If self == ROLE, mommy needs to avoid infinite recursion and
            // blowing her stack~
            let role = match self.env_key {
                "CARGO_MOMMYS_ROLES" => "mommy(?)".to_string(),
                _ => ROLE.load(rng)?,
            };

            return Err(format!(
                "{role} needs at least one value for {}~",
                self.env_key
            ));
        }

        Ok(choices[rng.usize(..choices.len())].to_owned())
    }
}

#[cfg(test)]
#[test]
fn test() {
    // Uncomment if you want a failing test
    // panic!("oops!!");
}
