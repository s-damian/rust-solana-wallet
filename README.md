
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
* **Send SOLs (lamports)**: Send SOLs to a recipient address (sign outgoing transaction).



## Summary

* [generate_seed](#-generate_seed-command): Generates a 12-word BIP39 mnemonic phrase, derives the corresponding seed, saves the keypair, and displays the public key.

* [from_mnemonic](#-from_mnemonic-command): Accepts a user-provided BIP39 mnemonic phrase, derives the corresponding seed, saves the keypair, and displays the public key.

* [get_pubkey_from_keypair_file](#-get_pubkey_from_keypair_file-command): Displays the public key from a keypair stored in a JSON file.

* [send_sols](#-send_sols-command): Send SOLs to a recipient address.



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

```generate_seed``` and ```from_mnemonic``` commands writes the generated keypair to the ```[project-directory]/storage/keypair/id.json``` file (```KEYPAIR_PATH``` env var), allowing you to store or utilize the keypair in your Solana applications.

**Multiple keypairs (derivations)**:

If you want to generate several keypairs and several public keys with a single mnemonic phrase, you must set the ```NB_DERIVATIONS``` environment variable to a value greater than ```1```.

Your non-derived keypair will be created in your ```[project-directory]/storage/keypair/id.json``` file (```KEYPAIR_PATH``` env var) JSON file.

And the other keypairs (which will be derived from your seed) will be created in JSON files in your ```[project-directory]/storage/keypair/derived``` directory (```KEYPAIR_DIR``` env var).


### 🌐 ```generate_seed``` command:

> Generate and display a random mnemonic phrase.

This command generates a new mnemonic phrase randomly, calculates the corresponding seed, displays the seed, displays the Solana public key, and generates and writes the keypair to the JSON file.

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



### 🌐 ```from_mnemonic``` command:

> Generate and display a mnemonic from a specific phrase.

> Command with arguments: cargo run -- from_mnemonic ```PHRASE```

To generate and display the seed and Solana public key from a specific mnemonic phrase, pass the phrase (12 words or 24 words for examples) as an argument.

This will also generates and writes the keypair to the JSON file.

**Example** with this 12 words: ```shed``` ```scorpion``` ```manual``` ```wheat``` ```monster``` ```phone``` ```winter``` ```toe``` ```dream``` ```kitchen``` ```salad``` ```column```.

* Command:

```
cargo run -- from_mnemonic "shed scorpion manual wheat monster phone winter toe dream kitchen salad column"
```

**Optional passphrase:** You will be prompted to enter a passphrase (leave blank to not use one).

* Example of result:

```
BIP39 Mnemonic (given phrase): shed scorpion manual wheat monster phone winter toe dream kitchen salad column
Seed: 34A0EACFFDF41445C0B7E43C2D730C54F4CD1D8334528F73E3D5F2C2977FAABA7CAD88EBDA6A1F02CE6BB596F04036305A32B96303F93FF864D268539739AFF8
Solana Public Key: FTGJPL5hia749v3jhNWJA7uE2VoVGyofB7BBL2cLwoPc
```


### 🌐 ```get_pubkey_from_keypair_file``` command:

> Retrieve public key from stored keypair.

This is useful for retrieving your Solana public key if you have already generated and stored your keypair locally.

This command reads your JSON keypair file stored, extracts the public key, and displays it.

* Command:

```
cargo run -- get_pubkey_from_keypair_file
```

This command reads the keypair stored in ```[project-directory]/storage/keypair/id.json``` file (```KEYPAIR_PATH``` env var).

* Example of result:

```
Solana Public Key: FTGJPL5hia749v3jhNWJA7uE2VoVGyofB7BBL2cLwoPc
```


### 🌐 ```send_sols``` command:

> Send SOLs (lamports) to a recipient address (sign outgoing transaction).

> Command with arguments: cargo run -- send ```RECIPIENT``` ```AMOUNT```

This command allows you to sign an outgoing transaction from your wallet to a destination address.

**Example** to send ```0.002``` SOLs (```2000000``` lamports) to recipient address ```EMLY3VvNZ41yMWyPQy2AiEfJTPpZdzeGNG5zaaq3Lihb```.

* Command:

```
cargo run -- send EMLY3VvNZ41yMWyPQy2AiEfJTPpZdzeGNG5zaaq3Lihb 2000000
```

This command will sign the transaction with the keypair which is stored in the file ```[project-directory]/storage/keypair/id.json``` file (```KEYPAIR_PATH``` env var).

* Example of result (when successfully):

```
Transaction sent successfully!
```

* Example of result (when it fails):

```
Failed to send transaction: unable to confirm transaction. This can happen in situations such as transaction expiration and insufficient fee-payer funds

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

* ```NB_DERIVATIONS``` (default value: ```1```).

* ```KEYPAIR_PATH``` (default value: ```./storage/keypair/id.json```).

* ```KEYPAIR_DIR``` (default value: ```./storage/keypair/derived```).

```KEYPAIR_DIR``` is only useful if ```NB_DERIVATIONS``` is > ```1```.

* ```RPC_URL``` (default value: ```https://api.testnet.solana.com```).



## Some interesting links

* **BIP39**: Learn more about the **BIP39** standard [here](https://github.com/bitcoin/bips/blob/master/bip-0039.mediawiki).
* **SLIP44**: Learn more about the **SLIP44** [here](https://github.com/satoshilabs/slips/blob/master/slip-0044.md).
