use clap::Parser;
use tests::run;

mod args_parser;
mod tests;
mod workspace;


fn main() {
    run(&args_parser::Args::parse());
}
