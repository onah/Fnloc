use clap::Parser;
use fnloc::Client;

fn main() {
    let cli = Client::parse();
    fnloc::run_analysis(&cli);
}
