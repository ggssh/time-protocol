use std::io;
use time_protocol::time_client::Client;

fn main() -> io::Result<()> {
    let client = Client::default();
    client.update()?;
    Ok(())
}
