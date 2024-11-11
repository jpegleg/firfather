use std::io::{self, BufRead};
use std::net::TcpStream;
use std::path::Path;

use ssh2::Session;
use clap::Parser;
use anyhow::{anyhow, Context, Result};
use shellexpand::tilde;
use chrono::Utc;

const UNPACK: &str = include_str!("unpack.ash");
const LETSEN: &str = include_str!("letsen.ash");
const SSHPRT: &str = include_str!("sshprt.cfg");
// reactionary SSH functions below...
