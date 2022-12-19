use std::process::ExitCode;

fn main() -> Result<ExitCode, Box<dyn std::error::Error>> {
    let cargo = std::env::var("CARGO")?;
    let mommys_little = std::env::var("CARGO_MOMMYS_LITTLE").unwrap_or_else(|_| "girl".to_owned());
    let mut arg_iter = std::env::args();
    let _cargo = arg_iter.next();

    let mut cmd = std::process::Command::new(cargo);
    cmd.args(arg_iter);
    let status = cmd.status()?;
    eprintln!("\x1b[1m");
    if status.success() {
        eprintln!("Good {}~\nMommy's so proud of you~ ❤️", mommys_little);
    } else {
        eprintln!("Mommy knows her little {} can do better~ ❤️", mommys_little);
    }
    eprintln!("\x1b[0m");
    Ok(ExitCode::from(status.code().unwrap_or(-1) as u8))
}

#[cfg(test)]
#[test]
fn test() {
    panic!("oops!!");
}
