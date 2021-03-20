use dotenv;
use std::env;
use std::path::Path;

#[test]
fn load_env() {
    dotenv::from_filename(Path::new(".env.testing"));

    let env_var = env::var("TEST_ENV");
    assert!(env_var.is_ok());
    assert_eq!(env_var, Ok(String::from("1")));

    let mode = dotenv::var("TEST_MODE");
    assert!(mode.is_ok());
    assert_eq!(mode.unwrap(), String::from("testing"));

    let other = dotenv::var("NONE_ENV");
    assert!(other.is_err())
}