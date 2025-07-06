use clap::Parser;

/// Function analyzer for Rust code - counts lines of code, comments, and empty lines per function
///
/// Can be used as a standalone command or as a Cargo subcommand:
/// - Standalone: fnloc [OPTIONS] [DIRECTORY]
/// - Cargo subcommand: cargo fnloc [OPTIONS] [DIRECTORY]
#[derive(Parser)]
#[command(name = "fnloc")]
#[command(version = "0.1.0")]
#[command(about = "Analyzes Rust functions and counts lines of code")]
#[command(
    long_about = "Function analyzer for Rust code that counts lines of code, comments, empty lines, cyclomatic complexity, and nesting depth per function.\n\nCan be used as a standalone command or as a Cargo subcommand."
)]
pub struct Client {
    /// Directory to analyze for Rust files
    #[arg(value_name = "DIRECTORY")]
    #[arg(default_value = "./src")]
    #[arg(help = "Directory to scan for Rust files")]
    pub directory: String,

    /// Output format
    #[arg(short = 'f', long = "format")]
    #[arg(value_enum)]
    #[arg(default_value = "table")]
    #[arg(help = "Output format")]
    pub format: OutputFormat,
}

#[derive(clap::ValueEnum, Clone, Debug)]
pub enum OutputFormat {
    /// Table format (default)
    Table,
    /// JSON format
    Json,
    /// CSV format
    Csv,
}
