use std::path::Path;

use log::debug;

use crate::model::Configuration;

fn create_empty_config_file(path: &String) -> Result<(), Box<dyn std::error::Error>> {
    write_config(path, &Configuration { presets: vec![] })?;
    Ok(())
}

pub fn read_config(path: &String) -> Result<Configuration, Box<dyn std::error::Error>> {
    let config_path = Path::new(path);
    if !config_path.exists() {
        debug!(
            "Configuration file {} does not exist. Creating empty configuration.",
            path
        );
        create_empty_config_file(path)?;
    }

    let config_str = std::fs::read_to_string(path)?;

    let conf: Configuration = serde_json::from_str(config_str.as_str())?;
    debug!("Configuration retrieved from file {}.", path);

    Ok(conf)
}

pub fn write_config(path: &String, conf: &Configuration) -> Result<(), Box<dyn std::error::Error>> {
    let conf_json = serde_json::to_string(&conf)?;

    debug!("Saving configuration to file {}", path);

    std::fs::write(path, conf_json)?;

    debug!("Configuration saved to file {}", path);

    Ok(())
}

#[cfg(test)]
mod tests {
    use uuid::Uuid;

    use crate::model::{DisplayConfigState, Preset, Props};

    use super::*;

    fn get_test_configuration() -> Configuration {
        Configuration {
            presets: vec![Preset {
                name: "TestPreset".to_string(),
                display_config: DisplayConfigState {
                    serial: 1u32,
                    monitors: vec![],
                    logical_monitors: vec![],
                    properties: Props::new(),
                },
            }],
        }
    }

    #[test]
    fn creates_empty_config_if_absent_on_read_attempt() {
        let path = format!("/tmp/{}.json", Uuid::new_v4().to_string());
        let configuration = read_config(&path).unwrap();

        assert_eq!(configuration, Configuration { presets: vec![] })
    }

    #[test]
    fn reads_existing_config() {
        let path = format!("/tmp/{}.json", Uuid::new_v4().to_string());

        let content = r#"
        {
            "presets": [
                {
                    "name": "TestPreset",
                    "display_config": {
                        "serial": 1,
                        "monitors": [],
                        "logical_monitors": [],
                        "properties": {}
                    }
                }
            ]
        }
        "#;

        std::fs::write(&path, content).unwrap();

        let configuration = read_config(&path).unwrap();

        assert_eq!(configuration, get_test_configuration());
    }

    #[test]
    fn writes_config() {
        let path = format!("/tmp/{}.json", Uuid::new_v4().to_string());

        write_config(&path, &get_test_configuration()).expect("Error writing configuration");

        let content = std::fs::read_to_string(path).unwrap();

        assert_eq!(
            content,
            serde_json::to_string(&get_test_configuration()).unwrap()
        );
    }
}
