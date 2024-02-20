mod worlddata;
mod user;
mod events;
mod actions;

use crate::user::*;
use crate::worlddata::*;
use crate::events::*;
use crate::actions::*;

use std::collections::HashMap;
use std::error::Error;
use tokio::fs::File;
use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use serde::{Serialize, Deserialize};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut user_data: UserData = UserData::new();
    let mut world_data: WorldData = WorldData::new();

    user_data.add_user(User {
        name: "admin".to_string(),
        user_id: 1,
        password: "password".to_string(),
        level: 1,
        weapon: HashMap::new(),
        armor: HashMap::new(),
        completion: 0,
        inventory: Vec::new()
    });

    match user_data.load_user("username").await {
        Ok(user) => {
            println!("Loaded user profile {:?} successfully.", user);
        },
        Err(e) => println!("Failed to load user profile {}", e)
    }

    let listener: TcpListener= TcpListener::bind("127.0.0.1:5555").await?;

    loop {
        let (mut socket, _) = listener.accept().await?;

        let user_data_clone = user_data.clone();
        let mut world_data_clone = world_data.clone();

        tokio::spawn(async move {
            let mut buf: [u8; 1024] = [0; 1024];

            let mut logged_in_user: Option<String> = None;

            loop {
                match socket.read(&mut buf).await {
                    Ok(0) => return, //connection closed
                    Ok(n) => {
                        let received_data = String::from_utf8_lossy(&buf[..n]).to_string();
                        let mut parts = received_data.trim().split_whitespace();
                        let command = parts.next().unwrap_or("");
                        let args: Vec<&str> = parts.collect();

                        match command {
                            "login" if args.len() == 2 => {
                                let username = args[0];
                                let password = args[1];
                                if user_data_clone.validate(username, password) {
                                    logged_in_user = Some(username.to_string());
                                    let response = "Login successful.";
                                    socket.write_all(response.as_bytes()).await.unwrap();
                                } else {
                                    let response = "Invalid username or password.";
                                    socket.write_all(response.as_bytes()).await.unwrap();
                                }
                            },
                            "attack" if !args.is_empty() => {
                                let entity_name = args[0];
                                if let Some(username) = &logged_in_user {
                                    if let Some(user) = user_data_clone.get_user(username) {
                                        let room_id = derive_room_id_from_user(user);
                                        match perform_attack(Some(username.clone()), entity_name, &mut world_data_clone, &user_data_clone) {
                                            Ok(_) => {
                                                let response = "Attack successful.";
                                                socket.write_all(response.as_bytes()).await.unwrap();
                                            },
                                            Err(e) => {
                                                let response = format!("Attack failed: {}", e);
                                                socket.write_all(response.as_bytes()).await.unwrap();
                                            }
                                        }
                                    } else {
                                        let response = "User not found.";
                                        socket.write_all(response.as_bytes()).await.unwrap();
                                    }
                                } else {
                                    let response = "You must be logged in to attack.";
                                    socket.write_all(response.as_bytes()).await.unwrap();
                                }
                            },
                            _ => {
                                let response = "Unknown command.";
                                socket.write_all(response.as_bytes()).await.unwrap();
                            }
                        }
                    },
                    Err(e) => {
                        eprintln!("Error: {:?}", e);
                        return;
                    },
                }
            }
        });
    }
}
