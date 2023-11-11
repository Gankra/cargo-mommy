#![allow(clippy::let_and_return)]

use fastrand::Rng;

include!(concat!(env!("OUT_DIR"), "/responses.rs"));

const AFFECTIONATE_TERM_PLACEHOLDER: &str = "AFFECTIONATE_TERM";
#[allow(dead_code)]
const DENIGRATING_TERM_PLACEHOLDER: &str = "DENIGRATING_TERM";
const MOMMYS_PRONOUN_PLACEHOLDER: &str = "MOMMYS_PRONOUN";
const MOMMYS_ROLE_PLACEHOLDER: &str = "MOMMYS_ROLE";
#[allow(dead_code)]
const MOMMYS_PART_PLACEHOLDER: &str = "MOMMYS_PART";

const AFFECTIONATE_TERMS_ENV_VAR: &str = "CARGO_MOMMYS_LITTLE";
#[allow(dead_code)]
const DENIGRATING_TERMS_ENV_VAR: &str = "CARGO_MOMMYS_FUCKING";
const MOMMYS_PRONOUNS_ENV_VAR: &str = "CARGO_MOMMYS_PRONOUNS";
#[allow(dead_code)]
const MOMMYS_PARTS_ENV_VAR: &str = "CARGO_MOMMYS_PARTS";
const MOMMYS_ROLES_ENV_VAR: &str = "CARGO_MOMMYS_ROLES";
const MOMMYS_MOODS_ENV_VAR: &str = "CARGO_MOMMYS_MOODS";
const MOMMYS_EMOTES_ENV_VAR: &str = "CARGO_MOMMYS_EMOTES";

const AFFECTIONATE_TERMS_DEFAULT: &str = "girl";
const MOMMYS_PRONOUNS_DEFAULT: &str = "her";
const MOMMYS_ROLES_DEFAULT: &str = "mommy";
const MOMMYS_MOODS_DEFAULT: &str = "chill";
const MOMMYS_EMOTES_DEFAULT: &str = "❤️/💖/💗/💓/💞";

#[cfg(feature = "yikes")]
const DENIGRATING_TERMS_DEFAULT: &str = "slut/toy/pet/pervert/whore";
#[cfg(feature = "yikes")]
const MOMMYS_PARTS_DEFAULT: &str = "milk";

const SUPPORTED_MOODS: &[&str] = &[
    "chill",
    #[cfg(feature = "thirsty")]
    "thirsty",
    #[cfg(feature = "yikes")]
    "yikes",
];

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

    let response = if status.success() {
        select_response(ResponseType::Positive)
    } else {
        select_response(ResponseType::Negative)
    };

    match response {
        Ok(resp) => eprintln!("\x1b[1m{}\x1b[0m", resp),
        Err(resp) => eprintln!("\x1b[31m{}\x1b[0m", resp),
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
    let affectionate_terms = parse_options(AFFECTIONATE_TERMS_ENV_VAR, AFFECTIONATE_TERMS_DEFAULT);
    let mommys_pronouns = parse_options(MOMMYS_PRONOUNS_ENV_VAR, MOMMYS_PRONOUNS_DEFAULT);
    let mommys_roles = parse_options(MOMMYS_ROLES_ENV_VAR, MOMMYS_ROLES_DEFAULT);
    let mommys_moods = parse_options(MOMMYS_MOODS_ENV_VAR, MOMMYS_MOODS_DEFAULT);
    let mommys_emotes = parse_options(MOMMYS_EMOTES_ENV_VAR, MOMMYS_EMOTES_DEFAULT);

    #[cfg(feature = "yikes")]
    let denigrating_terms = parse_options(DENIGRATING_TERMS_ENV_VAR, DENIGRATING_TERMS_DEFAULT);
    #[cfg(feature = "yikes")]
    let mommys_parts = parse_options(MOMMYS_PARTS_ENV_VAR, MOMMYS_PARTS_DEFAULT);

    // Choose what mood mommy is in~
    let mood = &mommys_moods[rng.usize(..mommys_moods.len())];

    let Some(responses) = &RESPONSES
        .iter()
        .find(|(mood_mode, _)| mood_mode == mood)
        .map(|x| x.1)
    else {
        let supported_moods_str = SUPPORTED_MOODS.join(", ");
        return Err(format!(
            "Unknown mood {mood}! We were compiled with: {supported_moods_str}"
        ));
    };

    // Choose what mommy will say~
    let responses = match response_type {
        ResponseType::Positive => responses[0],
        ResponseType::Negative => responses[1],
    };
    let response = &responses[rng.usize(..responses.len())];

    // Apply options to the message template~
    let response = apply_template(
        response,
        AFFECTIONATE_TERM_PLACEHOLDER,
        &affectionate_terms,
        &rng,
    );
    #[cfg(feature = "yikes")]
    let response = apply_template(
        &response,
        DENIGRATING_TERM_PLACEHOLDER,
        &denigrating_terms,
        &rng,
    );
    #[cfg(feature = "yikes")]
    let response = apply_template(&response, MOMMYS_PART_PLACEHOLDER, &mommys_parts, &rng);
    let response = apply_template(
        &response,
        MOMMYS_PRONOUN_PLACEHOLDER,
        &mommys_pronouns,
        &rng,
    );
    let mut response = apply_template(&response, MOMMYS_ROLE_PLACEHOLDER, &mommys_roles, &rng);

    let should_emote = rng.bool();
    if should_emote && !mommys_emotes.is_empty() {
        let emote = &mommys_emotes[rng.usize(..mommys_emotes.len())];
        response.push(' ');
        response.push_str(emote);
    }

    // Done~!
    Ok(response)
}

fn parse_options(env_var: &str, default: &str) -> Vec<String> {
    std::env::var(env_var)
        .unwrap_or_else(|_| default.to_owned())
        .split('/')
        .map(|s| s.to_owned())
        .collect()
}

fn apply_template(input: &str, template_key: &str, options: &[String], rng: &Rng) -> String {
    let mut last_position = 0;
    let mut output = String::new();
    for (index, matched) in input.match_indices(template_key) {
        output.push_str(&input[last_position..index]);
        output.push_str(&options[rng.usize(..options.len())]);
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
