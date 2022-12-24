#![allow(clippy::let_and_return)]

use rand::{rngs::StdRng, seq::SliceRandom, SeedableRng};

mod config;
use config::{load_config, Responses};

const AFFECTIONATE_TERM_PLACEHOLDER: &str = "AFFECTIONATE_TERM";
const MOMMYS_PRONOUN_PLACEHOLDER: &str = "MOMMYS_PRONOUN";
const MOMMYS_ROLE_PLACEHOLDER: &str = "MOMMYS_ROLE";

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
    let mut arg_iter = std::env::args();
    let _cargo = arg_iter.next();
    let _mommy = arg_iter.next();

    let mut cmd = std::process::Command::new(cargo);
    cmd.args(arg_iter);
    let status = cmd.status()?;
    eprintln!("\x1b[1m");
    if status.success() {
        eprintln!("{}", select_response(ResponseType::Positive))
    } else {
        eprintln!("{}", select_response(ResponseType::Negative));
    }
    eprintln!("\x1b[0m");
    Ok(status.code().unwrap_or(-1))
}

fn select_response(response_type: ResponseType) -> String {
    let config = load_config().unwrap();

    let mut rng = StdRng::from_entropy();

    // Get mommy's options~
    let affectionate_terms = parse_options(config.affectionate_terms);
    let mommys_pronouns = parse_options(config.pronouns);
    let mommys_roles = parse_options(config.roles);

    // Choose what mommy will say~
    let responses: Responses = config.responses.unwrap_or_default();

    let responses_binding = match response_type {
        ResponseType::Positive => responses.positive,
        ResponseType::Negative => responses.negative,
    }
    .unwrap();
    let response = responses_binding
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

fn parse_options(input: String) -> Vec<String> {
    input.split('/').map(|s| s.to_string()).collect()
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
