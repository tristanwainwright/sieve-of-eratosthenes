use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[arg(short, allow_negative_numbers = false)]
    pub n: u32,
}
