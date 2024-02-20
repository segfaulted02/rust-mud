use crate::events::*;
use crate::user::*;

use serde::{Deserialize, Serialize};
use core::fmt;
use std::collections::HashMap;
use std::fmt::Display;

#[derive(Clone)]
pub struct WorldData {
    pub rooms: HashMap<String, Room>
}

impl WorldData {
    pub fn new() -> Self {
        Self { rooms: HashMap::new() }
    }
    // functions to create and remove rooms
    pub fn add_room(&mut self, name: String, room: Room) {
        self.rooms.insert(name, room);
    }
    pub fn get_room_by_completion(&mut self, room_id: &str) -> Option<&mut Room> {
        self.rooms.get_mut(room_id)
    }
}

#[derive(Clone)]
pub struct Room {
    pub name: String,
    pub description: String,
    pub exits: Vec<Direction>,
    pub items: Vec<Item>,
    pub entities: Vec<Entity>,
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

#[derive(Clone)]
pub enum Direction {
    North,
    East,
    South,
    West
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Entity {
    pub name: String,
    pub health: i32,
    pub damage: i32,
    pub killable: bool,
    pub dialogue: String
}

impl Entity {
    pub fn new(name: &str, health: i32, damage: i32, killable: bool, dialogue: &str) -> Self {
        Self {
            name: name.to_string(),
            health,
            damage,
            killable,
            dialogue: dialogue.to_string(),
        }
    }

    pub fn interact(&self, target: &mut Entity) -> String {
        format!("{:?} interacted with {}", target, self.name)
    }

    pub fn attack(&mut self, target: &mut Entity) -> Result<(), String> {
        if !self.killable {
            return Err(format!("{} cannot be attacked", self.name));
        }

        println!("{} attacks {} for {} damage.", self.name, target.name, self.damage);
        target.health -= self.damage;

        if target.health <= 0 && target.killable {
            return Err(format!("{} has been defeated!", target.name));
        } else {
            return Err(format!("Unknown error"));
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Item {
    pub name: String,
    pub description: String,
    pub item_type: ItemType,
    pub value: i32,
    pub weight: f32,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum ItemType {
    Weapon,
    Armor,
    Other,
}

/*
Super fancy implementation for item that allows automatic formatting whenever Item is called to be printed.
Quite ideal to print out the inventory effectively.
*/
impl fmt::Display for Item {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}, Type: {:?}, Value: {}, Weight: {}", self.name, self.description, self.item_type, self.value, self.weight)
    }
}