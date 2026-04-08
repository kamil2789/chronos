use clap::Parser;
use tests::run;

mod args_parser;
mod test_collector;
mod tests;
mod workspace;

fn main() {
    run(&args_parser::Args::parse());
}
