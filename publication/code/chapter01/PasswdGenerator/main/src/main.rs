use anyhow::{bail, Result};
use clap::Parser;
use encryptor::password::generate_password;

/// A simple password generator for any account
#[derive(Parser, Debug)]
#[clap(version, about, long_about = None)]
struct Args {
    /// Seed to generate password
    #[clap(short, long)]
    seed: String,

    /// Length of password
    #[clap(short, long, default_value_t = 16)]
    length: usize,
}

fn main() -> Result<()> {
    let args = Args::parse();
    if args.seed.len() < 4 {
        bail!("seed {} length must >= 4", &args.seed);
    }

    let (seed, length) = (args.seed, args.length);
    let passwd = generate_password(&seed[..], length);
    match passwd {
        Ok(val) => println!("{}", val),
        Err(err) => println!("{}", err),
    }

    Ok(())
}
