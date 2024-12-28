extern crate rust_world;
use rust_world::server;

fn main() {
    let mut server = server::Server {
        ip_addr: String::from("127.0.0.1:7878"),
    };

    let _listener = server.listen(); 

    loop {
    }
}