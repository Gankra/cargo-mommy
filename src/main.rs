#[cfg(not(compat))]
use std::process::ExitCode;

#[cfg(not(compat))]
fn main() -> Result<ExitCode, Box<dyn std::error::Error>> {
    // `code` can be a negative value, so the cast can wrap
    main_impl().map(|code| (code as u8).into())
}

#[cfg(compat)]
fn main() {
    std::process::exit(main_impl().unwrap())
}

// `main`, but without the need to return a value that implements `Termination`
fn main_impl() -> Result<i32, Box<dyn std::error::Error>> {
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
        eprintln!("Good {}~\nMommy's so proud of you~ ❤️", mommys_little);
    } else {
        eprintln!("Mommy knows her little {} can do better~ ❤️", mommys_little);
    }
    eprintln!("\x1b[0m");
    Ok(status.code().unwrap_or(-1))
}

#[cfg(test)]
#[test]
fn test() {
    panic!("oops!!");
}
