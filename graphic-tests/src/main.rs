use clap::Parser;
use std::process::ExitCode;
use tests::run_framework;
use tracing_subscriber::EnvFilter;

mod args_parser;
mod image_comparison;
mod reporter;
mod test_collector;
mod tests;
mod workspace;

fn main() -> ExitCode {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| EnvFilter::new("chronos=info,wgpu=error")),
        )
        .init();

    let results = run_framework(&args_parser::Args::parse());

    if results.has_failures() {
        ExitCode::FAILURE
    } else {
        ExitCode::SUCCESS
    }
}
