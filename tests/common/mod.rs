use std::sync::Once;

static INIT: Once = Once::new();

pub fn setup() {
    INIT.call_once(|| {
        // Pour les tests: charger les variables d'environnement depuis le fichier ".env.testing".
        dotenv::from_filename(".env.testing").ok();
    });
}
