use crate::user::*;
use crate::worlddata::*;
use crate::events::*;

use std::collections::HashMap;
use std::error::Error;
use tokio::fs::File;
use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use serde::{Serialize, Deserialize};

/*pub(crate) fn perform_attack(attacker_name: &str, target_name: &str, entities: &mut Vec<Entity>) -> Result<(), String> {
    let attacker_index = entities.iter().position(|e| e.name == attacker_name)
        .ok_or(format!("Attacker {} not found.", attacker_name))?;

    let target_index = entities.iter().position(|e| e.name == target_name && e.killable)
        .ok_or(format!("Target {} not found or is not killable.", target_name))?;

    // ensuring the attacker and target are not the same entity
    if attacker_index == target_index {
        return Err("Attacker and target cannot be the same.".to_string());
    }

    // borrowing the entities at different indices
    let (smaller_index, larger_index) = if attacker_index < target_index { (attacker_index, target_index) } else { (target_index, attacker_index) };
    let (first_entity, second_entity) = entities.split_at_mut(larger_index);
    let attacker = &mut first_entity[smaller_index];
    let target = &mut second_entity[0];

    attacker.attack(target)
}*/
pub(crate) fn perform_attack(
    user_name: Option<String>,
    entity_name: &str,
    world_data: &mut WorldData,
    user_data: &UserData,
) -> Result<(), String> {
    if let Some(username) = user_name {
        if let Some(user) = user_data.get_user(&username) {
            // Assuming you have a way to map the user's completion or another attribute to a room's unique identifier
            let room_id = derive_room_id_from_user(user); // You need to implement this function

            if let Some(room) = world_data.get_room_by_completion(&room_id) {
                // Attempt to find and interact with the entity within the room
                if let Some(target_index) = room.entities.iter().position(|e| e.name == entity_name) {
                    // Implement attack logic here
                    println!("User {} attacks entity {} in room '{}'", username, entity_name, room.name);
                    // Example action: remove the entity if defeated
                    room.entities.remove(target_index);
                    return Ok(());
                } else {
                    return Err(format!("Entity {} not found in the current room.", entity_name));
                }
            } else {
                return Err(format!("Room '{}' not found.", room_id));
            }
        } else {
            return Err("User not found.".to_string());
        }
    } else {
        return Err("User must be logged in to perform an attack.".to_string());
    }
}
pub fn derive_room_id_from_user(user: &User) -> String {
    format!("room_{}", user.completion)
}