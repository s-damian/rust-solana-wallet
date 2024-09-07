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
