use clap::Parser;

use cli::Args;

use pokemon::{PokemonMeta, PokemonTable, PokemonDetails};
use rustemon::client::{
    CACacheManager, CacheMode, CacheOptions, Environment, RustemonClientBuilder,
};
use rustemon::error::Error;
use rustemon::Follow;

use std::time::Duration;

mod cli;
mod cyphersystem;
mod pokemon;



#[tokio::main]
async fn main() -> eyre::Result<()> {
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

    /*
    if let Some(name) = args.name {
        match PokemonMeta::new(&name, &client).await {
            Ok(row) => println!("{:#?}", row),
            Err(_) => todo!(),
        };
    }
    */

    // Testing
    if let Some(name) = args.name {
        match PokemonMeta::new(&name, &client).await {
            Ok(meta) => {
                //println!("{:#?}", meta)

                match PokemonDetails::new_from_meta(&meta, &client).await {
                    Ok(stat) => println!("{:#?}", stat),
                    Err(_) => todo!(),
                }
            }
            Err(_) => todo!(),
        };
    }

    if let Some(types) = args.types {
        match PokemonTable::new_from_type(&types, &client).await {
            Ok(table) => println!("{:#?}", table),
            Err(_) => todo!(),
        };
    }

    Ok(())
}
