use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub type Props = HashMap<String, String>;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Configuration {
    pub presets: Vec<Preset>,
}

impl Configuration {

    pub fn get_preset(&self, name: &str) -> Option<&Preset> {
        self.presets.iter().find(|p|p.name.as_str() == name)
    }

}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Preset {
    pub name: String,
    pub display_config: DisplayConfigState,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct DisplayConfigState {
    pub serial: u32,
    pub monitors: Vec<Monitor>,
    pub logical_monitors: Vec<LogicalMonitor>,
    pub properties: Props,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct MonitorInfo {
    pub connector: String,
    pub vendor: String,
    pub product: String,
    pub serial: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Mode {
    pub id: String,
    pub width: i32,
    pub height: i32,
    pub refresh_rate: f64,
    pub preferred_scale: f64,
    pub supported_scales: Vec<f64>,
    pub properties: Props,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Monitor {
    pub monitor_info: MonitorInfo,
    pub modes: Vec<Mode>,
    pub properties: Props,
}

impl Monitor {

    pub fn get_current_mode_id(&self) -> Option<&String> {
        self.modes.iter().find(|m| { m.properties.get("is-current").is_some_and(|v| { v == "1" }) })
            .map(|cur_md| { &cur_md.id })
    }

}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct LogicalMonitor {
    pub x: i32,
    pub y: i32,
    pub scale: f64,
    pub transform: u32,
    pub primary: bool,
    pub monitors: Vec<MonitorInfo>,
    pub properties: Props,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn generate_preset_with_name(name: &str, serial: u32) -> Preset {
        Preset {
            name: String::from(name),
            display_config: DisplayConfigState {
                serial,
                monitors: vec![],
                logical_monitors: vec![],
                properties: Props::new()
            }
        }
    }

    #[test]
    fn get_preset_by_name() {
        let preset1 = generate_preset_with_name("Preset1", 1);
        let preset2 = generate_preset_with_name("Preset2", 2);

        let conf = Configuration {
            presets: vec![
                generate_preset_with_name("Preset1", 1),
                generate_preset_with_name("Preset2", 2)
            ]
        };

        assert_eq!(&preset1, conf.get_preset("Preset1").unwrap());
        assert_eq!(&preset2, conf.get_preset("Preset2").unwrap());
        assert!(conf.get_preset("Missing").is_none());
    }

    #[test]
    fn get_current_mode_id() {
        let monitor = Monitor {
            monitor_info: MonitorInfo {
                connector: "".to_string(),
                vendor: "".to_string(),
                product: "".to_string(),
                serial: "".to_string(),
            },
            modes: vec![
                Mode {
                    id: "1".to_string(),
                    width: 0,
                    height: 0,
                    refresh_rate: 0.0,
                    preferred_scale: 0.0,
                    supported_scales: vec![],
                    properties: [("is-current".to_string(), "1".to_string())].into()
                },
                Mode {
                    id: "2".to_string(),
                    width: 0,
                    height: 0,
                    refresh_rate: 0.0,
                    preferred_scale: 0.0,
                    supported_scales: vec![],
                    properties: HashMap::new()
                }
            ],
            properties: HashMap::new()
        };

        assert_eq!("1", monitor.get_current_mode_id().unwrap());
    }

}