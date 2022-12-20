use rand::{rngs::StdRng, seq::IteratorRandom, SeedableRng};
use std::process::ExitCode;

const NEGATIVE_RESPONSES: &str = include_str!("../responses/negative-responses.txt");
const POSITIVE_RESPONSES: &str = include_str!("../responses/positive-responses.txt");
const AFFECTIONATE_TERM_PLACEHOLDER: &str = "AFFECTIONATE_TERM";

fn main() -> Result<ExitCode, Box<dyn std::error::Error>> {
    let cargo = std::env::var("CARGO")?;
    let mommys_little = std::env::var("CARGO_MOMMYS_LITTLE").unwrap_or_else(|_| "girl".to_owned());
    let mut arg_iter = std::env::args();
    let _cargo = arg_iter.next();
    let _mommy = arg_iter.next();

    let mut cmd = std::process::Command::new(cargo);
    cmd.args(arg_iter);
    let status = cmd.status()?;
    eprintln!("\x1b[1m");
    if status.success() {
        eprintln!("{}", select_response(POSITIVE_RESPONSES, mommys_little))
    } else {
        eprintln!("{}", select_response(NEGATIVE_RESPONSES, mommys_little));
    }
    eprintln!("\x1b[0m");
    Ok(ExitCode::from(status.code().unwrap_or(-1) as u8))
}

fn select_response(responses: &str, affectionate_term: String) -> String {
    let mut rng = StdRng::from_entropy();
    let response = responses
        .lines()
        .choose(&mut rng)
        .expect("non-zero amount of responses")
        .replace(AFFECTIONATE_TERM_PLACEHOLDER, &affectionate_term);

    return response;
}

#[cfg(test)]
#[test]
fn test() {
    panic!("oops!!");
}
