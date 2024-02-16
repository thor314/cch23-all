# cch23-all
![](https://img.shields.io/badge/made_by_cryptograthor-black?style=flat&logo=undertale&logoColor=hotpink)
![](https://github.com/thor314/cch23-all/actions/workflows/ci.yml/badge.svg)
<!-- [![crates.io](https://img.shields.io/crates/v/cch23-all.svg)](https://crates.io/crates/cch23-all) -->
<!-- [![Documentation](https://docs.rs/cch23-all/badge.svg)](https://docs.rs/cch23-all) -->
## License
Licensed under your option of either:
- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

## Contribution
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
## Deploy
```sh
# run locally
cargo shuttle run
# deploy
cargo shuttle project start # only needed the first time
cargo shuttle deploy
```

## SQL is migrations are not pure
if db issues arise:
```fish
set DATABASE_URL postgres://postgres:postgres@localhost:18607/postgres
sqlx migrate run
cargo sqlx prepare # check migrations have been correctly run
```

## Project created with flags:
- project-name: cch23-all
- description:  cch23-all
- authors:      Thor Kampefner <thorck@pm.me>
- crate_name:   cch23_all
- crate_type:   bin
- os-arch:      linux-x86_64
- username:     Thor Kampefner
- within_cargo: false
- is_init:      false
- now:          2024-02-16
- bin or lib:   bin 
- advanced:     advanced 
- cli:         
- license:      license 
- ci:           ci 
- itests:      
- benches:     
- async:        async 
- server:       server 
