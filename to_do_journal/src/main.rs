use structopt::StructOpt;

mod cli;
mod tasks;

fn main() {
    cli::CommandLineArgs::from_args();
}