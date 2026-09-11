//! Non-production entry point for deterministic documentation capture.

#[path = "support/documentation_capture.rs"]
mod capture;

fn main() {
    if let Err(error) = capture::run(std::env::args_os().skip(1)) {
        eprintln!("documentation capture failed: {error}");
        std::process::exit(1);
    }
}
