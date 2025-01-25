use futures::stream::FuturesUnordered;
use futures::StreamExt;
use rand::Rng;
use rustemon::client::RustemonClient;
use rustemon::error::Error;
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
pub struct StatBlock {
    name: String,
    flavor: String,
    abilities: Vec<Ability>,
    moves: Vec<Move>,
    level: u8,
    hp: u8,
    armor: u8,
}

impl StatBlock {
    pub async fn new_from_meta(meta: &PokemonMeta, c: &RustemonClient) -> Result<StatBlock, Error> {
        let name = meta.name.clone();

        // Random number between 1 and 6
        let mut rng = rand::thread_rng();
        let level: u8 = rng.gen_range(1..7);
        let hp: u8 = level * 3;
        let armor: u8 = 0;

        let pokemon = rustemon::pokemon::pokemon::get_by_name(&meta.name, c).await?;

        // Flavor Text
        let species = pokemon.species.follow(c).await?;
        let max_flavor_texts = species.flavor_text_entries.len();
        let flavor = species.flavor_text_entries[rng.gen_range(0..max_flavor_texts)]
            .flavor_text
            .clone();

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
                description: a.flavor_text_entries[0].flavor_text.clone(),
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
                        description: m.flavor_text_entries[0].flavor_text.clone(),
                    }
                } else {
                    Move {
                        name: m.name.clone(),
                        description: "None".to_string(),
                    }
                }
            })
            .collect();

        Ok(StatBlock {
            name,
            flavor,
            abilities,
            moves,
            level,
            hp,
            armor,
        })
    }
}

#[derive(Debug)]
pub struct Move {
    name: String,
    description: String,
}

#[derive(Debug)]
pub struct Ability {
    name: String,
    description: String,
}
