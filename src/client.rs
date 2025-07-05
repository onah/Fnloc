use clap::Parser;

/// Function analyzer for Rust code - counts lines of code, comments, and empty lines per function
#[derive(Parser)]
#[command(name = "fnloc")]
#[command(version = "0.1.0")]
#[command(about = "Analyzes Rust functions and counts lines of code")]
#[command(long_about = None)]
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
