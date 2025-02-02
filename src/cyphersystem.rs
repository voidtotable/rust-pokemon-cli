use crate::Pokemon;
use rand::rngs::ThreadRng;
use rand::seq::SliceRandom;
use rand::Rng;

#[derive(Debug)]
pub struct NPC {
    pub name: String,
    pub description: Option<String>,
    pub level: u8,
    pub health: u8,
    pub damage: u8,
    pub armor: u8,
    pub movement: Option<Movement>,
    pub abilities: Option<Vec<Ability>>,
    pub interaction: Option<String>,
    pub loot: Option<String>,
    pub motive: Option<String>,
}

impl NPC {
    pub fn new(name: String, level: u8) -> NPC {
        NPC {
            name,
            description: None,
            level,
            health: health(level),
            damage: level,
            armor: 0,
            movement: Some(Movement::Short),
            abilities: None,
            interaction: None,
            loot: None,
            motive: None,
        }
    }

    pub fn new_random(name: String, mut rng: ThreadRng) -> NPC {
        // Random int between 1 and 6
        let level: u8 = rng.gen_range(1..7);
        let health: u8 = health(level);
        let damage = level;
        // Random int between 0 and 2
        let armor: u8 = rng.gen_range(0..3);
        let movement = Some(Movement::Short);

        NPC {
            name,
            description: None,
            level,
            health,
            damage,
            armor,
            movement,
            abilities: None,
            interaction: None,
            loot: None,
            motive: None,
        }
    }

    /// Updates the level and recalculates health
    pub fn level(&mut self, level: u8) {
        self.level = level;
        self.health = health(level);
    }
}

// Calculates health based on level * 3
fn health(level: u8) -> u8 {
    level * 3
}

#[derive(Debug)]
pub enum Movement {
    Immediate,
    Short,
    Long,
    VeryLong,
    Other(String),
}

#[derive(Debug)]
pub struct Ability {
    name: String,
    description: Option<String>,
}

#[derive(Debug)]
pub struct PokemonNPC {
    pub npc: NPC,
    pub flavor: Vec<String>,
    pub types: Vec<String>,
    pub abilities: Vec<crate::pokemon::Ability>,
    pub moves: Vec<crate::pokemon::Move>,
}

impl PokemonNPC {
    pub fn new_from_pokemon(pokemon: Pokemon) -> Self {
        let npc = NPC::new(pokemon.name, 1);

        PokemonNPC {
            npc,
            flavor: pokemon.flavor,
            types: pokemon.types,
            abilities: pokemon.abilities,
            moves: pokemon.moves,
        }
    }

    /// Randomly prunes flavor to a single flavor, and moves to the specified max
    pub fn prune(&mut self, max_moves: u8, mut rng: ThreadRng) {
        // Choose moves
        let max_moves = usize::from(max_moves);
        if max_moves <= self.moves.len() {
            self.moves = self
                .moves
                .choose_multiple(&mut rng, max_moves)
                .cloned()
                .collect();
        }

        // Choose flavor
        if let Some(flavor) = self.flavor.choose(&mut rng) {
            self.flavor = vec![flavor.to_string()];
        }
    }

    /// Prunes PokemonNPC based the set NPC level.
    pub fn prune_by_level(&mut self, rng: ThreadRng) {
        self.prune(self.npc.level, rng);
    }

    /// Returns the first flavor text, or an empty string if that does not exist
    pub fn flavor(self) -> String {
        if let Some(flavor) = self.flavor.iter().next() {
            return flavor.to_string();
        } else {
            return "".to_string();
        }
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use rustemon::client::{
        CACacheManager, CacheMode, CacheOptions, Environment, RustemonClient, RustemonClientBuilder,
    };

    use super::*;

    fn test_client() -> RustemonClient {
        return RustemonClientBuilder::default()
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
    }

    fn test_rng() -> ThreadRng {
        return rand::thread_rng();
    }

    #[tokio::test]
    async fn it_should_create_pokemonnpc_from_pokemon() {
        let client = test_client();

        let name = "bulbasaur".to_string();
        match Pokemon::new(&name, &client).await {
            Ok(pokemon) => {
                let npc = PokemonNPC::new_from_pokemon(pokemon);
                println!("{:#?}", npc);
            }
            Err(e) => {
                eprintln!("{:#?}", e);
                panic!("Error creating Pokemon")
            }
        }
    }

    #[tokio::test]
    async fn it_should_prune() {
        let client = test_client();
        let rng = test_rng();

        let name = "bulbasaur".to_string();
        match Pokemon::new(&name, &client).await {
            Ok(pokemon) => {
                let mut npc = PokemonNPC::new_from_pokemon(pokemon);
                let max_moves = 2;
                
                npc.prune(max_moves, rng);
                assert_eq!(npc.moves.len(), <usize>::from(max_moves));
                assert_eq!(npc.flavor.len(), 1);
                println!("{:#?}", npc);

            }
            Err(e) => {
                eprintln!("{:#?}", e);
                panic!("Error creating Pokemon")
            }
        }
    }

    #[tokio::test]
    async fn it_should_prune_by_level() {
        let client = test_client();
        let rng = test_rng();

        let name = "bulbasaur".to_string();
        match Pokemon::new(&name, &client).await {
            Ok(pokemon) => {
                let mut npc = PokemonNPC::new_from_pokemon(pokemon);
                
                npc.prune_by_level(rng);
                assert_eq!(npc.moves.len(), 1);
                assert_eq!(npc.flavor.len(), 1);
                println!("{:#?}", npc);

            }
            Err(e) => {
                eprintln!("{:#?}", e);
                panic!("Error creating Pokemon")
            }
        }
    }

    #[tokio::test]
    async fn it_should_not_prune_moves() {
        let client = test_client();
        let rng = test_rng();

        let name = "bulbasaur".to_string();
        match Pokemon::new(&name, &client).await {
            Ok(pokemon) => {
                let mut npc = PokemonNPC::new_from_pokemon(pokemon);
                if let Ok(max_moves) = u8::try_from(npc.moves.len() + 1) {
                    npc.prune(max_moves, rng);
                    assert_ne!(npc.moves.len(), <usize>::from(max_moves));
                    assert_eq!(npc.flavor.len(), 1);
                    println!("{:#?}", npc);
                } else {
                    panic!("Error converting usize to u8")
                }
                

            }
            Err(e) => {
                eprintln!("{:#?}", e);
                panic!("Error creating Pokemon")
            }
        }
    }
    
}
