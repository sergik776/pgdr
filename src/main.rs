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
    let mut charset: Vec<u8> = Vec::new();
    if cli.lowercase {
        charset.extend(b'a'..=b'z');
    }
    if cli.uppercase {
        charset.extend(b'A'..=b'Z');
    }
    if cli.numbers {
        charset.extend(b'0'..=b'9');
    }
    if cli.symbols {
        charset.extend_from_slice(b"!@#$%^&*()_+-=");
    }
    let charset_len = charset.len() as u32;
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
        for &byte in &read_buf[..] {
            if written_total >= cli.length as usize { break; }
            let idx = (byte as u32) % charset_len;
            stdout.write_all(&[charset[idx as usize]])?;
            written_total += 1;
        }
    }
    stdout.write_all(b"\n")?;
    stdout.flush()?;
    Ok(())
}
