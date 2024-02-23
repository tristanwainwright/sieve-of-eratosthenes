use clap::Parser;

const N_HELP: &str = "Upper limit for generation, must be >= 2";

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[arg(short, value_parser = clap::value_parser!(u32).range(2..), help = N_HELP)]
    pub n: u32,
}
