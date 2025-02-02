use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Name
    #[arg(short, long)]
    pub name: Option<String>,

    /// Types
    #[arg(short, long)]
    pub types: Option<String>,

    /// Limit
    #[arg(short, long, default_value_t = 10)]
    pub limit: u8,
}
