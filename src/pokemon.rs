use futures::stream::FuturesUnordered;
use futures::StreamExt;
use itertools::Itertools;
use rand::Rng;
use rustemon::client::RustemonClient;
use rustemon::error::Error;
use rustemon::model::pokemon::Pokemon;
use rustemon::model::resource::FlavorText;
use rustemon::Follow;

#[derive(Debug)]
pub struct PokemonTable(Vec<PokemonMeta>);

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
                    PokemonMeta {
                        name: pokemon.name,
                        types,
                        abilities,
                        moves,
                    }
                });
                let mut stream = FuturesUnordered::from_iter(futures);

                let mut rows: Vec<PokemonMeta> = Vec::new();
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
pub struct PokemonMeta {
    name: String,
    types: Vec<String>,
    abilities: Vec<String>,
    moves: Vec<String>,
}

impl PokemonMeta {
    pub async fn new(name: &String, c: &RustemonClient) -> Result<PokemonMeta, Error> {
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

                Ok(PokemonMeta {
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

#[derive(Debug)]
pub struct PokemonDetails {
    name: String,
    flavor: Vec<String>,
    types: Vec<String>,
    abilities: Vec<Ability>,
    moves: Vec<Move>,
}

impl PokemonDetails {
    pub async fn new(name: &String, c: &RustemonClient) -> Result<PokemonDetails, Error> {
        let meta = PokemonMeta::new(name, c).await?;

        Self::new_from_meta(&meta, c).await
    }

    pub async fn new_from_meta(
        meta: &PokemonMeta,
        c: &RustemonClient,
    ) -> Result<PokemonDetails, Error> {
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
        let futures = meta.abilities.iter().map(|ability| async {
            rustemon::pokemon::ability::get_by_name(ability, c)
                .await
                .unwrap()
        });
        let mut stream = FuturesUnordered::from_iter(futures);

        let mut abilities: Vec<rustemon::model::pokemon::Ability> = Vec::new();
        while let Some(ability) = stream.next().await {
            abilities.push(ability);
        }

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
            .map(|move_| async { rustemon::moves::move_::get_by_name(move_, c).await.unwrap() });
        let mut stream = FuturesUnordered::from_iter(futures);

        let mut moves: Vec<rustemon::model::moves::Move> = Vec::new();
        while let Some(move_) = stream.next().await {
            moves.push(move_);
        }

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

        Ok(PokemonDetails {
            name,
            flavor,
            abilities,
            moves,
            types,
        })
    }
}

#[derive(Debug)]
pub struct Move {
    name: String,
    description: String,
    type_: String,
}

#[derive(Debug)]
pub struct Ability {
    name: String,
    description: String,
}
