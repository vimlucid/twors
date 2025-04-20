use std::{
    io,
    process::{Command, ExitCode, ExitStatus},
};

fn main() -> ExitCode {
    if !is_success(check_fmt()) {
        return ExitCode::FAILURE;
    }

    if !is_success(check_clippy()) {
        return ExitCode::FAILURE;
    }

    ExitCode::SUCCESS
}

fn check_fmt() -> io::Result<ExitStatus> {
    println!("Running rustfmt...");

    Command::new("cargo")
        .args(["fmt", "--check"])
        .spawn()?
        .wait()
}

fn check_clippy() -> io::Result<ExitStatus> {
    println!("Running clippy...");

    Command::new("cargo")
        .args(["clippy", "--", "-D", "warnings"])
        .spawn()?
        .wait()
}

fn is_success(result: io::Result<ExitStatus>) -> bool {
    match result {
        Ok(status) => status.success(),
        Err(_) => false,
    }
}
