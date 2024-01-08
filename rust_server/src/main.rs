use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
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
                let response = if received_data.trim() == "new" {
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
