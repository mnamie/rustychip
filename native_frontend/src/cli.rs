use clap::Parser;

#[derive(Parser)]
pub struct Cli {
    pub path: String,

    #[arg(short, long, default_value_t = 400)]
    pub clock_speed: u32,
}
