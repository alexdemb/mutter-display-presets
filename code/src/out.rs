use super::model::Preset;
use std::collections::HashMap;

pub fn print_preset(preset: &Preset) {
    println!("Preset: '{}'", preset.name);
    println!();

    let display_config = &preset.display_config;

    println!("Physical displays:");
    println!();

    let mut current_mode_by_connector = HashMap::<String, String>::new();

    for monitor in &display_config.monitors {
        println!("{}:", monitor.monitor_info.connector);
        println!("Vendor: {}", monitor.monitor_info.vendor);
        println!("Model: {}", monitor.monitor_info.product);
        print!("Supported modes: ");

        for mode in &monitor.modes {
            if let Some(v) = mode.properties.get("is-current") {
                if v == "1" {
                    current_mode_by_connector.insert(
                        monitor.monitor_info.connector.clone(),
                        format!("{}x{}@{}", mode.width, mode.height, mode.refresh_rate),
                    );
                }
            }
            print!("{}x{}@{} ", mode.width, mode.height, mode.refresh_rate);
        }
        println!();
        println!();
    }

    println!("Logical displays:");
    println!();
    for lm in &display_config.logical_monitors {
        let connectors: Vec<&String> = lm.monitors.iter().map(|m| &m.connector).collect();
        println!("Connectors: {:?}", connectors);
        println!("X: {}", lm.x);
        println!("Y: {}", lm.y);
        println!("Scale: {}", lm.scale);
        println!("Primary: {}", lm.primary);
        println!("Transform: {}", lm.transform);
        println!("Mode:");

        for connector in connectors {
            println!(
                "{}: {}",
                connector,
                current_mode_by_connector
                    .get(connector)
                    .unwrap_or(&"".to_string())
            );
        }

        println!();
    }
}
