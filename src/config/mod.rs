use serde::Deserialize;
use std::path::Path;
use toml::de::Error;
use url::Url;

use crate::utils::read_file_to_string_or_err;

#[derive(Deserialize)]
pub struct CoveAPIConfig {
    pub debug: Option<bool>,
    pub environment: Environment,
}

#[derive(Deserialize)]
pub struct Environment {
    pub openapi_path: Box<Path>,
    pub app_base_url: Url,
}

impl CoveAPIConfig {
    fn from_str(config_str: &str) -> Result<CoveAPIConfig, ConfigurationError> {
        match toml::from_str(config_str) {
            Ok(config) => Ok(config),
            Err(err) => Err(ConfigurationError::IllegalSyntax(err)),
        }
    }

    pub fn from_path(path: &Path) -> Result<CoveAPIConfig, ConfigurationError> {
        CoveAPIConfig::from_str(&read_file_to_string_or_err(path, ConfigurationError::IssueOpeningFile)?)
    }

    pub fn is_debug(&self) -> bool {
        match self.debug.unwrap_or(false) {
            true => true,
            false => false,
        }
    }
}

#[derive(Debug)]
pub enum ConfigurationError {
    IssueOpeningFile,
    IllegalSyntax(Error),
}

#[cfg(test)]
mod test {
    use std::path::Path;

    use url::Url;

    use super::{CoveAPIConfig, Environment};

    const CONFIG_STR: &str = r#"
        [environment]
        openapi_path = './dump/swagger.json'
        app_base_url = 'http://localhost:8080'
    "#;

    #[test]
    fn can_fetch_valid_path() {
        assert_eq!(
            CoveAPIConfig::from_str(CONFIG_STR)
                .unwrap()
                .environment
                .openapi_path
                .to_str()
                .unwrap(),
            "./dump/swagger.json"
        );
    }

    #[test]
    fn can_fetch_valid_url() {
        assert_eq!(
            CoveAPIConfig::from_str(CONFIG_STR)
                .unwrap()
                .environment
                .app_base_url
                .as_str(),
            "http://localhost:8080/"
        );
    }

    #[test]
    fn can_fetch_debug_info() {
        assert_eq!(CoveAPIConfig::from_str(CONFIG_STR).unwrap().debug, None);
    }

    #[test]
    fn asserts_debug_status_false_when_set_and_non() {
        let path = Path::new("./dump");
        let mut config = CoveAPIConfig {
            debug: None,
            environment: Environment {
                app_base_url: Url::parse("http://example.com").unwrap(),
                openapi_path: Box::from(path),
            },
        };

        assert!(!config.is_debug());
        config.debug = Some(false);
        assert!(!config.is_debug());
    }

    #[test]
    fn asserts_debug_status_true_when_set() {
        let path = Path::new("./dump");
        let config = CoveAPIConfig {
            debug: Some(true),
            environment: Environment {
                app_base_url: Url::parse("http://example.com").unwrap(),
                openapi_path: Box::from(path),
            },
        };
        assert!(config.is_debug());
    }

    #[test]
    fn can_read_config_file() {
        let path = Path::new("./dump/coveapi.toml");
        CoveAPIConfig::from_path(path).unwrap();
    }
}
