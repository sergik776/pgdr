use clap::{Parser, CommandFactory, error::ErrorKind};

#[derive(Parser)]
#[command(
    name = "pgdr",
    version = "1.0", 
    about = "High-performance cryptographically secure password generator"
)]
pub struct Cli {
    /// Total number of characters to generate
    #[arg(short = 'l', long, default_value_t = 16, value_name = "INT")]
    pub length: usize,

    /// Include lowercase letters (a-z)
    #[arg(short = 'L', long)]
    pub lowercase: bool,

    /// Include uppercase letters (A-Z)
    #[arg(short = 'U', long)]
    pub uppercase: bool,

    /// Include digits (0-9)
    #[arg(short = 'N', long)]
    pub numbers: bool,

    /// Include special symbols (!@#$%^&*()_+-=)
    #[arg(short = 'S', long)]
    pub symbols: bool,

    /// IO buffer size in bytes for reading and writing
    #[arg(short = 'b', long, default_value_t = 8192, value_name = "BYTES")]
    pub buffer_size: usize,

    /// Use /dev/random (blocking) instead of /dev/urandom (non-blocking)
    #[arg(short = 'r', long)]
    pub random_source: bool,
}
