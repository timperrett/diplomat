
use config;

#[test]
fn test_load() {
    let c: Result<config::Config, config::ConfigError> =
        config::load("etc/test.config.normal.toml".to_string());
    assert_eq!(c.unwrap().consul.ssl, true);
}
