use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub type Props = HashMap<String, String>;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Configuration {
    pub presets: Vec<Preset>,
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
