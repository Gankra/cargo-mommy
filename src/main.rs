#![allow(clippy::let_and_return)]

use fastrand::Rng;
use std::env;
use std::io::IsTerminal;
use std::path::PathBuf;

enum ResponseType {
    Positive,
    Negative,
    Overflow,
    FirstBeg,
    DidNotBeg,
}

/// Mommy intentionally lets her little ones call her recursively, since they might want to hear more from her~
///
/// If they call her a thousand times in a row, though, something has probably gone wrong 😏
const RECURSION_LIMIT: u8 = 100;
/// This name is intentionally not user-configurable. Mommy can't let the little ones make *too*
/// much of a mess~
const RECURSION_LIMIT_VAR: &str = "CARGO_MOMMY_RECURSION_LIMIT";

/// The lock file name
const LOCK_FILE_NAME: &str = "MOMMY.lock";

fn main() {
    // Ideally mommy would use ExitCode but that's pretty new and mommy wants
    // to support more little ones~
    let code = real_main().unwrap_or_else(|e| {
        eprintln!("Error: {e:?}");
        1
    });
    std::process::exit(code)
}

fn real_main() -> Result<i32, Box<dyn std::error::Error>> {
    let rng = Rng::new();

    let mut arg_iter = env::args().peekable();

    // Understand who mommy really is~
    //
    // *INHALES*
    //
    // Ok so first argument is this binary, should look like /some/path/to/cargo-mommy(.exe)
    // but we want to let it instead be cargo-daddy, and for everything to rekey itself to:
    // * make the default role be "daddy"
    // * make all the read env-vars be "CARGO_DADDYS_*"

    // Interpret the argument as a path so we can manipulate it~
    let first_arg = arg_iter.next();
    let bin_path = env::current_exe()
        .unwrap_or_else(|_| std::path::PathBuf::from(first_arg.unwrap_or_default()));
    // Get the extensionless-file name, and parse it case-insensitively~
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

    let cargo = env::var(format!("CARGO_{}S_ACTUAL", true_role.to_uppercase()))
        .or_else(|_| env::var("CARGO"))
        .unwrap_or_else(|_| "cargo".to_owned());

    // Check if someone has told mommy to keep calling herself~
    // Mommy loves you, darlings, but she can't keep running forever~
    let mut new_limit = 1;
    if let Ok(limit) = env::var(RECURSION_LIMIT_VAR) {
        if let Ok(n) = limit.parse::<u8>() {
            if n > RECURSION_LIMIT {
                let mut response = select_response(&true_role, &rng, ResponseType::Overflow);
                match &mut response {
                    Ok(s) | Err(s) => {
                        *s += "\nyou didn't set CARGO to something naughty, did you?\n"
                    }
                }
                pretty_print(response);
                return Ok(2);
            } else {
                new_limit = n + 1;
            }
        }
    }

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
    // believe we do this, `cargo-mommy mommy` will still only get you one message).

    if arg_iter.peek().map_or(false, |arg| arg == &true_role) {
        let _ = arg_iter.next();
    }

    // mommy will attempt to parse your input as a integer~
    // but if you provide nonsense you'll get punished~

    let beg_chance: u8 = BEG_CHANCE
        .load(&true_role, &rng)?
        .trim()
        .parse()
        .unwrap_or_else(|err: std::num::ParseIntError| match err.kind() {
            std::num::IntErrorKind::PosOverflow => 100,
            std::num::IntErrorKind::NegOverflow => 0,
            _ => 20,
        });

    // Sometimes mommy will decide to make you beg. So we have to check to make sure that if we
    // are to pop that argument off. But also note that it can be ok to beg if not required~
    //
    // Mommy also makes sure not to break anyone who wants to use a real tool called "cargo
    // please" if set to zero.

    let begging = if beg_chance != 0 {
        arg_iter
            .peek()
            .map_or(false, |arg| arg == "please")
            .then(|| arg_iter.next())
            .flatten()
            .is_some()
    } else {
        true
    };

    // *WHEEZES*
    //
    // *PANTS FOR A MINUTE*
    //
    // Ok so now we want to detect if the invocation looked like "cargo mommy i mean daddy"
    // if it *does* we want to copy ourselves to rename cargo-mommy to cargo-daddy. To make this
    // simpler, collect the args into a vec so we can peek more than one element.
    //
    // ...
    //
    // ~
    let mut args: Vec<_> = arg_iter.collect();
    {
        // We speculate the "i mean" part so that can easily discard it
        // in the case of "cargo mommy i mean mommy", making the execution
        // equivalent to "cargo mommy mommy". Not popping off the extra
        // "mommy" let "cargo mommy i mean mommy i mean mommy" work right~
        let new_role = args.get(2);
        let mean = args.get(1) == Some(&"mean".to_owned());
        let i = args.get(0) == Some(&"i".to_owned());
        if i && mean {
            if let Some(new_role) = new_role.cloned() {
                // Ok at this point we're confident we got "i mean <new_role>"
                // so definitely consume those arguments~
                args.drain(..2);

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
                            Err(format!(
                                "{role} couldn't copy {pronoun}self...\n{e:?}",
                                role = ROLE.load(&true_role, &rng)?,
                                pronoun = PRONOUN.load(&true_role, &rng)?,
                            ))?
                        } else {
                            // Just exit immediately on success, don't try to get too clever here~
                            eprintln!("{true_role} is now {new_role}~");
                            return Ok(0);
                        }
                    } else {
                        Err(format!(
                            "{role} couldn't copy {pronoun}self...\n(couldn't find own parent dir)",
                            role = ROLE.load(&true_role, &rng)?,
                            pronoun = PRONOUN.load(&true_role, &rng)?,
                        ))?;
                    }
                }
            }
        }
    }

    // mommy probably shouldn't be too smart with file system errors
    let mut maybe_beg = match check_need_beg(&rng, beg_chance) {
        Err(err) => {
            eprintln!(
                "\x1b[1m{} fought against the file system and lost~\x1b[0m",
                ROLE.load(&true_role, &rng)?
            );
            Err(err)?
        }
        Ok(beg) => beg,
    };

    let (response_kind, code) = if begging || maybe_beg.is_not_needed() {
        // Can add handling for if they are begging at the first required beg.
        // Because that means they are begging more than they need to.
        //
        // This could be good or bad. Depending on mommy's mood.
        if let Err(err) = maybe_beg.remove_lock() {
            eprintln!(
                "\x1b[1m{} fought against the file system and lost~\x1b[0m",
                ROLE.load(&true_role, &rng)?
            );
            Err(err)?
        };

        // Time for mommy to call cargo~
        let mut cmd = std::process::Command::new(cargo);
        cmd.args(args)
            .env(RECURSION_LIMIT_VAR, new_limit.to_string());
        let status = cmd.status()?;
        let code = status.code().unwrap_or(1);
        if is_quiet_mode_enabled(cmd.get_args()) {
            return Ok(code);
        }

        (
            if status.success() {
                ResponseType::Positive
            } else {
                ResponseType::Negative
            },
            code,
        )
    } else {
        // uh oh, someone isn't begging like they need to~

        match maybe_beg.needs {
            NeedsBeg::Needed(BegKind::First) => (ResponseType::FirstBeg, 69),
            NeedsBeg::Needed(BegKind::NotFirst) => (ResponseType::DidNotBeg, 69),
            NeedsBeg::NotNeeded => unreachable!(
                "mommy cannot reach this case~ someone did something naughty and needs a spanking~"
            ),
        }
    };

    // Time for mommy to tell you how you did~
    let response = select_response(&true_role, &rng, response_kind);

    pretty_print(response);

    Ok(code)
}

