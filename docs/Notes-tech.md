## Networks

| Name         | Type              | Environment      |
|--------------|-------------------|------------------|
| **Devnet**   | Public devnet     | For development  |
| **Testnet**  | Public testnet    | For last testing |
| **Mainnet**  | Production        | For production   |



## Lint

Format the code:

```bash
cargo fmt
```

Lint:

```bash
cargo clippy
```

Lint (with warnings):

```bash
cargo clippy --all-targets
```

### With `Makefile.toml` :

```bash
cargo make lint-all
```



## Run tests

Create your ```.env.testing``` file:

```bash
cp .env.testing.example .env.testing
```

Run the tests :

```bash
cargo test
```

Run tests by capturing and displaying output :

```bash
cargo test -- --nocapture
```



## Run / Build / Release

### Run:

Compile (if necessary) and immediately execute the program:

```bash
cargo run <arguments>
```


### Build - Debug:

Compile a debug version:

```bash
cargo build
```

> This command will create the compiled file: `target/debug/rust_solana_wallet`


### Build - Release:

Compile a release version for production:

```bash
cargo build --release
```

> This command will create the compiled file: `target/release/rust_solana_wallet`



## Using the Release

This example demonstrates how to create a compiled version and use it on your Linux Desktop.

- Navigate to the project directory:

```bash
cd /<your-path>/rust-solana-wallet
```

- Compile to create a release:

```bash
cargo build --release
```

- Copy the release file to your Desktop:

```bash
cp ./target/release/rust_solana_wallet ~/Desktop
```

- Configure your .`env` file, then copy it to your Desktop:

```bash
cp ./.env ~/Desktop
```

- Navigate to your Desktop:

```bash
cd ~/Desktop
```

- Now you can use the Wallet release. For example, to generate a seed:

```bash
./rust_solana_wallet generate_seed
```
