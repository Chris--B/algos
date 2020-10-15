## Algorithms


### Overview

This is my test bed for algorithms that are interesting to implement. I am reading through Steven S. Skiena's *The Algorithm Design Manual* and working out some of the problems or algorithms here.

### Building

All of the things here are written in Rust and use Cargo to build.
Setup Rust by following the instructions here: https://rustup.rs/

```bash
cargo build # build the code
cargo test # run the unit tests
cargo bench # run the bench marks. This produces graphs that may be of interest.
```

`watch.sh` is a convenient helper script that runs [`cargo-watch`](https://crates.io/crates/cargo-watch) how I like it.

Happy hacking. 😊
