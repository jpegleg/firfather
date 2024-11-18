use std::io::{self, BufRead};
use std::net::TcpStream;
use std::path::Path;

use ssh2::Session;
use anyhow::{anyhow, Context, Result};
use shellexpand::tilde;
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
