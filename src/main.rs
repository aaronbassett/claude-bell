use std::process::ExitCode as StdExitCode;

fn main() -> StdExitCode {
    match claude_bell::run() {
        Ok(code) => StdExitCode::from(code as u8),
        Err(e) => {
            eprintln!("Error: {}", e);
            StdExitCode::from(claude_bell::ExitCode::AppError as u8)
        }
    }
}
