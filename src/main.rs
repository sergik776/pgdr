mod cli_commands;

use crate::cli_commands::Cli;
use clap::{Parser, CommandFactory, error::ErrorKind as ClapErrorKind};
use std::fs::File;
use std::io::{self, Read, Write};

fn main() -> io::Result<()> {
    let cli = Cli::parse();
    if !cli.lowercase && !cli.uppercase && !cli.numbers && !cli.symbols {
        Cli::command()
            .error(ClapErrorKind::MissingRequiredArgument, "Specify at least one: -L, -U, -n, -s")
            .exit();
    }
    let mut lookup = [false; 256];
    for b in 0..=255u8 {
        if (cli.lowercase && b.is_ascii_lowercase()) ||
           (cli.uppercase && b.is_ascii_uppercase()) ||
           (cli.numbers && b.is_ascii_digit()) ||
           (cli.symbols && b"!@#$%^&*()_+-=".contains(&b)) {
            lookup[b as usize] = true;
        }
    }
    let path = if cli.random_source { "/dev/random" } else { "/dev/urandom" };
    let mut entropy_source = File::open(path)?;
    let mut stdout = io::BufWriter::with_capacity(cli.buffer_size, io::stdout().lock());
    let mut read_buf = vec![0u8; cli.buffer_size];
    let mut written_total = 0;
    while written_total < cli.length {
        let n = entropy_source.read(&mut read_buf)?;
        if n == 0 {
            return Err(io::Error::new(
                io::ErrorKind::UnexpectedEof,
                format!("Insufficient entropy available in {}. Try using /dev/urandom (remove -R) or wait for more system interrupts.", path)
            ));
        }
        for &byte in &read_buf[..n] {
            if lookup[byte as usize] {
                stdout.write_all(&[byte])?;
                written_total += 1;
                if written_total >= cli.length {
                    break;
                }
            }
        }
    }
    stdout.write_all(b"\n")?;
    stdout.flush()?;
    Ok(())
}
