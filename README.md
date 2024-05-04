
# Solana Wallet in Rust - Example

<a href="https://github.com/s-damian/rust-solana-wallet">
<img src="https://raw.githubusercontent.com/s-damian/medias/main/technos-logos/solana.webp" alt="Solana Logo" height="100px">
</a>
<a href="https://github.com/s-damian/rust-solana-wallet">
<img src="https://raw.githubusercontent.com/s-damian/medias/main/technos-logos/rust.webp" alt="Rust Logo" height="100px">
</a>

> This is an example of a **Solana Wallet** in **Rust**.

> This project demonstrates how to create and manage a Solana Wallet using Rust.

* **Generate Mnemonic**: Creates a new random BIP39 mnemonic phrase.
* **Seed**: Derives a seed from the mnemonic phrase.
* **Passphrase**: You can optionally use a passphrase.
* **Keypair Generation**: Generates a Solana keypair (public and private key) from the derived seed.
* **Keypair Storage**: Saves the generated keypair to a local JSON file for future use.
* **Key Derivation**: Supports generating multiple keypairs from a single seed by applying BIP44 derivation paths.
* **Public Key Display**: Retrieves and displays the public key from the locally stored keypair.



## Summary

* [generate_seed](#-generate_seed-operation): Generates a 12-word BIP39 mnemonic phrase, derives the corresponding seed, saves the keypair, and displays the public key.

* [from_mnemonic](#-from_mnemonic-operation): Accepts a user-provided BIP39 mnemonic phrase, derives the corresponding seed, saves the keypair, and displays the public key.

* [get_pubkey_from_keypair_file](#-get_pubkey_from_keypair_file-operation): Displays the public key from a keypair stored in a JSON file.



## Requirements

* **Rust**: Ensure Rust is installed on your system. You can install Rust using [rustup](https://rustup.rs/).
* **Cargo**: Rust's package manager, installed automatically with Rust. Learn more about Cargo [here](https://doc.rust-lang.org/cargo/).



## Setup

* Clone the repository:

```
git clone https://github.com/s-damian/rust-solana-wallet.git
```

* Navigate into the project directory:

```
cd /[your-path]/rust-solana-wallet
```

* Create your ```.env``` file:

```
cp .env.example .env
```



## How to use?

**Keypair storage**:

```generate_seed``` and ```from_mnemonic``` operations writes the generated keypair to the ```[project-directory]/storage/keypair/id.json``` file, allowing you to store or utilize the keypair in your Solana applications.

**Multiple keypairs (derivations)**:

If you want to generate several keypairs and several public keys with a single mnemonic phrase, you must set the ```NB_DERIVATIONS``` environment variable to a value greater than ```1```.

Your non-derived keypair will be created in your ```KEYPAIR_PATH``` JSON file.

And the other keypairs (which will be derived from your seed) will be created in JSON files in your ```KEYPAIR_DIR``` directory.


### 🌐 ```generate_seed``` operation:

> Generate and Display a Random Mnemonic.

This command generates a new mnemonic phrase randomly, calculates the corresponding seed, displays the seed, displays the Solana public key, and generates and writes the keypair to the file.

* Command:

```
cargo run -- generate_seed
```

**Optional passphrase:** You will be prompted to enter a passphrase (leave blank to not use one).

* Example of result:

```
BIP39 Mnemonic (random phrase): shed scorpion manual wheat monster phone winter toe dream kitchen salad column
Seed: 34A0EACFFDF41445C0B7E43C2D730C54F4CD1D8334528F73E3D5F2C2977FAABA7CAD88EBDA6A1F02CE6BB596F04036305A32B96303F93FF864D268539739AFF8
Solana Public Key: FTGJPL5hia749v3jhNWJA7uE2VoVGyofB7BBL2cLwoPc
```



### 🌐 ```from_mnemonic``` operation:

> Generate and Display a Mnemonic from a Specific Phrase.

To generate and display the seed and Solana public key from a specific mnemonic phrase, pass the phrase (12 words or 24 words by example) as an argument.

This will also generates and writes the keypair to the file.

**Example** with this 12 words: ```fit``` ```refuse``` ```hotel``` ```collect``` ```tortoise``` ```race``` ```rail``` ```weasel``` ```little``` ```medal``` ```couch``` ```remember```.

* Command:

```
cargo run -- from_mnemonic "fit refuse hotel collect tortoise race rail weasel little medal couch remember"
```

**Optional passphrase:** You will be prompted to enter a passphrase (leave blank to not use one).

* Example of result:

```
BIP39 Mnemonic (given phrase): fit refuse hotel collect tortoise race rail weasel little medal couch remember
Seed: 2C9AE93C7FA7D5296472B6E0F8928F94963E96ACAFDF1924AF8B7A8471B04FA15F49C98023FDC84BBB5979085F91A577E1A36A7BAC9C4C735D44379D7A915D59
Solana Public Key: EsiyKK61Ycv4XXqUoFJa2SuFJGHjVeWgAB5UvaNkb713
```

**Note**: The BIP39 standard includes a predefined list of words used to generate cryptographic keys. Your custom mnemonic phrase must consist of words exclusively from this list to be valid. Using words not in the BIP39 list will lead to errors in generating a valid seed.


### 🌐 ```get_pubkey_from_keypair_file``` operation:

> Retrieve Public Key from Stored Keypair.

This is useful for retrieving your Solana public key if you have already generated and stored your keypair locally.

This command reads the keypair stored in ```[project-directory]/storage/keypair/id.json```, extracts the public key, and displays it.

* Command:

```
cargo run -- get_pubkey_from_keypair_file
```

* Example of result:

```
Solana Public Key: EsiyKK61Ycv4XXqUoFJa2SuFJGHjVeWgAB5UvaNkb713
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



## Environment variables

Environment variables are configured in the ```.env``` file.

You can configure these environment variables:

* ```NB_DERIVATIONS``` env var (default value: ```1```).

* ```KEYPAIR_PATH``` env var (default value: ```./storage/keypair/id.json```).

* ```KEYPAIR_DIR``` env var (default value: ```./storage/keypair/derived```).

```KEYPAIR_DIR``` is only useful if ```NB_DERIVATIONS``` is > ```1```.



## Some interesting links

* **BIP39**: Learn more about the **BIP39** standard [here](https://github.com/bitcoin/bips/blob/master/bip-0039.mediawiki).
* **SLIP44**: Learn more about the **SLIP44** [here](https://github.com/satoshilabs/slips/blob/master/slip-0044.md).
