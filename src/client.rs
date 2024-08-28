
use tokio::net::TcpStream;
use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
use std::io::stdin;

#[tokio::main]
async fn main() -> io::Result<()> {

    tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;

    // Connect to a server
    let connected = TcpStream::connect("127.0.0.1:7878").await;

    match connected {
        Ok(_) => {
            println!("Connected to server");
        }
        Err(e) => {
            println!("Error occurred: {:?}", e);
            return Ok(());
        }
    }

    let mut stream = connected.unwrap();
    loop
    {
        let mut buffer = String::new();
        stdin().read_line(&mut buffer).unwrap();

        stream.write_all(buffer.as_bytes()).await?;
        println!("Sent Hello, awaiting reply...");

        let mut buffer = [0; 512];
        let n = stream.read(&mut buffer).await?;
        println!("Received: {}", String::from_utf8_lossy(&buffer[..n]));
    }
    
}