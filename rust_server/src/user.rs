use crate::events::*;
use crate::worlddata::*;

use std::collections::HashMap;
use std::fmt;

pub struct User {
    pub name: String,
    pub user_id: i32,
    pub password: String,
    pub level: i32,
    // HashMap maps weapon name to damage
    pub weapon: HashMap<String, i32>,
    // HashMap maps armor name to defense
    pub armor: HashMap<String, i32>,
    /* 
    Integer value to define the progress in the game. Starts at 0
    Completing each room will increment by 1, tracking game progress.
    */
    pub completion: i32,
    pub inventory: Vec<Item>
}

impl User {
    pub fn print_inventory(&self) {
        println!("{}'s Inventory:", self.name);
        for item in &self.inventory {
            println!("{}", item);
        }
    }
}