use std::time::Duration;

const DEFAULT_CONFIG_FILE_NAME: &str = "display-presets.json";

pub fn default_timeout() -> Duration {
    Duration::from_secs(10)
}

pub fn default_config_file_path() -> String {
    if let Ok(xdg_config_home) = std::env::var("XDG_CONFIG_HOME") {
        format!("{}/{}", xdg_config_home, DEFAULT_CONFIG_FILE_NAME)
    } else if let Ok(home) = std::env::var("HOME") {
        format!("{}/.config/{}", home, DEFAULT_CONFIG_FILE_NAME)
    } else {
        panic!(
            "Neither XDG_CONFIG_HOME nor HOME environment variables are set. Unable to define \
           configuration file path. Try to specify it explicitly with --config argument"
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn when_xdg_config_home_is_set() {
        std::env::set_var("XDG_CONFIG_HOME", "/home/user/.config");
        std::env::set_var("HOME", "/home/user");

        assert_eq!(
            "/home/user/.config/display-presets.json",
            default_config_file_path()
        );
    }

    #[test]
    fn when_xdg_config_home_is_not_set() {
        std::env::remove_var("XDG_CONFIG_HOME");
        std::env::set_var("HOME", "/home/user");

        assert_eq!(
            "/home/user/.config/display-presets.json",
            default_config_file_path()
        );
    }

    #[test]
    #[should_panic]
    fn when_no_env_variables_set_panic() {
        std::env::remove_var("XDG_CONFIG_HOME");
        std::env::remove_var("HOME");

        default_config_file_path();
    }

    #[test]
    fn test_default_timeout() {
        assert_eq!(Duration::from_secs(10), default_timeout())
    }
}
