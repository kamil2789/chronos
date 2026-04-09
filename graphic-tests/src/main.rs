use clap::Parser;
use tests::run;

mod args_parser;
mod image_utils;
mod reporter;
mod test_collector;
mod tests;
mod workspace;

fn main() {
    run(&args_parser::Args::parse());
}
