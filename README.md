
# Example of a Solana Wallet in Rust

<a href="https://github.com/s-damian/rust-solana-wallet">
<img src="https://raw.githubusercontent.com/s-damian/medias/main/technos-logos/solana-logo.webp" alt="Solana Logo" height="100px">
</a>
<a href="https://github.com/s-damian/rust-solana-wallet">
<img src="https://raw.githubusercontent.com/s-damian/medias/main/technos-logos/rust-logo.webp" alt="Rust Logo" height="100px">
</a>

This project demonstrates how to create and manage a Solana wallet using Rust.

It provides functionalities to generate a new random mnemonic phrase or use a given phrase to generate and display the corresponding Solana seed and public key.



## Summary

* [generate_seed](#-generate_seed-generate-and-display-a-random-mnemonic): Generates a 12-word BIP 39 mnemonic phrase, derives the corresponding seed, saves the keypair, and displays the public key.

* [from_mnemonic](#-from_mnemonic-generate-and-display-a-mnemonic-from-a-specific-phrase): Accepts a user-provided 12-word BIP 39 mnemonic phrase, derives the corresponding seed, saves the keypair, and displays the public key.

* [get_pubkey_from_keypair_file](#-get_pubkey_from_keypair_file-retrieve-public-key-from-stored-keypair): Displays the public key from a keypair stored in a JSON file, which is supplied by the user.



## Requirements

* **Rust**: Ensure Rust is installed on your system. You can install Rust using [rustup](https://rustup.rs/).
* **Cargo**: Rust's package manager, installed automatically with Rust. Learn more about Cargo [here](https://doc.rust-lang.org/cargo/).
* **BIP39 Basic knowledge**: Learn more about the **BIP 39** standard [here](https://github.com/bitcoin/bips/blob/master/bip-0039.mediawiki).



## Setup

Before running the project, make sure to clone the repository and navigate into the project directory:

```
git clone https://github.com/s-damian/rust-solana-wallet.git
```

```
cd /[your-path]/rust-solana-wallet
```

```
cp .env.example .env
```



## How to use?

These operations writes the generated keypair to the ```[project-directory]/storage/keypair/id.json``` file, allowing you to store or utilize the keypair in your Solana applications.


### 🌐 "generate_seed": Generate and Display a Random Mnemonic:

This command generates a new mnemonic phrase randomly, calculates the corresponding seed, displays the seed, displays the Solana public key, and generates and writes the keypair to the file.

* Command:

```
cargo run generate_seed
```

* Example of result:

```
BIP39 Mnemonic (random phrase) : shed scorpion manual wheat monster phone winter toe dream kitchen salad column
BIP39 Seed : 34A0EACFFDF41445C0B7E43C2D730C54F4CD1D8334528F73E3D5F2C2977FAABA7CAD88EBDA6A1F02CE6BB596F04036305A32B96303F93FF864D268539739AFF8
Public Key: FTGJPL5hia749v3jhNWJA7uE2VoVGyofB7BBL2cLwoPc
```



### 🌐 "from_mnemonic": Generate and Display a Mnemonic from a Specific Phrase:

To generate and display the seed and Solana public key from a specific mnemonic phrase, pass the phrase (12 words or 24 words by example) as an argument.

This will also generates and writes the keypair to the file.

**Example** with this 12 words: ```fit``` ```refuse``` ```hotel``` ```collect``` ```tortoise``` ```race``` ```rail``` ```weasel``` ```little``` ```medal``` ```couch``` ```remember```.

* Command:

```
cargo run from_mnemonic "fit refuse hotel collect tortoise race rail weasel little medal couch remember"
```

* Example of result:

```
BIP39 Mnemonic (given phrase) : fit refuse hotel collect tortoise race rail weasel little medal couch remember
BIP39 Seed : 2C9AE93C7FA7D5296472B6E0F8928F94963E96ACAFDF1924AF8B7A8471B04FA15F49C98023FDC84BBB5979085F91A577E1A36A7BAC9C4C735D44379D7A915D59
Public Key: EsiyKK61Ycv4XXqUoFJa2SuFJGHjVeWgAB5UvaNkb713
```

**Note**: The BIP 39 standard includes a predefined list of words used to generate cryptographic keys. Your custom mnemonic phrase must consist of words exclusively from this list to be valid. Using words not in the BIP 39 list will lead to errors in generating a valid seed.


### 🌐 "get_pubkey_from_keypair_file": Retrieve Public Key from Stored Keypair:

This command reads the keypair stored in ```[project-directory]/storage/keypair/id.json```, extracts the public key, and displays it. This is useful for retrieving your Solana public key if you have already generated and stored your keypair locally.

* Command:

```
cargo run get_pubkey_from_keypair_file
```

* Example of result:

```
Public Key: EsiyKK61Ycv4XXqUoFJa2SuFJGHjVeWgAB5UvaNkb713
```



## Show Help

```
cargo run -- --help
```



## Lint and Static Analysis

Maintain code quality and style consistency using the following tools:


### Code Formatting:

To format the codebase according to Rust's coding standards, use:

```
cargo fmt
```


### Static Code Analysis:

To perform static code analysis and catch common mistakes and inefficiencies, use:

```
cargo clippy
```



## Env

You can configure the ```KEYPAIR_PATH``` env variable in the ```.env file``` (default value: ```./storage/keypair/id.json```).
