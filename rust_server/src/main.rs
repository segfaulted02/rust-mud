mod worlddata;
mod user;
mod events;

use crate::user::*;
use crate::worlddata::*;
use crate::events::*;

use std::collections::HashMap;
use std::error::Error;
use tokio::fs::File;
use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use serde::{Serialize, Deserialize};

#[derive(Clone)]
pub struct UserData {
    // Maps username to User object
    pub users: HashMap<String, User>
}

impl UserData {
    fn new() -> Self {
        UserData {
            users: HashMap::new()
        }
    }
    fn add_user(&mut self, user: User) {
        self.users.insert(user.name.clone(), user);
    }

    fn validate(&self, name: &str, password: &str) -> bool {
        if let Some(user) = self.users.get(name) {
            return user.password == password;
        }
        false
    }

    // asynchronous because file I/O operations are blocking
    async fn load_user(&self, username: &str) -> Result<User, Box<dyn Error>> {
        let file_path = format!("./user_data/{}.json", username);
        let mut file = File::open(file_path).await?;

        let mut contents = String::new();
        file.read_to_string(&mut contents).await?;

        let user: User = serde_json::from_str(&contents)?;

        Ok(user)
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut user_data: UserData = UserData::new();

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
                            "command" => {
                                // handle user commands, such as "interact", "attack", etc.
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
