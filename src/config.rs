use dotenv::dotenv;
#[cfg(test)]
use mocktopus::macros::*;

#[derive(Debug)]
pub enum ConfigValidationError {
    Io(std::io::ErrorKind),
    MixedEnv(String),
    DotenvParsing(String),
}

#[cfg_attr(test, mockable)]
fn dotenv_wrapper() -> dotenv::Result<std::path::PathBuf> {
    dotenv()
}

pub fn validate_config(env_config: MoveConfig) -> Result<String, ConfigValidationError> {
    match dotenv_wrapper() {
        Ok(ref r) if env_config.is_empty() => Ok(format!("using env vars from file: {:?}", r)),
        Ok(ref r) => Err(ConfigValidationError::MixedEnv(format!(
            "Mixing env vars and .env file is not supported. You have two options \n \
             1. Delete file: {:?} \n \
             2. Unset env vars {:?}",
            r,
            env_config.valid_keys()
        ))),
        Err(dotenv::Error::Io(ref e))
            if e.kind() == std::io::ErrorKind::NotFound && env_config.is_valid() =>
        {
            Ok(format!("using env vars from process"))
        }
        Err(dotenv::Error::Io(e)) => Err(ConfigValidationError::Io(e.kind())),
        Err(dotenv::Error::LineParse(key)) => Err(ConfigValidationError::DotenvParsing(format!(
            "found invalid line in .env: {:?}",
            key
        ))),
        Err(dotenv::Error::EnvVar(key)) => Err(ConfigValidationError::DotenvParsing(format!(
            "error: {:?}",
            key
        ))),
    }
}

#[derive(Debug)]
pub struct MoveConfig {
    id: ConfigItem,
    key: ConfigItem,
}

impl MoveConfig {
    pub fn from_vars() -> MoveConfig {
        let mut config = MoveConfig::new();
        config.id.value = std::env::var(&config.id.name);
        config.key.value = std::env::var(&config.key.name);

        config
    }

    pub fn is_empty(&self) -> bool {
        self.id.value.is_err() && self.key.value.is_err()
    }

    pub fn is_valid(&self) -> bool {
        self.id.value.is_ok() && self.key.value.is_ok()
    }

    pub fn valid_keys(&self) -> Vec<&String> {
        let valid_keys = vec![&self.id, &self.key];
        valid_keys
            .iter()
            .filter_map(|key| {
                if key.value.is_ok() {
                    Some(&key.name)
                } else {
                    None
                }
            })
            .collect()
    }

    fn new() -> Self {
        MoveConfig {
            id: ConfigItem {
                name: "AWS_ACCESS_KEY_ID".to_string(),
                value: Err(std::env::VarError::NotPresent),
            },
            key: ConfigItem {
                name: "AWS_SECRET_ACCESS_KEY".to_string(),
                value: Err(std::env::VarError::NotPresent),
            },
        }
    }
}

#[derive(Debug)]
struct ConfigItem {
    name: String,
    value: Result<String, std::env::VarError>,
}

#[cfg(test)]
mod test {
    use super::{dotenv_wrapper, validate_config, ConfigItem, ConfigValidationError, MoveConfig};
    use mocktopus::mocking::*;

    fn setup() -> MoveConfig {
        MoveConfig {
            id: ConfigItem {
                name: "AWS_ACCESS_KEY_ID".to_string(),
                value: Ok("MOCKED_ACCESS_ID".to_string()),
            },
            key: ConfigItem {
                name: "AWS_SECRET_ACCESS_KEY".to_string(),
                value: Ok("MOCKED_ACCESS_KEY".to_string()),
            },
        }
    }

    #[test]
    fn validation_config_uses_dotenv_for_empty_env() {
        let env_config = MoveConfig::new();
        dotenv_wrapper
            .mock_safe(|| MockResult::Return(Ok(std::path::PathBuf::from("./mocked-env"))));

        let validation_result = validate_config(env_config);

        assert!(validation_result.is_ok());
        assert_eq!(
            validation_result.unwrap(),
            format!(
                "using env vars from file: {:?}",
                std::path::PathBuf::from("./mocked-env").to_str().unwrap()
            )
        );
    }

    #[test]
    fn validation_config_uses_dotenv_with_existing_env() {
        let env_config = setup();
        dotenv_wrapper
            .mock_safe(|| MockResult::Return(Ok(std::path::PathBuf::from("./mocked-env"))));

        let validation_result = validate_config(env_config);

        match validation_result {
            Err(ConfigValidationError::MixedEnv(_)) => assert!(true),
            x => assert!(
                false,
                "Wrong result type: {:?}, should be ConfigValidationError::MixedEnv",
                x
            ),
        };
    }

    #[test]
    fn validation_config_env_file_not_found_but_env_is_valid() {
        let env_config = setup();
        dotenv_wrapper.mock_safe(|| {
            MockResult::Return(Err(dotenv::Error::Io(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "File not found",
            ))))
        });

        let validation_result = validate_config(env_config);

        assert!(validation_result.is_ok());
        assert_eq!(validation_result.unwrap(), "using env vars from process");
    }

    #[test]
    fn validation_config_env_file_not_found_and_env_is_invalid() {
        let env_config = MoveConfig::new();
        dotenv_wrapper.mock_safe(|| {
            MockResult::Return(Err(dotenv::Error::Io(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "File not found",
            ))))
        });

        let validation_result = validate_config(env_config);

        match validation_result {
            Err(ConfigValidationError::Io(std::io::ErrorKind::NotFound)) => assert!(true),
            x => assert!(false, "Wrong result type: {:?}, should be ConfigValidationError::Io(std::io::ErrorKind::NotFound)", x),
        };
    }

    #[test]
    fn validation_config_exposes_io_error_kinds() {
        let env_config = setup();
        dotenv_wrapper.mock_safe(|| {
            MockResult::Return(Err(dotenv::Error::Io(std::io::Error::new(
                std::io::ErrorKind::PermissionDenied,
                "Permission denied",
            ))))
        });

        let validation_result = validate_config(env_config);

        match validation_result {
            Err(ConfigValidationError::Io(std::io::ErrorKind::PermissionDenied)) => assert!(true),
            x => assert!(false, "Wrong result type: {:?}, should be ConfigValidationError::Io(std::io::ErrorKind::PermissionDenied)", x),
        };
    }

    #[test]
    fn validation_config_dotenv_cant_parse_line() {
        let env_config = setup();
        dotenv_wrapper.mock_safe(|| {
            MockResult::Return(Err(dotenv::Error::LineParse(
                "Error parsing line".to_string(),
            )))
        });

        let validation_result = validate_config(env_config);

        match validation_result {
            Err(ConfigValidationError::DotenvParsing(_)) => assert!(true),
            x => assert!(
                false,
                "Wrong result type: {:?}, should be ConfigValidationError::DotenvParsing(_)",
                x
            ),
        };
    }

    #[test]
    fn validation_config_dotenv_cant_fetch_envvar() {
        let env_config = setup();
        dotenv_wrapper.mock_safe(|| {
            MockResult::Return(Err(dotenv::Error::EnvVar(std::env::VarError::NotPresent)))
        });

        let validation_result = validate_config(env_config);

        match validation_result {
            Err(ConfigValidationError::DotenvParsing(_)) => assert!(true),
            x => assert!(
                false,
                "Wrong result type: {:?}, should be ConfigValidationError::DotenvParsing(_)",
                x
            ),
        };
    }
}
