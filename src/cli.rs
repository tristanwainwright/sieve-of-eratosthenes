use clap::Parser;

const VERSION: &str = "0.1.0";
const ABOUT: &str = "Sieve algorithm to generate list of prime numbers from 2 to n.";

#[derive(Parser, Debug)]
#[command(version = VERSION, about = ABOUT, long_about = None)]
pub struct Args {
    #[arg(short, allow_negative_numbers = false)]
    pub n: u32,
}
