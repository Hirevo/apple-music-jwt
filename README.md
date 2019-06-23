Apple Music JWT
===============

This is an Apple Music developer token generator, written in Rust.  
It generates the JSON Web Token, given a team ID, a key ID and its associated private key file.

Usage
-----

```plaintext
USAGE:
    apple-music-jwt [OPTIONS] <PRIVATE_KEY> --key-id <key_id> --team-id <team_id>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -e, --expires-in <expires_in>    The duration for which the token must be valid (exp).
    -k, --key-id <key_id>            The Key ID to use (kid).
    -t, --team-id <team_id>          The Team ID to use (iss).

ARGS:
    <PRIVATE_KEY>    The path to the private key to use.
```
