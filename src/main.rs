use clap::Parser;
use futures::stream::FuturesUnordered;
use futures::StreamExt;

use rustemon::error::Error;
use std::time::Duration;

use rustemon::client::{
    CACacheManager, CacheMode, CacheOptions, Environment, RustemonClient, RustemonClientBuilder,
};

use rustemon::Follow;

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

    if let Some(types) = args.types.as_deref() {

        /*
        let type_ = rustemon::pokemon::type_::get_by_name(types, &client).await;

        match type_ {
            Ok(type_) => {
                let futures = type_.pokemon.iter().map(|tp| async {
                    let pokemon = tp.pokemon.follow(&client).await.unwrap();
                    let types: Vec<String> = pokemon
                        .types
                        .iter()
                        .map(|t| t.type_.name.to_string())
                        .collect();
                    let abilities: Vec<String> = pokemon
                        .abilities
                        .iter()
                        .map(|a| a.ability.name.to_string())
                        .collect();
                    let moves: Vec<String> = pokemon
                        .moves
                        .iter()
                        .map(|m| m.move_.name.to_string())
                        .collect();
                    PokemonRow {
                        name: pokemon.name,
                        types,
                        abilities,
                        moves,
                    }
                });

                let mut stream = FuturesUnordered::from_iter(futures);

                let mut rows: Vec<PokemonRow> = Vec::new();
                while let Some(row) = stream.next().await {
                    rows.push(row);
                }

                //println!("{:#?}", type_);
                println!("{:#?}", rows);
            }
            Err(_) => todo!(),
        }
        */
    }
}

#[derive(Debug)]
struct PokemonTable(Vec<PokemonRow>);

impl PokemonTable {
    async fn new_from_type(type_: String, c: &RustemonClient) -> Result<PokemonTable, Error> {
        todo!()
    }
}

#[derive(Debug)]
struct PokemonRow {
    name: String,
    types: Vec<String>,
    abilities: Vec<String>,
    moves: Vec<String>,
}

impl PokemonRow {
    async fn new(name: &String, c: &RustemonClient) -> Result<PokemonRow, Error> {
        match rustemon::pokemon::pokemon::get_by_name(name, c).await {
            Ok(pokemon) => {
                let types: Vec<String> = pokemon
                    .types
                    .iter()
                    .map(|t| t.type_.name.to_string())
                    .collect();

                let abilities: Vec<String> = pokemon
                    .abilities
                    .iter()
                    .map(|a| a.ability.name.to_string())
                    .collect();

                let moves: Vec<String> = pokemon
                    .moves
                    .iter()
                    .map(|m| m.move_.name.to_string())
                    .collect();

                Ok(PokemonRow {
                    name: pokemon.name,
                    types,
                    abilities,
                    moves,
                })
            }
            Err(e) => Err(e),
        }
    }
}

/* Example Output

| NAME      | TYPES        | ABILITIES    | FLAVOR TEXT    |
| --------- | ------------ | ------------ | -----------    |
| Bulbasaur | grass,poison | growl,tackle | blah blah blah |
| Rhyhorn   | ground,rock  | fart,giggle  | blah blah blah |



*/
