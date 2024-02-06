use crate::events::*;
use crate::user::*;

use serde::{Deserialize, Serialize};
use core::fmt;
use std::collections::HashMap;

pub struct WorldData {
    pub rooms: HashMap<String, Room>
}

impl WorldData {
    // functions to create and remove rooms
    pub fn add_room(&mut self, name: String, room: Room) {
        self.rooms.insert(name, room);
    }
}

pub struct Room {
    pub name: String,
    pub description: String,
    pub exits: Vec<Direction>,
    pub items: Vec<Item>,
    pub entities: Vec<Entity>
}

impl Room {
    pub fn add_entity(&mut self, entity: Entity) {
        self.entities.push(entity);
    }
    /* 
    This is a poor function, as to remove an Entity, this removes all instances of the Entity
    in the defined structures. Will work on creating a more precise function, but this is the
    temporary solution.
    */
    pub fn remove_entity(&mut self, entity: &Entity) {
        self.entities.retain(|entity| entity.name != entity.name);
    }

    pub fn add_item(&mut self, item: Item) {
        self.items.push(item);
    }
    /*
    Similar to remove_entity, this removes all items under a certain name from the room.
    Ideally, will be changed.
    */
    pub fn remove_item(&mut self, item: Item) {
        self.items.retain(|item| item.name != item.name)
    }
}

pub enum Direction {
    North,
    East,
    South,
    West
}

pub struct Entity {
    pub name: String,
    pub health: i32,
    pub damage: i32,
    pub killable: bool,
    pub dialogue: String
}

impl Entity {
    pub fn interact(&self, user_name: &str) -> String {
        format!("{} interacted with {}", user_name, self.name)
    }
    pub fn attack(&self, user_name: &str) -> String {
        format!("{} attacked {}.", user_name, self.name)
    }
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Item {
    pub name: String,
    pub description: String,
    //will likely change "benefit" to be not a string, but rather hold another type of object,
    //so that the benefits can be quantified better (more inventory space, more damage, etc.)
    pub benefit: String,
    pub weight: f32,
    pub pickup: bool
}

/*
Super fancy implementation for item that allows automatic formatting whenever Item is called to be printed.
Quite ideal to print out the inventory effectively.
*/
impl fmt::Display for Item {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}, Benefit: {}, Weight: {}, Pickup: {}", self.name, self.description, self.benefit, self.weight, self.pickup)
    }
}