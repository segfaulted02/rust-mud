use crate::worlddata::*;

use serde::{Deserialize, Serialize};
use std::{collections::HashMap, error::Error, fs::File, io::Read};

#[derive(Serialize, Deserialize, Clone, Debug)]
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
    In tandem, also holds the current room.
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

#[derive(Clone)]
pub struct UserData {
    // Maps username to User object
    pub users: HashMap<String, User>
}

impl UserData {
    pub fn new() -> Self {
        UserData {
            users: HashMap::new()
        }
    }
    pub fn add_user(&mut self, user: User) {
        self.users.insert(user.name.clone(), user);
    }

    pub fn validate(&self, name: &str, password: &str) -> bool {
        if let Some(user) = self.users.get(name) {
            return user.password == password;
        }
        false
    }

    // asynchronous because file I/O operations are blocking
    pub async fn load_user(&self, username: &str) -> Result<User, Box<dyn Error>> {
        let file_path = format!("./user_data/{}.json", username);
        let mut file = File::open(file_path)?;

        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        let user: User = serde_json::from_str(&contents)?;

        Ok(user)
    }
    pub fn get_user(&self, name: &str) -> Option<&User> {
        return self.users.get(name);
    }
}