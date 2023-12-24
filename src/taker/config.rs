//! Taker configuration. Controlling various behavior.
//!
//!  Represents the configuration options for the Taker module, controlling behaviors
//! such as refund locktime, connection attempts, sleep delays, and timeouts.

use std::{io, path::PathBuf};

use crate::utill::{parse_field, parse_toml};
/// Taker configuration with refund, connection, and sleep settings.
#[derive(Debug, Clone, PartialEq)]
pub struct TakerConfig {
    pub refund_locktime: u16,
    pub refund_locktime_step: u16,

    pub first_connect_attempts: u32,
    pub first_connect_sleep_delay_sec: u64,
    pub first_connect_attempt_timeout_sec: u64,

    pub reconnect_attempts: u32,
    pub reconnect_short_sleep_delay: u64,
    pub reconnect_long_sleep_delay: u64,
    pub short_long_sleep_delay_transition: u32,
    pub reconnect_attempt_timeout_sec: u64,
}

impl Default for TakerConfig {
    fn default() -> Self {
        Self {
            refund_locktime: 48,
            refund_locktime_step: 48,
            first_connect_attempts: 5,
            first_connect_sleep_delay_sec: 1,
            first_connect_attempt_timeout_sec: 20,
            reconnect_attempts: 3200,
            reconnect_short_sleep_delay: 10,
            reconnect_long_sleep_delay: 60,
            short_long_sleep_delay_transition: 60,
            reconnect_attempt_timeout_sec: 300,
        }
    }
}

impl TakerConfig {
    pub fn new(file_path: Option<&PathBuf>) -> io::Result<Self> {
        let default_config = Self::default();

        let section = if let Some(path) = file_path {
            if path.exists() {
                parse_toml(path)?
            } else {
                log::warn!(
                    "Taker config file not found, creating default config file at path: {}",
                    path.display()
                );
                create_default_taker_dirs(&path);
                parse_toml(&path)?
            }
        } else {
            let default_path = PathBuf::from("taker.toml");
            if default_path.exists() {
                parse_toml(&default_path)?
            } else {
                let default_taker_config_path = get_config_dir().join("taker.toml");
                if !default_taker_config_path.exists() {
                    create_default_taker_dirs(&default_taker_config_path);
                }
                log::warn!(
                    "Taker config file not found, creating a default config file at path: {}",
                    default_path.display()
                );
                parse_toml(&default_taker_config_path)?
            }
        };

        let taker_config_section = section.get("taker_config").cloned().unwrap_or_default();

        Ok(Self {
            refund_locktime: parse_field(
                taker_config_section.get("refund_locktime"),
                default_config.refund_locktime,
            )
            .unwrap_or(default_config.refund_locktime),
            refund_locktime_step: parse_field(
                taker_config_section.get("refund_locktime_step"),
                default_config.refund_locktime_step,
            )
            .unwrap_or(default_config.refund_locktime_step),
            first_connect_attempts: parse_field(
                taker_config_section.get("first_connect_attempts"),
                default_config.first_connect_attempts,
            )
            .unwrap_or(default_config.first_connect_attempts),
            first_connect_sleep_delay_sec: parse_field(
                taker_config_section.get("first_connect_sleep_delay_sec"),
                default_config.first_connect_sleep_delay_sec,
            )
            .unwrap_or(default_config.first_connect_sleep_delay_sec),
            first_connect_attempt_timeout_sec: parse_field(
                taker_config_section.get("first_connect_attempt_timeout_sec"),
                default_config.first_connect_attempt_timeout_sec,
            )
            .unwrap_or(default_config.first_connect_attempt_timeout_sec),
            reconnect_attempts: parse_field(
                taker_config_section.get("reconnect_attempts"),
                default_config.reconnect_attempts,
            )
            .unwrap_or(default_config.reconnect_attempts),
            reconnect_short_sleep_delay: parse_field(
                taker_config_section.get("reconnect_short_sleep_delay"),
                default_config.reconnect_short_sleep_delay,
            )
            .unwrap_or(default_config.reconnect_short_sleep_delay),
            reconnect_long_sleep_delay: parse_field(
                taker_config_section.get("reconnect_long_sleep_delay"),
                default_config.reconnect_long_sleep_delay,
            )
            .unwrap_or(default_config.reconnect_long_sleep_delay),
            short_long_sleep_delay_transition: parse_field(
                taker_config_section.get("short_long_sleep_delay_transition"),
                default_config.short_long_sleep_delay_transition,
            )
            .unwrap_or(default_config.short_long_sleep_delay_transition),
            reconnect_attempt_timeout_sec: parse_field(
                taker_config_section.get("reconnect_attempt_timeout_sec"),
                default_config.reconnect_attempt_timeout_sec,
            )
            .unwrap_or(default_config.reconnect_attempt_timeout_sec),
        })
    }
}

