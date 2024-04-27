use crate::server::Server;

mod server;

fn main() {
    let server = Server::new("box1".to_owned());
    println!("server: {:?}", server);
}
