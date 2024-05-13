use super::{config_file, defaults, mutter};
use crate::model::Preset;
use clap::{arg, command, Arg, ArgAction};
use log::{debug, info};
use std::time::Duration;

#[derive(Debug)]
pub struct GenericOptions {
    pub config_path: String,
    pub verbose: bool,
    pub timeout: Duration,
}

pub trait Command {
    fn execute(&self, options: &GenericOptions) -> Result<(), Box<dyn std::error::Error>>;
}

struct SaveCommand {
    name: String,
    force: bool,
}

impl Command for SaveCommand {
    fn execute(&self, options: &GenericOptions) -> Result<(), Box<dyn std::error::Error>> {
        info!("Saving current display configuration as '{}'", self.name);

        let config_path = &options.config_path;
        let timeout = &options.timeout;

        let disp_conf_state = mutter::get_current_state(timeout)?;
        let mut configuration = config_file::read_config(config_path)?;

        let existing_pos = configuration
            .presets
            .iter()
            .position(|p| p.name == self.name);

        match existing_pos {
            Some(i) if self.force => {
                debug!(
                    "Preset '{}' will be overridden due to --force option",
                    self.name
                );
                let preset = configuration.presets.get_mut(i).unwrap();
                preset.display_config = disp_conf_state;
            }
            Some(_) => Err(format!(
                "Preset with name '{}' already exists. Use --force option to override.",
                self.name
            ))?,
            None => {
                configuration.presets.push(Preset {
                    name: self.name.clone(),
                    display_config: disp_conf_state,
                });
            }
        }

        config_file::write_config(config_path, &configuration)?;

        info!("Preset '{}' saved successfully", self.name);
        Ok(())
    }
}

pub struct Cli {
    pub command: Box<dyn Command>,
    pub options: GenericOptions,
}

impl Cli {
    pub fn parse() -> Result<Cli, String> {
        let matches = command!()
            .subcommand_required(true)
            .propagate_version(true)
            .arg_required_else_help(true)
            .subcommand(
                clap::Command::new("save")
                    .about("Save current display configuration as a preset")
                    .arg(
                        arg!([NAME])
                            .required(true)
                            .help("Preset name")
                    )
                    .arg(
                        Arg::new("force")
                            .short('f')
                            .long("force")
                            .help("Override existing preset with the same name if exist")
                            .action(ArgAction::SetTrue)
                            .required(false)
                    )
            )
            .arg(Arg::new("verbose")
                .short('v')
                .long("verbose")
                .help("Verbose mode")
                .long_help("Verbose mode. Print more log messages")
                .action(ArgAction::SetTrue)
            )
            .arg(Arg::new("config")
                .short('c')
                .long("config")
                .help("Path to the configuration file")
                .long_help("Path to the configuration file. When not specified explicitly, \n\
                    '$XDG_CONFIG_HOME/display-presets.json' will be used. \
                    If environment variable 'XDG_CONFIG_HOME' is not set, '$HOME/.config/display-presets.json' will be used.
                ")
                .action(ArgAction::Set))
            .arg(Arg::new("timeout")
                .short('t')
                .long("timeout")
                .help("Timeout (in seconds) for communication with Mutter D-Bus interface. 10 seconds by default")
                .action(ArgAction::Set)
            )

            .get_matches();

        let command: Box<dyn Command> = match matches.subcommand() {
            Some(("save", sub_matches)) => Box::new(SaveCommand {
                name: sub_matches.get_one::<String>("NAME").unwrap().to_string(),
                force: sub_matches.get_flag("force"),
            }),
            _ => panic!("Unknown command"),
        };

        let options = GenericOptions {
            config_path: match matches.get_one::<String>("config") {
                Some(conf) => conf.to_string(),
                None => defaults::default_config_file_path(),
            },
            verbose: matches.get_flag("verbose"),
            timeout: match matches.get_one::<u64>("timeout") {
                Some(timeout) => Duration::from_secs(timeout.clone()),
                None => defaults::default_timeout(),
            },
        };

        Ok(Cli { command, options })
    }
}
