use std::io;
use time_protocol::time_server::Server;

fn main() -> io::Result<()> {
    let server = Server::default();
    server.start()?;
    Ok(())
}
