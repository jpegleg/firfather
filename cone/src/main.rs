use std::io::{self, BufRead};
use std::net::TcpStream;
use std::path::Path;

use ssh2::Session;
use clap::Parser;
use anyhow::{anyhow, Context, Result};
use shellexpand::tilde;
use chrono::Utc;

const ASH_SCRIPT: &str = include_str!("script.ash");

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    target: String,

    /// Path to the SSH private key PEM PKCS8 plain RSA
    #[arg(short = 'k', long, default_value = "~/.ssh/id_rsa")]
    key_path: String,

    /// Port number for SSH (default is 22) but any can be supplied
    #[arg(short, long, default_value_t = 22)]
    port: u16,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let key_path = tilde(&args.key_path).to_string();
    let (username, hostname) = parse_target(&args.target)?;
    let tcp_address = format!("{}:{}", hostname, args.port);
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
    let remote_script_path = "/tmp/embedded_script.ash";
    let command = format!(
        "echo '{}' > {path} && ash {path}; EXIT_CODE=$?; rm {path}; exit $EXIT_CODE",
        escape_single_quotes(ASH_SCRIPT),
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

fn parse_target(target: &str) -> Result<(String, String)> {
    let parts: Vec<&str> = target.split('@').collect();
    if parts.len() != 2 {
        return Err(anyhow!("Invalid target format. Expected user@hostname."));
    }
    Ok((parts[0].to_string(), parts[1].to_string()))
}

fn escape_single_quotes(input: &str) -> String {
    input.replace('\'', r"'\''")
}
