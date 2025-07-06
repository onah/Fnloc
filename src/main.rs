use clap::Parser;
use fnloc::Client;

fn main() {
    // Handle cargo subcommand invocation
    // When called as 'cargo fnloc', the first argument will be 'fnloc'
    let args: Vec<String> = std::env::args().collect();
    let filtered_args = if args.len() > 1 && args[1] == "fnloc" {
        // Skip the first two arguments: program name and 'fnloc'
        std::iter::once(args[0].clone())
            .chain(args.iter().skip(2).cloned())
            .collect()
    } else {
        args
    };

    let cli = Client::parse_from(filtered_args);
    fnloc::run_analysis(&cli);
}
