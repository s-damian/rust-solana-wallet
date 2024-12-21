# Rust Solana Wallet - Commands


A Solana Wallet made in France 🇫🇷

**Here you can see the examples of the commands as in a real context**


| Functionality | Description | Command |
|---------------|-------------|---------|
| [Generate Mnemonic](#generate-mnemonic) | Creates a new mnemonic phrase (12 words, with an optional passphrase). | `generate_seed` |
| [Recover Keypair](#recover-keypair) | Recover keypair and seed from a mnemonic phrase. | `recover_seed <RECOVERY_PHRASE>` |
| [Send SOL (lamports)](#send-sol-lamports) | Send SOL to a recipient address (sign outgoing transaction). | `send <RECIPIENT_PUBKEY> <AMOUNT_IN_LAMPORTS>` |
| [Public Key Display](#get-public-key) | Retrieves and displays the public key from the locally stored keypair. | `pubkey` |
| [Get Balance](#get-balance-by-public-key) | Get balance (in SOL and in lamports) by public key. | `balance_by_pubkey <PUBKEY>` |


## Generate Mnemonic

This command generates a new mnemonic phrase (12 words).

```bash
cargo run -- generate_seed
```

Example of result (without passphrase):

```bash
BIP39 Mnemonic (random phrase): owner cherry you seek bless holiday humble rare orchard tennis cycle solid
Enter passphrase (optional):
Seed: A8FBCB43911A10E2F1A6F3289816240ED16FC0EE49F16A488E93A052BAE5169CB8E4EBAF8BC9D0F4545C5108CF11745D247582A0FD28A3095DE2A4CA28C457B5
Solana Public Key: BcHM6w7ywAsktXmmWG4Jjk2PmL2stS1K3ZKnQ6da24hk
```


## Recover Keypair

This command allows you to retrieve your seed (and therefore your private key) via a given mnemonic phrase.

```bash
cargo run -- recover_seed "owner cherry you seek bless holiday humble rare orchard tennis cycle solid"
```

Example of result (without passphrase):

```bash
BIP39 Mnemonic (given phrase): owner cherry you seek bless holiday humble rare orchard tennis cycle solid
Enter passphrase (optional):
Seed: A8FBCB43911A10E2F1A6F3289816240ED16FC0EE49F16A488E93A052BAE5169CB8E4EBAF8BC9D0F4545C5108CF11745D247582A0FD28A3095DE2A4CA28C457B5
Solana Public Key: BcHM6w7ywAsktXmmWG4Jjk2PmL2stS1K3ZKnQ6da24hk
```


## Send SOL (lamports)

This command allows you to send Lamports to a destination address.

```bash
cargo run -- send 27nJwboVxL39gGfwFeefiHYqrFtipmHAHgkxo3xjPJ3L 2000000
```

PS: 2000000 Lamports = 0.002 SOL.

Example of result:

```bash
Transaction sent successfully!
```


## Get Public Key

This command allows you to view your Solana public key if you have already stored your keypair locally.

```bash
cargo run -- pubkey
```

Example of result:

```bash
Solana Public Key: BcHM6w7ywAsktXmmWG4Jjk2PmL2stS1K3ZKnQ6da24hk
```


## Get Balance by Public Key

This command allows you to see the balance of a public address.

```bash
cargo run -- balance_by_pubkey BcHM6w7ywAsktXmmWG4Jjk2PmL2stS1K3ZKnQ6da24hk
```

Example of result:

```bash
Balance: 0.010000000 SOL (10000000 lamports)
```
