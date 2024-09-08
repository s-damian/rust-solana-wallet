use std::sync::Once;

static INIT: Once = Once::new();

pub fn setup() {
    INIT.call_once(|| {
        // Pour les tests: charger les variables d'environnement depuis le fichier ".env.testing".
        dotenv::from_filename(".env.testing").ok();
    });

    assert_eq!(
        std::env::var("TEST_MODE").unwrap_or_default(),
        "true",
        "TEST_MODE should be set to 'true' for tests"
    );
}

pub fn verify_pubkey(pubkey: &str) {
    assert!(
        (32..=44).contains(&pubkey.len()),
        "Public key should be between 32 and 44 characters long, but it's {} characters long",
        pubkey.len()
    );
    assert!(
        pubkey.chars().all(|c| c.is_ascii_alphanumeric()),
        "Public key contains non-alphanumeric characters"
    );
}