fn pretty_print(response: Result<String, String>) {
    let stylize = std::io::stderr().is_terminal();
    match (response, stylize) {
        (Ok(resp), true) => eprintln!("\x1b[1m{resp}\x1b[0m"),
        (Err(resp), true) => eprintln!("\x1b[31m{resp}\x1b[0m"),
        (Ok(resp) | Err(resp), false) => eprintln!("{resp}"),
    }
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

/// Mommy should be able to tell if this is her first time asking for a pet to beg~
enum BegKind {
    First,
    NotFirst,
}

enum NeedsBeg {
    NotNeeded,
    Needed(BegKind),
}

/// whether mommy needs her pet to beg, and how to create a lock if they do.
struct BegCtx {
    /// Whether or not mommy requires begging
    needs: NeedsBeg,

    /// Path to the lock file that may or may not exist
    path: PathBuf,
}

impl BegCtx {
    #[must_use]
    fn is_not_needed(&self) -> bool {
        matches!(self.needs, NeedsBeg::NotNeeded)
    }

    /// Remove a lock file
    fn remove_lock(&mut self) -> Result<(), std::io::Error> {
        match std::fs::remove_file(&self.path) {
            Err(err) if err.kind() == std::io::ErrorKind::NotFound => Ok(()),
            err => err,
        }
    }
}

/// does mommy need a little extra~?
fn check_need_beg(rng: &Rng, beg_chance: u8) -> Result<BegCtx, std::io::Error> {
    // TODO(?): Make this configurable where it's placed
    let lock_file_path = {
        let mut file = home::cargo_home()?;
        file.push(LOCK_FILE_NAME);
        file
    };

    // Fast path if mommy's pet is always good~
    if beg_chance == 0 {
        return Ok(BegCtx {
            needs: NeedsBeg::NotNeeded,
            path: lock_file_path,
        });
    }

    let beg_pick = rng.u8(..100);

    // Unconditionally create lock file to try and mitigate mitigate funny toctou
    let maybe_lock = std::fs::OpenOptions::new()
        .create_new(true)
        .append(true)
        .open(&lock_file_path);

    match maybe_lock {
        Ok(_) => Ok(BegCtx {
            needs: if beg_pick < beg_chance {
                NeedsBeg::Needed(BegKind::First)
            } else {
                NeedsBeg::NotNeeded
            },
            path: lock_file_path,
        }),
        Err(err) if err.kind() == std::io::ErrorKind::AlreadyExists => Ok(BegCtx {
            needs: NeedsBeg::Needed(BegKind::NotFirst),
            path: lock_file_path,
        }),
        Err(err) => Err(err),
    }
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
        ResponseType::Overflow => group.overflow,
        ResponseType::FirstBeg => group.beg_first,
        // TODO: Implement this category
        ResponseType::DidNotBeg => group.beg_first,
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
    overflow: &'a [&'a [Chunk<'a>]],
    beg_first: &'a [&'a [Chunk<'a>]],
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
