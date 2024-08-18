# Solana Wallet in Rust - Example

<a href="https://github.com/s-damian/rust-solana-wallet">
<img src="https://raw.githubusercontent.com/s-damian/medias/main/technos-logos/solana.webp" alt="Solana Logo" height="100px">
</a>
<a href="https://github.com/s-damian/rust-solana-wallet">
<img src="https://raw.githubusercontent.com/s-damian/medias/main/technos-logos/rust.webp" alt="Rust Logo" height="100px">
</a>

> #Solana #Wallet #Rust #BIP39 #Web3 #Blockchain

> This is an example of a **Solana Wallet** in **Rust** 🦀

![Build](https://github.com/s-damian/rust-solana-wallet/actions/workflows/tests.yml/badge.svg)
![Clippy](https://github.com/s-damian/rust-solana-wallet/actions/workflows/static-analysis.yml/badge.svg)
![License](https://img.shields.io/badge/License-MIT-blue)

This **example of a Solana Wallet** is developed by [Stephen Damian](https://github.com/s-damian)

### This wallet manages:

- **Generate Mnemonic**: Creates a new random BIP39 mnemonic phrase.
- **Seed**: Derives a seed from the mnemonic phrase.
- **Passphrase**: You can optionally use a passphrase.
- **Keypair Generation**: Generates a Solana keypair (public and private key) from the derived seed.
- **Keypair Storage**: Saves the generated keypair to a local JSON file for future use.
- **Key Derivation**: Supports generating multiple keypairs from a single seed by applying BIP44 derivation paths.
- **Send SOL (lamports)**: Send SOL to a recipient address (sign outgoing transaction).
- **Get Balance**: Get balance (in SOL and in lamports) by public key.
- **Public Key Display**: Retrieves and displays the public key from the locally stored keypair.


## Summary of commands

- [generate_seed](#-generate_seed-command): Generates a 12-word BIP39 mnemonic phrase, derives the corresponding seed, saves the keypair, and displays the public key.
- [from_mnemonic](#-from_mnemonic-command): Accepts a user-provided BIP39 mnemonic phrase, derives the corresponding seed, saves the keypair, and displays the public key.
- [send](#-send-command): Send SOL to a recipient address.
- [get_pubkey_balance](#-get_pubkey_balance-command): Get balance by public key.
- [get_pubkey_from_keypair_file](#-get_pubkey_from_keypair_file-command): Displays the public key from a keypair stored in a JSON file.


## Project Overview

Rust Solana Wallet - A lightweight Solana wallet developed in Rust.

**Status**: Under development 🚧

![Img](./img/img-1.png)


## Roadmap

- [x] Initial command-line interface (CLI) implementation.
- [ ] Implement a graphical user interface.
- [ ] Add support for SPL tokens.


## Prerequisites

- **Rust** ```>= 1.75.0```: Ensure Rust is installed on your system. You can install Rust using [Rustup](https://rustup.rs/).
- **Cargo**: Rust's package manager, installed automatically with Rust. Learn more about Cargo [here](https://doc.rust-lang.org/cargo/).


## Setup

- Clone the repository:

```bash
git clone https://github.com/s-damian/rust-solana-wallet.git
```

- Navigate into the project directory:

```bash
cd /[your-path]/rust-solana-wallet
```

- Create your ```.env``` file:

```bash
cp .env.example .env
```


## How to use?

**Keypair storage**:

```generate_seed``` and ```from_mnemonic``` commands writes the generated keypair to the ```[project-directory]/storage/keypair/id.json``` file (```KEYPAIR_PATH``` env var), allowing you to store or utilize the keypair in your Solana applications.

**Multiple keypairs (derivations)**:

If you want to generate several keypairs and several public keys with a single mnemonic phrase, you must set the ```NB_DERIVATIONS``` environment variable to a value greater than ```1```.

Your non-derived keypair will be created in your ```[project-directory]/storage/keypair/id.json``` file (```KEYPAIR_PATH``` env var) JSON file.

And the other keypairs (which will be derived from your seed) will be created in JSON files in your ```[project-directory]/storage/keypair/derived``` directory (```KEYPAIR_DIR``` env var).

### 🌐 generate_seed command:

> Generate and display a random mnemonic phrase.

This command generates a new mnemonic phrase randomly, calculates the corresponding seed, displays the seed, displays the Solana public key, and generates and writes the keypair to the JSON file.

- Command:

```bash
cargo run -- generate_seed
```

**Optional passphrase:** You will be prompted to enter a passphrase (leave blank to not use one).

- Example of result:

```bash
BIP39 Mnemonic (random phrase): shed scorpion manual wheat monster phone winter toe dream kitchen salad column
Seed: 34A0EACFFDF41445C0B7E43C2D730C54F4CD1D8334528F73E3D5F2C2977FAABA7CAD88EBDA6A1F02CE6BB596F04036305A32B96303F93FF864D268539739AFF8
Solana Public Key: FTGJPL5hia749v3jhNWJA7uE2VoVGyofB7BBL2cLwoPc
```

### 🌐 from_mnemonic command:

> Generate and display a mnemonic from a specific phrase.

> Command with arguments: cargo run -- from_mnemonic ```PHRASE```

To generate and display the seed and Solana public key from a specific mnemonic phrase, pass the phrase (12 words or 24 words for examples) as an argument.

This will also generates and writes the keypair to the JSON file.

**Example** with this 12 words: ```shed``` ```scorpion``` ```manual``` ```wheat``` ```monster``` ```phone``` ```winter``` ```toe``` ```dream``` ```kitchen``` ```salad``` ```column```.

- Command:

```bash
cargo run -- from_mnemonic "shed scorpion manual wheat monster phone winter toe dream kitchen salad column"
```

**Optional passphrase:** You will be prompted to enter a passphrase (leave blank to not use one).

- Example of result:

```bash
BIP39 Mnemonic (given phrase): shed scorpion manual wheat monster phone winter toe dream kitchen salad column
Seed: 34A0EACFFDF41445C0B7E43C2D730C54F4CD1D8334528F73E3D5F2C2977FAABA7CAD88EBDA6A1F02CE6BB596F04036305A32B96303F93FF864D268539739AFF8
Solana Public Key: FTGJPL5hia749v3jhNWJA7uE2VoVGyofB7BBL2cLwoPc
```

### 🌐 send command:

> Send SOL (lamports) to a recipient address (sign outgoing transaction).

> Command with arguments: cargo run -- send ```RECIPIENT``` ```AMOUNT```

This command allows you to sign an outgoing transaction from your wallet to a destination address.

**Example** to send ```0.002``` SOL (```2000000``` lamports) to recipient address ```EMLY3VvNZ41yMWyPQy2AiEfJTPpZdzeGNG5zaaq3Lihb```.

- Command:

```bash
cargo run -- send EMLY3VvNZ41yMWyPQy2AiEfJTPpZdzeGNG5zaaq3Lihb 2000000
```

This command will sign the transaction with the keypair which is stored in the file ```[project-directory]/storage/keypair/id.json``` file (```KEYPAIR_PATH``` env var).

- Example of result (when successfully):

```bash
Transaction sent successfully!
```

- Example of result (when it fails):

```bash
Failed to send transaction: ...

```

### 🌐 get_pubkey_balance command:

> Get balance by public key.

> Command with arguments: cargo run -- get_pubkey_balance ```PUBKEY```

This command allows you to see the balance (in SOL and in lamports) of a public address.

**Example** to see the balance of the public address ```EMLY3VvNZ41yMWyPQy2AiEfJTPpZdzeGNG5zaaq3Lihb```.

- Command:

```bash
cargo run -- get_pubkey_balance EMLY3VvNZ41yMWyPQy2AiEfJTPpZdzeGNG5zaaq3Lihb
```

- Example of result:

```bash
Balance: 0.005910000 SOL (5910000 lamports)

```

### 🌐 get_pubkey_from_keypair_file command:

> Retrieve public key from stored keypair.

This is useful for retrieving your Solana public key if you have already generated and stored your keypair locally.

This command reads your JSON keypair file stored, extracts the public key, and displays it.

- Command:

```bash
cargo run -- get_pubkey_from_keypair_file
```

This command reads the keypair stored in ```[project-directory]/storage/keypair/id.json``` file (```KEYPAIR_PATH``` env var).

- Example of result:

```bash
Solana Public Key: FTGJPL5hia749v3jhNWJA7uE2VoVGyofB7BBL2cLwoPc
```


## Environment variables

Environment variables are configured in the ```.env``` file.

You can configure these environment variables:
- ```NB_DERIVATIONS``` (default value: ```1```).
- ```KEYPAIR_PATH``` (default value: ```./storage/keypair/id.json```).
- ```KEYPAIR_DIR``` (default value: ```./storage/keypair/derived```).
  - PS: ```KEYPAIR_DIR``` is only useful if ```NB_DERIVATIONS``` is > ```1```.
- ```RPC_URL``` (default value: ```https://api.testnet.solana.com```).


## Some interesting links

- **BIP39**: Learn more about the **BIP39** standard [here](https://github.com/bitcoin/bips/blob/master/bip-0039.mediawiki).
- **SLIP44**: Learn more about the **SLIP44** [here](https://github.com/satoshilabs/slips/blob/master/slip-0044.md).


## Security

This wallet is an example and should not be used to store large amounts of SOL without a thorough security review. Always make sure to back up your mnemonic phrases and private keys in a secure location.


## FAQ

Q: Can this wallet be used on the Solana mainnet?

A: While technically possible, this wallet is designed as an educational example and is not recommended for use on the mainnet without a thorough security review.


## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for more details.
