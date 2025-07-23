use clap::Parser;
use fnloc::Client;

fn main() {
    let cli = Client::parse();

    if let Err(e) = fnloc::run_analysis(&cli) {
        eprintln!("Error: {e}");
        std::process::exit(1);
    }

    std::process::exit(0);
}
