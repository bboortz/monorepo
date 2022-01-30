use env_logger::Env;
use std::sync::Once;

static INIT: Once = Once::new();

/// Setup function that is only run once, even if called multiple times.
pub fn setup() {
    INIT.call_once(|| {
        let env = Env::default()
            .filter_or("APP_LOG_LEVEL", "warn")
            .write_style_or("APP_LOG_STYLE", "always");

        env_logger::init_from_env(env);
    });
    debug!("initializing conntray ...");
}
