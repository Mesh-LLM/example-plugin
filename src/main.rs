use std::process::ExitCode;

fn main() -> ExitCode {
    match mesh_example::run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("example: {error:#}");
            ExitCode::FAILURE
        }
    }
}
