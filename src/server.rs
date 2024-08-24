mod session_manager;
use std::sync::{Arc};
use tokio::net::{TcpListener};
use tokio::sync::Mutex;
use crate::session_manager::SessionManager;

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").await.unwrap();
    println!("Server listening on port 7878");

    let manager = Arc::new(Mutex::new(SessionManager::new()));
    
    loop {
        match listener.accept().await {
            Ok((stream, _)) => {    
                println!("New client connected");
                SessionManager::add_session(manager.clone(), stream).await;
            }
            Err(e) => {
                println!("Error occurred: {:?}", e);
            }
        }
    }
}
