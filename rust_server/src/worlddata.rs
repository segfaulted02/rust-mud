use std::collections::HashMap;

struct WorldData {
    rooms: HashMap<String, Room>
}

impl WorldData {
    // will be static, holding the available rooms
    // function to manipulate room data (picking up item, killing monster, etc)
}

struct Room {
    name: String,
    description: String,
    exits: HashMap<Direction, String>,
    items: HashMap<Item, String>,
    enemies: HashMap<Entity, String>
}

enum Direction {
    North,
    East,
    South,
    West
}

enum Entity {
    Monster {
        name: String,
        health: i32,
        damage: i32,
        killable: bool,
    },
    Npc {
        name: String,
        health: i32,
        damage: i32,
        killable: bool,
        dialogue: String
    }
}

enum Item {
    Object {
        name: String,
        description: String,
        //will likely change "benefit" to be not a string, but rather hold another type of object,
        //so that the benefits can be quantified better (more inventory space, more damage, etc.)
        benefit: String,
        weight: i32,
        pickup: bool
    }
}