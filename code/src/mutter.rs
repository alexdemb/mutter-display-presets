use super::model::{DisplayConfigState, LogicalMonitor, Mode, Monitor, MonitorInfo, Props};
use dbus::arg::{PropMap, RefArg};
use dbus::blocking::{Connection, Proxy};
use std::collections::HashMap;
use std::time::Duration;

const DESTINATION: &str = "org.gnome.Mutter.DisplayConfig";
const PATH: &str = "/org/gnome/Mutter/DisplayConfig";

type GetCurrentStateResponse = (
    u32,
    Vec<(
        (String, String, String, String),
        Vec<(String, i32, i32, f64, f64, Vec<f64>, PropMap)>,
        PropMap,
    )>,
    Vec<(
        i32,
        i32,
        f64,
        u32,
        bool,
        Vec<(String, String, String, String)>,
        PropMap,
    )>,
    PropMap,
);
type GetCurrentStateMonitorInfo = (String, String, String, String);
type GetCurrentStateMode = (String, i32, i32, f64, f64, Vec<f64>, PropMap);
type GetCurrentStateLogicalMonitor = (
    i32,
    i32,
    f64,
    u32,
    bool,
    Vec<(String, String, String, String)>,
    PropMap,
);
type GetCurrentStateMonitor = (
    (String, String, String, String),
    Vec<(String, i32, i32, f64, f64, Vec<f64>, PropMap)>,
    PropMap,
);

impl From<&GetCurrentStateResponse> for DisplayConfigState {
    fn from(value: &GetCurrentStateResponse) -> Self {
        DisplayConfigState {
            serial: value.0,
            monitors: value.1.iter().map(Monitor::from).collect(),
            logical_monitors: value.2.iter().map(LogicalMonitor::from).collect(),
            properties: prop_map_to_props(&value.3),
        }
    }
}

impl From<&GetCurrentStateMonitorInfo> for MonitorInfo {
    fn from(value: &GetCurrentStateMonitorInfo) -> Self {
        MonitorInfo {
            connector: value.0.clone(),
            vendor: value.1.clone(),
            product: value.2.clone(),
            serial: value.3.clone(),
        }
    }
}

impl From<&GetCurrentStateMode> for Mode {
    fn from(value: &GetCurrentStateMode) -> Self {
        return Mode {
            id: value.0.clone(),
            width: value.1,
            height: value.2,
            refresh_rate: value.3,
            preferred_scale: value.4,
            supported_scales: value.5.clone(),
            properties: prop_map_to_props(&value.6),
        };
    }
}

impl From<&GetCurrentStateLogicalMonitor> for LogicalMonitor {
    fn from(value: &GetCurrentStateLogicalMonitor) -> Self {
        LogicalMonitor {
            x: value.0,
            y: value.1,
            scale: value.2,
            transform: value.3,
            primary: value.4,
            monitors: value.5.iter().map(MonitorInfo::from).collect(),
            properties: prop_map_to_props(&value.6),
        }
    }
}

impl From<&GetCurrentStateMonitor> for Monitor {
    fn from(value: &GetCurrentStateMonitor) -> Self {
        Monitor {
            monitor_info: MonitorInfo::from(&value.0),
            modes: value.1.iter().map(Mode::from).collect(),
            properties: prop_map_to_props(&value.2),
        }
    }
}

fn prop_map_to_props(prop_map: &PropMap) -> Props {
    let mut props: Props = HashMap::new();
    for (key, value) in prop_map {
        if let Some(v) = value.as_str() {
            props.insert(String::from(key), String::from(v));
        } else if let Some(i) = value.as_i64() {
            props.insert(String::from(key), format!("{}", i));
        }
    }
    props
}

pub fn get_current_state(
    timeout: &Duration,
) -> Result<DisplayConfigState, Box<dyn std::error::Error>> {
    let conn = Connection::new_session()?;
    let proxy: Proxy<_> = conn.with_proxy(DESTINATION, PATH, timeout.clone());

    use super::mutter_dbus::OrgGnomeMutterDisplayConfig;

    let current_state = proxy.get_current_state()?;

    Ok(DisplayConfigState::from(&current_state))
}

