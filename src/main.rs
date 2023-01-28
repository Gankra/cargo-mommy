#![allow(clippy::let_and_return)]

use rand::{rngs::StdRng, seq::SliceRandom, SeedableRng};

include!(concat!(env!("OUT_DIR"), "/responses.rs"));

const AFFECTIONATE_TERM_PLACEHOLDER: &str = "AFFECTIONATE_TERM";
const MOMMYS_PRONOUN_PLACEHOLDER: &str = "MOMMYS_PRONOUN";
const MOMMYS_ROLE_PLACEHOLDER: &str = "MOMMYS_ROLE";

const AFFECTIONATE_TERMS_ENV_VAR: &str = "CARGO_MOMMYS_LITTLE";
const MOMMYS_PRONOUNS_ENV_VAR: &str = "CARGO_MOMMYS_PRONOUNS";
const MOMMYS_ROLES_ENV_VAR: &str = "CARGO_MOMMYS_ROLES";

const AFFECTIONATE_TERMS_DEFAULT: &str = "girl";
const MOMMYS_PRONOUNS_DEFAULT: &str = "her";
const MOMMYS_ROLES_DEFAULT: &str = "mommy";

enum ResponseType {
    Positive,
    Negative,
}

fn main() {
    // Ideally mommy would use ExitCode but that's pretty new and mommy wants
    // to support more little ones~
    let code = real_main().unwrap_or_else(|e| {
        eprintln!("Error: {:?}", e);
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
    eprintln!("\x1b[1m");
    if status.success() {
        eprintln!("{}", select_response(ResponseType::Positive))
    } else {
        eprintln!("{}", select_response(ResponseType::Negative));
    }
    eprintln!("\x1b[0m");
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

fn select_response(response_type: ResponseType) -> String {
    let mut rng = StdRng::from_entropy();

    // Get mommy's options~
    let affectionate_terms = parse_options(AFFECTIONATE_TERMS_ENV_VAR, AFFECTIONATE_TERMS_DEFAULT);
    let mommys_pronouns = parse_options(MOMMYS_PRONOUNS_ENV_VAR, MOMMYS_PRONOUNS_DEFAULT);
    let mommys_roles = parse_options(MOMMYS_ROLES_ENV_VAR, MOMMYS_ROLES_DEFAULT);

    // Choose what mommy will say~
    let response = match response_type {
        ResponseType::Positive => POSITIVE_RESPONSES,
        ResponseType::Negative => NEGATIVE_RESPONSES,
    }
    .choose(&mut rng)
    .expect("non-zero amount of responses");

    // Apply options to the message template~
    let response = apply_template(
        response,
        AFFECTIONATE_TERM_PLACEHOLDER,
        &affectionate_terms,
        &mut rng,
    );
    let response = apply_template(
        &response,
        MOMMYS_PRONOUN_PLACEHOLDER,
        &mommys_pronouns,
        &mut rng,
    );
    let response = apply_template(&response, MOMMYS_ROLE_PLACEHOLDER, &mommys_roles, &mut rng);

    // Done~!
    response
}

fn parse_options(env_var: &str, default: &str) -> Vec<String> {
    std::env::var(env_var)
        .unwrap_or_else(|_| default.to_owned())
        .split('/')
        .map(|s| s.to_owned())
        .collect()
}

fn apply_template(input: &str, template_key: &str, options: &[String], rng: &mut StdRng) -> String {
    let mut last_position = 0;
    let mut output = String::new();
    for (index, matched) in input.match_indices(template_key) {
        output.push_str(&input[last_position..index]);
        output.push_str(options.choose(rng).unwrap());
        last_position = index + matched.len();
    }
    output.push_str(&input[last_position..]);
    output
}

#[cfg(test)]
#[test]
fn test() {
    // Uncomment if you want a failing test
    // panic!("oops!!");
}
