use clap::Parser;

use pokemon::{PokemonRow, PokemonTable};
use rustemon::client::{
    CACacheManager, CacheMode, CacheOptions, Environment, RustemonClientBuilder,
};
use rustemon::error::Error;
use rustemon::Follow;

use std::time::Duration;

pub mod pokemon;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Name
    #[arg(short, long)]
    name: Option<String>,

    /// Types
    #[arg(short, long)]
    types: Option<String>,

    /// Limit
    #[arg(short, long, default_value_t = 10)]
    limit: u8,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    let client = RustemonClientBuilder::default()
        .with_mode(CacheMode::NoStore)
        .with_manager(CACacheManager::default())
        .with_options(CacheOptions {
            shared: true,
            cache_heuristic: 0.2,
            immutable_min_time_to_live: Duration::from_secs(3600),
            ignore_cargo_cult: true,
        })
        .with_environment(Environment::Custom(
            "https://pokeapi.co/api/v2/".parse().unwrap(),
        ))
        .try_build()
        .unwrap();

    if let Some(name) = args.name {
        match PokemonRow::new(&name, &client).await {
            Ok(row) => println!("{:#?}", row),
            Err(_) => todo!(),
        };
    }

    if let Some(types) = args.types {
        match PokemonTable::new_from_type(&types, &client).await {
            Ok(table) => println!("{:#?}", table),
            Err(_) => todo!(),
        };
    }
}
