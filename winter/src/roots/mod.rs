use std::io::{self, BufRead};
use std::net::TcpStream;
use std::path::Path;

use ssh2::Session;
use anyhow::{anyhow, Context, Result};
use chrono::Utc;

const UNPACK: &str = include_str!("../../unpack.ash");
const LETSEN: &str = include_str!("../../letsen.ash");
const SSHPRT: &str = include_str!("../../sshprt.cfg");

// ssh function, also see "cone" as related code...
//  these are loaded from (ASH) script files at compile time:
// unpack is a restore function: unpack a tarball
// letsen is a PKI function: use certbot
// sshprt is setting the SSH port to use
// 

//    the unpack function
pub fn unpackreact(key_path: String, username: String, hostname: String, tcp_address: String) -> Result<()> {
    let tcp = match TcpStream::connect(&tcp_address) {
        Ok(stream) => stream,
        Err(e) => {
            eprintln!("Error: Could not connect to {}: {}", tcp_address, e);
            return Err(anyhow!("TCP connection failed"));
        }
    };

    let mut session = Session::new().context("Failed to create SSH session")?;
    session.set_tcp_stream(tcp);
    session.handshake().context("SSH handshake failed")?;
    session
        .userauth_pubkey_file(
            &username,
            None,
            Path::new(&key_path),
            None,
        )
        .with_context(|| format!("SSH authentication failed for user {}", username))?;

    if !session.authenticated() {
        return Err(anyhow!("SSH authentication failed"));
    }

    let mut channel = session.channel_session().context("Failed to open SSH channel")?;
    let remote_script_path = "/tmp/unpack_script.ash";
    let command = format!(
        "echo '{}' > {path} && ash {path}; EXIT_CODE=$?; rm {path}; exit $EXIT_CODE",
        escape_single_quotes(UNPACK),
        path = remote_script_path
    );

    channel.exec(&command).context("Failed to execute command")?;

    let stdout = channel.stream(0);
    let mut reader = io::BufReader::new(stdout);
    let mut line = String::new();

    while reader.read_line(&mut line)? > 0 {
        let timestamp = Utc::now().to_rfc3339();
        print!("{} {}", timestamp, line);
        line.clear();
    }

    channel.wait_close().context("Failed to close SSH channel")?;

    let exit_status = channel.exit_status().context("Failed to get exit status")?;
    println!("Remote ASH script exited with code {}", exit_status);

    Ok(())
}

fn escape_single_quotes(input: &str) -> String {
    input.replace('\'', r"'\''")
}
