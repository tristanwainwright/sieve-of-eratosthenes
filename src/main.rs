mod cli;
mod sieve;

use crate::cli::Args;
use crate::sieve::sieve_of_eratosthenes;
use clap::Parser;

fn main() {
    let Args { n } = Args::parse();
    let primes: Vec<u32> = sieve_of_eratosthenes(n);
    println!("{:?}", primes);
}
