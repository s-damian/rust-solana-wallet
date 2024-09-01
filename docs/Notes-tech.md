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



## Run tests

```bash
cargo test
```



## Run / Build / Release

### Run:

Compiler (si nécessaire) et exécuter immédiatement le programme :

```bash
cargo run <arguments>
```


### Build - Debug:

Compiler (une version de débogage) :

```bash
cargo build
```

> Cette commande va créer ce fichier compilé: `target/debug/rust_solana_wallet`


### Build - Release:

Compiler (une version de Release pour la production) :

```bash
cargo build --release
```

> Cette commande va créer ce fichier compilé: `target/release/rust_solana_wallet`



## Utiliser la Release

Dans cet exemple, nous allons créer une version compilée, et nous allons l'utilser dans notre Desktop de notre Linux.

- Allez au répertoire du projet :

```bash
cd /<your-path>/rust-solana-wallet
```

- Compilez pour créer une release :

```bash
cargo build --release
```

- Copiez fichier de Release dans votre Desktop :

```bash
cp ./target/release/rust_solana_wallet /home/<your-username>/Desktop
```

- Configurez votre fichier `.env`, puis copiez le dans votre Desktop :

```bash
cp ./.env /home/<your-username>/Desktop
```

- Allez dans votre Desktop:

```bash
cd /home/<your-username>/Desktop
```

- Puis vous pouvez utiliser la release du Wallet. Exemple (pour générer une seed) :

```bash
./rust_solana_wallet generate_seed
```
