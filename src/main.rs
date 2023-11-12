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

    // Understand who mommy really is~
    //
    // *INHALES*
    //
    // Ok so first argument is this binary, should look like /some/path/to/cargo-mommy(.exe)
    // but we want to let it instead be cargo-daddy, and for everything to rekey itself to:
    // * make the default role be "daddy"
    // * make all the read env-vars be "CARGO_DADDYS_*"
    let first_arg = arg_iter.next().unwrap_or_default();
    let first_arg = first_arg
        .strip_suffix(".exe")
        .unwrap_or(&first_arg)
        .to_owned();
    let true_role = if let Some((_path, role)) = first_arg.rsplit_once("cargo-") {
        role.to_owned()
    } else {
        // If something messed up is going on, default to "mommy"
        "mommy".to_owned()
    };

    // *GASPS FOR BREATH*
    //
    // *INHALES DESPERATELY*
    //
    // cargo subcommands when run as "cargo blah" get the "blah" argument passed to themselves
    // as the *second* argument. However if we are invoked directly as "cargo-blah" we won't
    // have that extra argument! So if there's a second argument that is "mommy" (or whatever)
    // we pop off that redundant copy before forwarding the rest of the args back to "cargo ...".
    // if we don't do this, we'll infinitely recurse into ourselves by re-calling "cargo mommy"!
    // (note that it *is* supported to do `cargo mommy mommy` and get two messages, although I
    // belive we do this, `cargo-mommy mommy` will still only get you onw message).
    //
    // ...
    //
    // ~
    if arg_iter.peek().map_or(false, |arg| arg == &true_role) {
        let _role = arg_iter.next();
    }

    // Time for mommy to call cargo~
    let mut cmd = std::process::Command::new(cargo);
    cmd.args(arg_iter);
    let status = cmd.status()?;
    let code = status.code().unwrap_or(-1);
    if is_quiet_mode_enabled(cmd.get_args()) {
        return Ok(code);
    }

    // Time for mommy to tell you how you did~
    let response = if status.success() {
        select_response(&true_role, ResponseType::Positive)
    } else {
        select_response(&true_role, ResponseType::Negative)
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

fn select_response(true_role: &str, response_type: ResponseType) -> Result<String, String> {
    let rng = Rng::new();

    // Get mommy's options~

    // Choose what mood mommy is in~
    let mood = MOOD.load(true_role, &rng)?;

    let Some(group) = &CONFIG.moods.iter().find(|group| group.name == mood) else {
        let supported_moods_str = CONFIG
            .moods
            .iter()
            .map(|group| group.name)
            .collect::<Vec<_>>()
            .join(", ");
        return Err(format!(
            "{role} doesn't know how to feel {mood}... {pronoun} moods are {supported_moods_str}",
            role = ROLE.load(true_role, &rng)?,
            pronoun = PRONOUN.load(true_role, &rng)?,
        ));
    };

    // Choose what mommy will say~
    let responses = match response_type {
        ResponseType::Positive => group.positive,
        ResponseType::Negative => group.negative,
    };
    let response = &responses[rng.usize(..responses.len())];

    // Apply options to the message template~
    let mut response = CONFIG.apply_template(true_role, response, &rng)?;

    // Let mommy show a little emote~
    let should_emote = rng.bool();
    if should_emote {
        if let Ok(emote) = EMOTE.load(true_role, &rng) {
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
    fn apply_template(
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
    fn load(&self, true_role: &str, rng: &Rng) -> Result<String, String> {
        // try to load custom settings from env vars~
        let var = std::env::var(self.env(true_role));
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
            let role = ROLE.load(true_role, rng)?;
            return Err(format!(
                "{role} needs at least one value for {}~",
                self.env_key
            ));
        }

        // now select a choice from the options~
        Ok(choices[rng.usize(..choices.len())].to_owned())
    }

    /// Gets the name of the env var to load~
    fn env(&self, true_role: &str) -> String {
        // Normally we'd load from CARGO_MOMMYS_*
        // but if cargo-mommy is cargo-daddy, we should load CARGO_DADDYS_* instead~
        let screaming_role = true_role.to_ascii_uppercase();
        let var = format!("CARGO_{screaming_role}S_{}", self.env_key);
        eprintln!("using: {}", var);
        var
    }
}

#[cfg(test)]
#[test]
fn test() {
    // Uncomment if you want a failing test
    // panic!("oops!!");
}
