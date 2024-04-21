
# Example of a Solana Wallet in Rust

This project demonstrates how to create and manage a Solana wallet using Rust. It provides functionalities to generate a new random mnemonic phrase or use a given phrase to generate and display the corresponding Solana seed and public key.



## Requirements

* Rust: Ensure Rust is installed on your system. You can install Rust using rustup.
* Cargo: Rust's package manager, installed automatically with Rust.



## Setup

Before running the project, make sure to clone the repository and navigate into the project directory:

```
git clone [repository-url]
cd [project-directory]
```



## Compilation and Execution

These operations writes the generated keypair to ```[project-directory]/storage/keypair.txt```, allowing you to store or utilize the keypair in your Solana applications.

### Generate and Display a Random Mnemonic

This command generates a new mnemonic phrase randomly, calculates the corresponding seed, displays the seed, displays the Solana public key, and writes the keypair to a file:

```
cargo run
```

### Generate and Display a Mnemonic from a Specific Phrase

To generate and display the seed and Solana public key from a specific 12-word mnemonic phrase, pass the phrase (12 words) as an argument.
This will also save the generated keypair to a file.

**Example** with the 12 words "fit refuse hotel collect tortoise race rail weasel little medal couch remember":

```
cargo run "fit refuse hotel collect tortoise race rail weasel little medal couch remember"
```.

**Note**: The BIP 39 standard includes a predefined list of words used to generate cryptographic keys. Your custom mnemonic phrase must consist of words exclusively from this list to be valid. Using words not in the BIP 39 list will lead to errors in generating a valid seed.



## Lint and Static Analysis

Maintain code quality and style consistency using the following tools:

### Code Formatting

To format the codebase according to Rust's coding standards, use:

```
cargo fmt
```

### Static Code Analysis

To perform static code analysis and catch common mistakes and inefficiencies, use:

```
cargo clippy
```



## Note

Replace ```repository-url``` and ```project-directory``` with the actual URL of your Git repository and the name of the folder where your project is stored.
