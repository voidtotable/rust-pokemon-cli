use askama::Template;
use clap::Parser;

use cli::Args;

use cyphersystem::PokemonNPC;
use pokemon::{Meta, Pokemon, PokemonTable};
use rustemon::client::{
    CACacheManager, CacheMode, CacheOptions, Environment, RustemonClientBuilder,
};

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

    let rng = rand::thread_rng();
        
   

    // Testing
    if let Some(name) = args.name {
        match Pokemon::new(&name, &client).await {
            Ok(pokemon) => {
                //println!("{:#?}", pokemon)

                let npc = PokemonNPC::new_from_pokemon(pokemon).prune(3, rng);
                println!("{}", npc.render().unwrap())
            }
            Err(_) => todo!(),
        }
    }

    if let Some(types) = args.types {
        match PokemonTable::new_from_type(&types, &client).await {
            Ok(table) => println!("{:#?}", table),
            Err(_) => todo!(),
        };
    }

    Ok(())
}
