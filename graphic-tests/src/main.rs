use clap::Parser;
use tests::run_framework;

mod args_parser;
mod image_utils;
mod reporter;
mod test_collector;
mod tests;
mod workspace;

fn main() {
    run_framework(&args_parser::Args::parse());
}
