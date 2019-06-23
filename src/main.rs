#![allow(clippy::unreadable_literal)]
//! This is an Apple Music developer token generator, written in Rust.  
//! It generates the JSON Web Token, given a team ID, a key ID and its associated private key file.
//! 
//! Usage
//! -----
//! 
//! ```plaintext
//! USAGE:
//!     apple-music-jwt [OPTIONS] <PRIVATE_KEY> --key-id <key_id> --team-id <team_id>
//! 
//! FLAGS:
//!     -h, --help       Prints help information
//!     -V, --version    Prints version information
//! 
//! OPTIONS:
//!     -e, --expires-in <expires_in>    The duration for which the token must be valid (exp).
//!     -k, --key-id <key_id>            The Key ID to use (kid).
//!     -t, --team-id <team_id>          The Team ID to use (iss).
//! 
//! ARGS:
//!     <PRIVATE_KEY>    The path to the private key to use.
//! ```

extern crate frank_jwt as jwt;

use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

use chrono::{Duration, Utc};
use serde_json::json;
use structopt::StructOpt;

pub mod error;

use error::Error;

#[derive(Debug, Clone, PartialEq, StructOpt)]
/// An Apple Music developer token generator.  
/// <https://developer.apple.com/documentation/applemusicapi/getting_keys_and_creating_tokens>
pub struct Config {
    #[structopt(short = "t", long = "team-id")]
    /// The Team ID to use (iss).
    pub team_id: String,

    #[structopt(short = "k", long = "key-id")]
    /// The Key ID to use (kid).
    pub key_id: String,

    #[structopt(short = "e", long = "expires-in")]
    /// The duration for which the token must be valid (exp).
    pub expires_in: Option<humantime::Duration>,

    #[structopt(name = "PRIVATE_KEY")]
    /// The path to the private key to use.
    pub secret_file: PathBuf,
}

fn main() -> Result<(), Error> {
    let config = Config::from_args();

    let exp = config.expires_in.map_or_else(
        || Duration::weeks(23),
        |dur| Duration::from_std(dur.into()).expect("unexpected negative duration"),
    );
    let payload = json!({
        "iss": config.team_id,
        "iat": Utc::now().timestamp(),
        "exp": (Utc::now() + exp).timestamp(),
    });
    let header = json!({
        "alg": "ES256",
        "kid": config.key_id,
    });

    let mut secret = Vec::new();
    File::open(config.secret_file)?.read_to_end(&mut secret)?;

    let token = jwt::encode(header, &secret, &payload, jwt::Algorithm::ES256)?;
    println!("{}", token);

    Ok(())
}
