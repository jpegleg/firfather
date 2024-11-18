use std::io::{Read, Write};
use std::fs::File;
use std::thread;
use uuid::Uuid;
use chrono::prelude::*;
use serde::Deserialize;

mod roots;
mod needles;

// targets for winter to manage
#[derive(Deserialize)]
struct Config {
    newconeaddress: String,
    newconerootport: String,
    newkeypath: String,
    currentconeaddress: String,
    currentconerootport: String,
    currentkeypath: String,
}

fn main() -> std::io::Result<()> {
    let mut file = File::open("/etc/winter.toml")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let config: Config = toml::from_str(&contents).unwrap();
    let initu = Utc::now();
    // edit this section to create the logic as to when
    // to execute reactions
    //
    // make decisions then if we need to restore a cone from tarball, for example
    //     roots::unpackreact(key_path, username, hostname, tcp_address)
    // where the values passed are populated either from the winter.toml
    Ok(())
}
