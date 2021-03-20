#[macro_use]
extern crate serde_derive;

use config::{Config, File, ConfigError};

#[derive(Debug, Deserialize)]
pub struct Settings {
    test_env: i32,
    test_mode: String,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let mut config = Config::default();

        config.merge(File::with_name("config"));

        config.try_into()
    }
}

#[test]
fn simple() {
    let settings = Settings::new().unwrap();
    assert_eq!(settings.test_env, 1);
    assert_eq!(settings.test_mode, String::from("testing"));
}