# zkhn-rust-api
![](https://img.shields.io/badge/made_by_cryptograthor-black?style=flat&logo=undertale&logoColor=hotpink)
![](https://github.com/thor314/zkhn-rust-api/actions/workflows/ci.yml/badge.svg)
<!-- [![crates.io](https://img.shields.io/crates/v/zkhn-rust-api.svg)](https://crates.io/crates/zkhn-rust-api) -->
<!-- [![Documentation](https://docs.rs/zkhn-rust-api/badge.svg)](https://docs.rs/zkhn-rust-api) -->

## Run locally
You will need Rust, `cargo-shuttle`, `sqlx-cli`, Docker, and postgres installed.

### install
```sh
### Rust:
# If you do not have rust installed:
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
# If you have not updated rust recently, you may need to run: 
rustup update
# we use the nightly toolchain, specified in rust-toolchain.toml. You should not need to change anything here.
## May need to run
rustup default nightly

### Install cargo-shuttle:
cargo install cargo-binstall # fast-installer for rust binaries
cargo binstall -y cargo-shuttle 
cargo binstall -y sqlx-cli
```

### Postgres installation and setup
Debian:
If `systemctl status postgresql` does not show active, follow steps 1 and 2:
1. Install PostgreSQL
   `sudo apt-get install postgresql`
2. Start the PostgreSQL service
   `sudo service postgresql start`
3. Switch to the PostgreSQL user
   `sudo -i -u postgres`

macOS:
If `brew services info postgresql` does not show active, follow steps 1 and 2:
1. Install PostgreSQL
   - Download and install the PostgreSQL installer from https://www.postgresql.org/download/macosx/
   - Or, if you have Homebrew, run `brew install postgresql`
2. Start the PostgreSQL service
   `brew services start postgresql` (if installed via Homebrew)
   Or, start the service manually if installed via the installer
3. Switch to the PostgreSQL user
   `sudo -u postgres psql`

Common steps for both platforms:

4. Create a new PostgreSQL user (optional)
   `createuser --interactive --pwprompt`
5. Create a new database
   `createdb tk-shuttle-zkhn-rust-api`
6. Grant privileges to your user, replacing `$YOUR_USER`
   ```
   psql tk-shuttle-zkhn-rust-api
   GRANT ALL PRIVILEGES ON DATABASE tk-shuttle-zkhn-rust-api TO $YOUR_USER;
   ```
7. Exit the PostgreSQL prompt `quit` or `ctrl/cmd-d`


### run the server
We should now be able to run the server (yay!)

```sh
cargo shuttle run

# in another terminal:
curl 127.0.0.1:8000/health
# expect: ok

# verify that we can create a user:
curl -X POST \
     -H "Content-Type: application/json" \
     -d '{
           "username": "alice",
           "password": "bob_is_lame",
           "email": "alice@example.com",
           "about": "This is an about string" 
         }' \
     http://localhost:8000/users

# now get the user:
curl -X GET http://localhost:8000/users/alice

# expect:
# {"username":"alice", ... }
```

If `cargo shuttle run` gives an error about docker on MacOS, run `brew install --cask docker`, open Docker (GUI) in /Applications, check that docker is running via `docker info`.

See `api/tests` for usage.

## Deploy to Shuttle
```sh
cargo shuttle project start # only needed the first time
cargo shuttle deploy
```

## License
Licensed under your option of either:
- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

## Contribution
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
