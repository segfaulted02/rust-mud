mod worlddata;

use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use std::collections::HashMap;

struct UserData {
    users: HashMap<String, String>
}

impl UserData {
    fn new() -> Self {
        UserData {
            users: HashMap::new()
        }
    }
    fn addUser(&mut self, username: &str, password: &str) {
        self.users.insert(username.to_string(), password.to_string());
    }

    fn validate(&self, username: &str, password: &str) -> bool {
        if let Some(stored_password) = self.users.get(username) {
            return stored_password == password;
        }
        false
    }

    fn login(&self, username: &str, password: &str) -> bool {
        self.validate(username, password)
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut user_data = UserData::new();
    user_data.addUser("admin", "password");

    let listener= TcpListener::bind("127.0.0.1:5555").await?;

    loop {
        let (mut socket, _) = listener.accept().await?;

        tokio::spawn(async move {
            let mut buf = [0; 1024];

            loop {
                let n = match socket.read(&mut buf).await {
                    Ok(n) if n == 0 => return,
                    Ok(n) => n,
                    Err(e) => {
                        eprintln!("error in reading socket; err = {:?}", e);
                        return
                    }
                };
                let received_data = match std::str::from_utf8(&buf[..n]) {
                    Ok(v) => v,
                    Err(e) => {
                        eprint!("Error: {:?}", e);
                        return
                    },
                };
                let _response = if received_data.trim() == "new" {
                    "Starting new game...\n"
                } else {
                    "Invalid input\n"
                };
                if let Err(e) = socket.write_all(&buf[0..n]).await {
                    eprintln!("error in writing to socket; err = {:?}", e);
                    return;
                }
            }
        });
    }
}
