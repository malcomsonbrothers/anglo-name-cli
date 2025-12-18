use std::process::ExitCode;

fn main() -> ExitCode {
    if let Err(err) = anglo_name_cli::run() {
        eprintln!("Error: {err:?}");
        ExitCode::FAILURE
    } else {
        ExitCode::SUCCESS
    }
}
