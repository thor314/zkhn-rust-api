# zkhn-rust-api
![](https://img.shields.io/badge/made_by_cryptograthor-black?style=flat&logo=undertale&logoColor=hotpink)
![](https://github.com/thor314/zkhn-rust-api/actions/workflows/ci.yml/badge.svg)
<!-- [![crates.io](https://img.shields.io/crates/v/zkhn-rust-api.svg)](https://crates.io/crates/zkhn-rust-api) -->
<!-- [![Documentation](https://docs.rs/zkhn-rust-api/badge.svg)](https://docs.rs/zkhn-rust-api) -->

## Run locally
You will need Rust and `cargo-shuttle` installed.

### install
```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
cargo install cargo-binstall # fast-installer for rust binaries
cargo-binstall -y cargo-shuttle 
```

### run the server
```sh
# ensure the database is correctly set up
sudo -u postgres psql postgres://postgres:postgres@localhost:17360

# from the psql prompt
> CREATE DATABASE "tk-shuttle-zkhn-rust-api";
> exit;

cargo shuttle run
# in another terminal:
curl 127.0.0.1:8000/health
# see: ok
```

See `api/tests` for examples.

## Deploy to Shuttle
```sh
# run locally
cargo shuttle run
# deploy
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
