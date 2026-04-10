use clap::Parser;
use tests::run_framework;
use tracing_subscriber::EnvFilter;

mod args_parser;
mod image_comparison;
mod reporter;
mod test_collector;
mod tests;
mod workspace;

fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| EnvFilter::new("chronos=info,wgpu=error")),
        )
        .init();

    run_framework(&args_parser::Args::parse());
}
