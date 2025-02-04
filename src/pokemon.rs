use futures::stream::FuturesUnordered;
//use futures::StreamExt;
use eyre::Error;
use itertools::Itertools;
use rustemon::client::RustemonClient;
use rustemon::Follow;
use tokio_stream::{self as stream, StreamExt};

#[derive(Debug)]
pub struct PokemonTable(Vec<Meta>);

impl PokemonTable {
    pub async fn new_from_type(type_: &String, c: &RustemonClient) -> Result<PokemonTable, Error> {
        match rustemon::pokemon::type_::get_by_name(type_, c).await {
            Ok(t) => {
                let futures = t.pokemon.iter().map(|tp| async {
                    let pokemon = tp.pokemon.follow(&c).await?;
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
                    Ok(Meta {
                        name: pokemon.name,
                        types,
                        abilities,
                        moves,
                    })
                });
                let rows = FuturesUnordered::from_iter(futures)
                    .collect::<Result<_, rustemon::error::Error>>()
                    .await?;

                return Ok(PokemonTable(rows));
            }
            Err(e) => return Err(e.into()),
        }
    }
}

#[derive(Debug)]
pub struct Meta {
    name: String,
    types: Vec<String>,
    abilities: Vec<String>,
    moves: Vec<String>,
}

impl Meta {
    pub async fn new(name: &String, c: &RustemonClient) -> Result<Meta, Error> {
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

                Ok(Meta {
                    name: pokemon.name,
                    types,
                    abilities,
                    moves,
                })
            }
            Err(e) => Err(e.into()),
        }
    }
}

#[derive(Debug)]
pub struct Pokemon {
    pub name: String,
    pub flavor: Vec<String>,
    pub types: Vec<String>,
    pub abilities: Vec<Ability>,
    pub moves: Vec<Move>,
}

impl Pokemon {
    pub async fn new(name: &String, c: &RustemonClient) -> Result<Pokemon, Error> {
        let meta = Meta::new(name, c).await?;

        Self::new_from_meta(&meta, c).await
    }

    pub async fn new_from_meta(meta: &Meta, c: &RustemonClient) -> Result<Pokemon, Error> {
        let name = meta.name.clone();
        let types = meta.types.clone();

        let pokemon = rustemon::pokemon::pokemon::get_by_name(&meta.name, c).await?;

        // Flavor Text
        let species = pokemon.species.follow(c).await?;
        let flavor: Vec<String> = species
            .flavor_text_entries
            .iter()
            .filter(|f| f.language.name.contains("en"))
            .map(|f| f.flavor_text.replace("\n", " ").replace("\u{c}", ""))
            .unique()
            .collect();

        // Abilities
        let futures = meta
            .abilities
            .iter()
            .map(|ability| async { rustemon::pokemon::ability::get_by_name(ability, c).await });

        let abilities: Vec<rustemon::model::pokemon::Ability> =
            FuturesUnordered::from_iter(futures)
                .collect::<Result<_, _>>()
                .await?;

        let abilities: Vec<Ability> = abilities
            .iter()
            .map(|a| Ability {
                name: a.name.clone(),
                description: a
                    .flavor_text_entries
                    .iter()
                    .filter(|a| a.language.name.contains("en"))
                    .map(|a| a.flavor_text.replace("\n", " "))
                    .collect::<Vec<String>>()[0]
                    .clone(),
            })
            .collect();

        // Moves
        let futures = meta
            .moves
            .iter()
            .map(|move_| async { rustemon::moves::move_::get_by_name(move_, c).await });

        let moves: Vec<rustemon::model::moves::Move> = FuturesUnordered::from_iter(futures)
            .collect::<Result<_, _>>()
            .await?;

        let moves: Vec<Move> = moves
            .iter()
            .map(|m| {
                if m.flavor_text_entries.len() > 0 {
                    Move {
                        name: m.name.clone(),
                        description: m
                            .flavor_text_entries
                            .iter()
                            .filter(|m| m.language.name.contains("en"))
                            .map(|m| m.flavor_text.replace("\n", " ").replace("\u{ad} ", ""))
                            .collect::<Vec<String>>()[0]
                            .clone(),
                        type_: m.type_.name.clone(),
                    }
                } else {
                    Move {
                        name: m.name.clone(),
                        description: "None".to_string(),
                        type_: m.type_.name.clone(),
                    }
                }
            })
            .collect();

        Ok(Pokemon {
            name,
            flavor,
            abilities,
            moves,
            types,
        })
    }
}

#[derive(Debug, Clone)]
pub struct Move {
    pub name: String,
    pub description: String,
    pub type_: String,
}

#[derive(Debug, Clone)]
pub struct Ability {
    pub name: String,
    pub description: String,
}
