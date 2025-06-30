use clap::Parser;

/// Function analyzer for Rust code - counts lines of code, comments, and empty lines per function
#[derive(Parser)]
#[command(name = "fnloc")]
#[command(version = "0.1.0")]
#[command(about = "Analyzes Rust functions and counts lines of code")]
#[command(long_about = None)]
pub struct Cli {
    /// Directory to analyze for Rust files
    #[arg(value_name = "DIRECTORY")]
    #[arg(default_value = "./src")]
    #[arg(help = "Directory to scan for Rust files")]
    pub directory: String,

    /// Show verbose output with additional information
    #[arg(short, long)]
    #[arg(help = "Enable verbose output")]
    pub verbose: bool,

    /// Minimum lines of code to display functions (filter small functions)
    #[arg(short = 'm', long = "min-lines")]
    #[arg(default_value = "0")]
    #[arg(help = "Minimum lines of code to display functions")]
    pub min_lines: usize,

    /// Maximum number of functions to display
    #[arg(short = 'l', long = "limit")]
    #[arg(help = "Maximum number of functions to display")]
    pub limit: Option<usize>,

    /// Sort by different criteria
    #[arg(short, long)]
    #[arg(value_enum)]
    #[arg(default_value = "code")]
    #[arg(help = "Sort criteria for function listing")]
    pub sort: SortBy,

    /// Output format
    #[arg(short = 'f', long = "format")]
    #[arg(value_enum)]
    #[arg(default_value = "table")]
    #[arg(help = "Output format")]
    pub format: OutputFormat,
}

#[derive(clap::ValueEnum, Clone, Debug)]
pub enum SortBy {
    /// Sort by total lines (default)
    Total,
    /// Sort by code lines
    Code,
    /// Sort by comment lines
    Comments,
    /// Sort by function name
    Name,
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
