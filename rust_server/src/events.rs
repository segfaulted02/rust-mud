use crate::user::*;
use crate::worlddata::*;

pub fn first_room(worlddata: &mut WorldData) {
    // Room structure definition for the initial room
    let mut starting_room: Room = Room {
        name: "Beginnings".to_string(),
        description: "You wake up in a damp, cold room. Your mind is foggy, and you are sitting, your back against the wall.
        You slowly stand up, collecting yourself, and notice a rusty sword, with a chipped blade, on the ground.
        You pick it up, wondering, what happened, and how do I get home?".to_string(),
        exits: Vec::new(),
        items: Vec::new(),
        entities: Vec::new()
    };

    let rusty_sword = Item {
        name: "Rusty Sword".to_string(),
        description: "A rusty sword, with a chipped blade.".to_string(),
        item_type: ItemType::Weapon,
        value: 2,
        weight: 5.0,
    };

    let hidden_key = Item {
        name: "Hidden Key".to_string(),
        description: "A small key, appearing to fit the keyhole in the cold, damp room.".to_string(),
        item_type: ItemType::Other,
        value: 0,
        weight: 0.1,
    };

    starting_room.add_item(rusty_sword);
    starting_room.add_item(hidden_key);

    worlddata.add_room(starting_room.name.clone(), starting_room)
}

// More rooms can be added ...