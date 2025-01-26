use rand::Rng;

pub struct NPC {
    name: String,
    description: Option<String>,
    level: u8,
    health: u8,
    damage: u8,
    armor: u8,
    movement: Option<Movement>,
    abilities: Option<Vec<Ability>>,
    interaction: Option<String>,
    loot: Option<String>,
    motive: Option<String>,
}

impl NPC {
    pub fn new(name: String, level: u8) -> NPC {
        NPC{
            name,
            description: None,
            level,
            health: level * 3,
            damage: level,
            armor: 0,
            movement: Some(Movement::Short),
            abilities: None,
            interaction: None,
            loot: None,
            motive: None,
        }
        
    }

    pub fn new_random(name: String) -> NPC {
        // Random int between 1 and 6
        let mut rng = rand::thread_rng();
        let level: u8 = rng.gen_range(1..7);
        let health: u8 = level * 3;
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
}

pub enum Movement {
    Immediate,
    Short,
    Long,
    VeryLong,
    Other(String),
}

pub struct Ability {
    name: String,
    description: Option<String>,
}


pub struct PokemonNPC {
    npc: NPC,
    types: Vec<String>,
    moves: Vec<crate::pokemon::Move>,

}