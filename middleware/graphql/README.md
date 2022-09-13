## Rust GraphQL Tutorial

Learning how to create a GraphQL server in Rust.

Creates 2 binaries, one to run locally for debugging and another to deploy to AWS as a lambda.

To start the local server, run...

```shell
make run_local_graph
```

To deploy the graph as a lambda, run...

```shell
make release.package.deploy
```

..this will cross compile to linux first.

You will need to install the `musl` libc library and the `rust-std-x86_64-unknown-linux-musl` rustup component.

NOTES - some nodes on setting up a MAC to cross compile to linux

```shell
brew install FiloSottile/musl-cross/musl-cross
```

Compile using this command

```shell
TARGET_CC=x86_64-linux-musl-gcc RUSTFLAGS="-C linker=x86_64-linux-musl-gcc" cargo build --target=x86_64-unknown-linux-musl
```

Or create a config file under .cargo/config

```shell
[target.x86_64-unknown-linux-musl]
linker = "x86_64-linux-musl-gcc"
```

Then you can just run `cargo build --target=x86_64-unknown-linux-musl`
