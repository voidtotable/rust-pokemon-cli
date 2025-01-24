use futures::stream::FuturesUnordered;
use futures::StreamExt;
use rustemon::client::RustemonClient;
use rustemon::error::Error;
use rustemon::Follow;

#[derive(Debug)]
pub struct PokemonTable(Vec<PokemonRow>);

impl PokemonTable {
    pub async fn new_from_type(type_: &String, c: &RustemonClient) -> Result<PokemonTable, Error> {
        match rustemon::pokemon::type_::get_by_name(type_, c).await {
            Ok(t) => {
                let futures = t.pokemon.iter().map(|tp| async {
                    let pokemon = tp.pokemon.follow(&c).await.unwrap();
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
                return Ok(PokemonTable(rows));
            }
            Err(_) => todo!(),
        }
    }
}

#[derive(Debug)]
pub struct PokemonRow {
    name: String,
    types: Vec<String>,
    abilities: Vec<String>,
    moves: Vec<String>,
}

impl PokemonRow {
    pub async fn new(name: &String, c: &RustemonClient) -> Result<PokemonRow, Error> {
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
