#![allow(clippy::let_and_return)]

use fastrand::Rng;
use std::io::IsTerminal;

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
    let rng = Rng::new();
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

    // Interpret the argument as a path so we can manipulate it~
    let bin_path = std::path::PathBuf::from(arg_iter.next().unwrap_or_default());
    // Get the extensionless-file name, and parse if case-insensitively~
    let bin_name = bin_path
        .file_stem()
        .unwrap_or_default()
        .to_string_lossy()
        .to_lowercase();
    let true_role = if let Some((_path, role)) = bin_name.rsplit_once("cargo-") {
        role.to_owned()
    } else {
        // If something messed up is going on "mommy" will always take care of it~
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

    if arg_iter.peek().map_or(false, |arg| arg == &true_role) {
        let _ = arg_iter.next();
    }

    // *WHEEZES*
    //
    // *PANTS FOR A MINUTE*
    //
    // Ok so now we want to detect if the invocation looked like "cargo mommy i mean daddy"
    // if it *does* we want to copy ourselves to rename cargo-mommy to cargo-daddy. We can't
    // clone std::env::args() and we need to peek more than one element, so we create a
    // "speculation" buffer that we will chain back into the args on failed speculation.
    // on successful speculation we'll purge the buffer, effectively consuming the args.
    //
    // ...
    //
    // ~
    let mut speculated = vec![];
    {
        // We speculate the "i mean" part so that can easily discard it
        // in the case of "cargo mommy i mean mommy", making the execution
        // equivalent to "cargo mommy mommy". Not popping off the extra
        // "mommy" let "cargo mommy i mean mommy i mean mommy" work right~
        speculated.extend(arg_iter.by_ref().take(2));
        let new_role = arg_iter.peek();
        let mean = speculated.get(1) == Some(&"mean".to_owned());
        let i = speculated.get(0) == Some(&"i".to_owned());
        if i && mean {
            if let Some(new_role) = new_role.cloned() {
                // Ok at this point we're confident we got "i mean <new_role>"
                // so definitely consume the speculated arguments~
                speculated.clear();

                // If the new role is the same as before, they typed something like
                // "cargo mommy i mean mommy test" so we don't need to do anything~
                if new_role != true_role {
                    if let Some(parent) = bin_path.parent() {
                        let new_bin_name = format!("cargo-{new_role}");
                        let mut new_bin_path = parent.join(new_bin_name);
                        if let Some(ext) = bin_path.extension() {
                            new_bin_path.set_extension(ext);
                        }
                        if let Err(e) = std::fs::copy(bin_path, new_bin_path) {
                            return Err(format!(
                                "{role} couldn't copy {pronoun}self...\n{e:?}",
                                role = ROLE.load(&true_role, &rng)?,
                                pronoun = PRONOUN.load(&true_role, &rng)?,
                            ))?;
                        } else {
                            // Just exit immediately on success, don't try to get too clever here~
                            eprintln!("{true_role} is now {new_role}~");
                            return Ok(0);
                        }
                    } else {
                        return Err(format!(
                            "{role} couldn't copy {pronoun}self...\n(couldn't find own parent dir)",
                            role = ROLE.load(&true_role, &rng)?,
                            pronoun = PRONOUN.load(&true_role, &rng)?,
                        ))?;
                    }
                }
            }
        }
    }

    // Time for mommy to call cargo~
    let mut cmd = std::process::Command::new(cargo);
    cmd.args(speculated.into_iter().chain(arg_iter));
    let status = cmd.status()?;
    let code = status.code().unwrap_or(-1);
    if is_quiet_mode_enabled(cmd.get_args()) {
        return Ok(code);
    }

    // Time for mommy to tell you how you did~
    let response = if status.success() {
        select_response(&true_role, &rng, ResponseType::Positive)
    } else {
        select_response(&true_role, &rng, ResponseType::Negative)
    };

    let stylize = std::io::stderr().is_terminal();
    match (response, stylize) {
        (Ok(resp), true) => eprintln!("\x1b[1m{resp}\x1b[0m"),
        (Err(resp), true) => eprintln!("\x1b[31m{resp}\x1b[0m"),
        (Ok(resp) | Err(resp), false) => eprintln!("{resp}"),
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

fn select_response(
    true_role: &str,
    rng: &Rng,
    response_type: ResponseType,
) -> Result<String, String> {
    // Choose what mood mommy is in~
    let mood = MOOD.load(true_role, rng)?;

    let Some(group) = &CONFIG.moods.iter().find(|group| group.name == mood) else {
        let supported_moods_str = CONFIG
            .moods
            .iter()
            .map(|group| group.name)
            .collect::<Vec<_>>()
            .join(", ");
        return Err(format!(
            "{role} doesn't know how to feel {mood}... {pronoun} moods are {supported_moods_str}",
            role = ROLE.load(true_role, rng)?,
            pronoun = PRONOUN.load(true_role, rng)?,
        ));
    };

    // Choose what mommy will say~
    let responses = match response_type {
        ResponseType::Positive => group.positive,
        ResponseType::Negative => group.negative,
    };
    let response = &responses[rng.usize(..responses.len())];

    // Apply options to the message template~
    let mut response = CONFIG.apply_template(true_role, response, rng)?;

    // Let mommy show a little emote~
    let should_emote = rng.bool();
    if should_emote {
        if let Ok(emote) = EMOTE.load(true_role, rng) {
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
        // If we have multiple words in our role, we must also be careful with spaces~
        let screaming_role = true_role.to_ascii_uppercase().replace(' ', "_");
        format!("CARGO_{screaming_role}S_{}", self.env_key)
    }
}

#[cfg(test)]
#[test]
fn test() {
    // Uncomment if you want a failing test
    // panic!("oops!!");
}
