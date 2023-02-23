# wk - a Rust Pomodoro timer CLI

This is a Rust self-teaching project and at present should not be relied upon.

# History:

## February 23, 2023
It was unclear at first how to get [this clap quickstart](https://docs.rs/clap/latest/clap/_derive/_tutorial/index.html#quick-start) to build.  The hint was in " This requires enabling the derive feature flag."  It turns out the easiest way to do this is as follows:

```
cargo add clap --features derive
```

This adds the correct dependencyt line to Cargo.toml:

```
[dependencies]
clap = { version = "4.1.6", features = ["derive"] }
```