fn create_default_taker_dirs(target_path: &PathBuf) {
    let config_string = String::from(
        "\
                        [taker_config]\n\
                        refund_locktime = 48\n\
                        refund_locktime_step = 48\n\
                        first_connect_attempts = 5\n\
                        first_connect_sleep_delay_sec = 1\n\
                        first_connect_attempt_timeout_sec = 20\n\
                        reconnect_attempts = 3200\n\
                        reconnect_short_sleep_delay = 10\n\
                        reconnect_long_sleep_delay = 60\n\
                        short_long_sleep_delay_transition = 60\n\
                        reconnect_attempt_timeout_sec = 300\n\
                        ",
    );
    write_default_config(target_path, config_string).unwrap();
}

#[cfg(test)]
mod tests {
    use crate::utill::get_home_dir;

    use super::*;
    use std::{
        fs::{self, File},
        io::Write,
        path::PathBuf,
    };

    fn create_temp_config(contents: &str, file_name: &str) -> PathBuf {
        let file_path = PathBuf::from(file_name);
        let mut file = File::create(&file_path).unwrap();
        writeln!(file, "{}", contents).unwrap();
        file_path
    }

    fn remove_temp_config(path: &PathBuf) {
        fs::remove_file(path).unwrap();
    }

    #[test]
    fn test_valid_config() {
        let contents = r#"
        [taker_config]
        refund_locktime = 48
        refund_locktime_step = 48
        first_connect_attempts = 5
        first_connect_sleep_delay_sec = 1
        first_connect_attempt_timeout_sec = 20
        reconnect_attempts = 3200
        reconnect_short_sleep_delay = 10
        reconnect_long_sleep_delay = 60
        short_long_sleep_delay_transition = 60
        reconnect_attempt_timeout_sec = 300
        "#;
        let config_path = create_temp_config(contents, "valid_taker_config.toml");
        let config = TakerConfig::new(Some(&config_path)).unwrap();
        remove_temp_config(&config_path);

        let default_config = TakerConfig::default();
        assert_eq!(config, default_config);
    }

    #[test]
    fn test_missing_fields() {
        let contents = r#"
            [taker_config]
            refund_locktime = 48
        "#;
        let config_path = create_temp_config(contents, "missing_fields_taker_config.toml");
        let config = TakerConfig::new(Some(&config_path)).unwrap();
        remove_temp_config(&config_path);

        assert_eq!(config.refund_locktime, 48);
        assert_eq!(config, TakerConfig::default());
    }

    #[test]
    fn test_incorrect_data_type() {
        let contents = r#"
            [taker_config]
            refund_locktime = "not_a_number"
        "#;
        let config_path = create_temp_config(contents, "incorrect_type_taker_config.toml");
        let config = TakerConfig::new(Some(&config_path)).unwrap();
        remove_temp_config(&config_path);

        assert_eq!(config, TakerConfig::default());
    }

    #[test]
    fn test_different_data() {
        let contents = r#"
            [taker_config]
            refund_locktime = 49
        "#;
        let config_path = create_temp_config(contents, "different_data_taker_config.toml");
        let config = TakerConfig::new(Some(&config_path)).unwrap();
        remove_temp_config(&config_path);
        assert_eq!(config.refund_locktime, 49);
        assert_eq!(
            TakerConfig {
                refund_locktime: 48,
                ..config
            },
            TakerConfig::default()
        )
    }

    #[test]
    fn test_missing_file() {
        let config_path = get_home_dir().join("taker.toml");
        let config = TakerConfig::new(Some(&config_path)).unwrap();
        remove_temp_config(&config_path);
        assert_eq!(config, TakerConfig::default());
    }
}