pub fn apply_monitors_config(serial: u32, persistent: bool, state: &DisplayConfigState, timeout: &Duration) -> Result<(), Box<dyn std::error::Error>> {
    let conn = Connection::new_session()?;
    let proxy: Proxy<_> = conn.with_proxy(DESTINATION, PATH, timeout.clone());

    let method = if persistent { 2 } else { 1 };
    let mut logical_monitors: Vec<(i32, i32, f64, u32, bool, Vec<(&str, &str, PropMap)>)> = Vec::new();

    for lm in &state.logical_monitors {
        let mut modes: Vec<(&str, &str, PropMap)> = Vec::new();

        let connectors: Vec<&String> = lm.monitors.iter()
            .map(|m| { &m.connector })
            .collect();

        for connector in connectors {
            if let Some(monitor) = state.monitors.iter().find(|m| m.monitor_info.connector == *connector) {
                if let Some(current_mode_id) = monitor.get_current_mode_id() {
                    modes.push((connector.as_str(), &current_mode_id.as_str(), PropMap::new()));
                }
            }

        }

        logical_monitors.push((lm.x, lm.y, lm.scale, lm.transform, lm.primary, modes));
    }

    use super::mutter_dbus::OrgGnomeMutterDisplayConfig;
    proxy.apply_monitors_config(serial, method, logical_monitors, PropMap::new())?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn monitor_info_from_dbus() {
        let connector = "C".to_string();
        let product = "P".to_string();
        let vendor = "V".to_string();
        let serial = "S".to_string();

        assert_eq!(
            MonitorInfo {
                connector: connector.clone(),
                product: product.clone(),
                vendor: vendor.clone(),
                serial: serial.clone()
            },
            MonitorInfo::from(&(
                connector.clone(),
                vendor.clone(),
                product.clone(),
                serial.clone()
            ))
        )
    }

    #[test]
    fn mode_from_dbus() {
        let id = String::from("I");
        let width: i32 = 1920;
        let height: i32 = 1080;
        let refresh_rate: f64 = 60.0;
        let preferred_scale: f64 = 1.0;
        let supported_scales = vec![1.0, 1.25, 1.5, 2.0];

        assert_eq!(
            Mode {
                id: id.clone(),
                width,
                height,
                refresh_rate,
                preferred_scale,
                supported_scales: supported_scales.clone(),
                properties: Props::new()
            },
            Mode::from(&(
                id.clone(),
                width,
                height,
                refresh_rate,
                preferred_scale,
                supported_scales.clone(),
                PropMap::new()
            ))
        )
    }

    #[test]
    fn monitor_from_dbus() {
        let monitor_info_dbus = (
            "C".to_string(),
            "V".to_string(),
            "P".to_string(),
            "S".to_string(),
        );
        let mode_dbus = (
            "I".to_string(),
            1920,
            1080,
            60.0,
            1.0,
            vec![1.0, 1.25, 1.5, 2.0],
            PropMap::new(),
        );

        assert_eq!(
            Monitor {
                monitor_info: MonitorInfo::from(&monitor_info_dbus),
                modes: vec![Mode::from(&mode_dbus)],
                properties: Props::new()
            },
            Monitor::from(&(monitor_info_dbus, vec![mode_dbus], PropMap::new()))
        )
    }

    #[test]
    fn logical_monitor_from_dbus() {
        let monitor_info_dbus = (
            "C".to_string(),
            "V".to_string(),
            "P".to_string(),
            "S".to_string(),
        );

        assert_eq!(
            LogicalMonitor {
                x: 10,
                y: 20,
                scale: 1.0,
                transform: 2,
                primary: true,
                monitors: vec![MonitorInfo::from(&monitor_info_dbus)],
                properties: Props::new()
            },
            LogicalMonitor::from(&(
                10,
                20,
                1.0,
                2,
                true,
                vec![monitor_info_dbus],
                PropMap::new()
            ))
        )
    }

    #[test]
    fn display_config_state_from_dbus() {
        let monitor_info_dbus = (
            "C".to_string(),
            "V".to_string(),
            "P".to_string(),
            "S".to_string(),
        );
        let mode_dbus = (
            "I".to_string(),
            1920,
            1080,
            60.0,
            1.0,
            vec![1.0, 1.25, 1.5, 2.0],
            PropMap::new(),
        );
        let monitor_dbus = (monitor_info_dbus.clone(), vec![mode_dbus], PropMap::new());
        let logical_monitor_dbus = (
            10,
            20,
            1.0,
            2,
            true,
            vec![monitor_info_dbus.clone()],
            PropMap::new(),
        );

        assert_eq!(
            DisplayConfigState {
                serial: 1,
                monitors: vec![Monitor::from(&monitor_dbus)],
                logical_monitors: vec![LogicalMonitor::from(&logical_monitor_dbus)],
                properties: Props::new(),
            },
            DisplayConfigState::from(&(
                1u32,
                vec![monitor_dbus],
                vec![logical_monitor_dbus],
                PropMap::new()
            ))
        )
    }
}
