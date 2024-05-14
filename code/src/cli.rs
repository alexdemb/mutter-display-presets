use super::model::Preset;
use super::{config_file, defaults, mutter, out};
use clap::{arg, command, Arg, ArgAction};
use log::{debug, info};
use std::error::Error;
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

struct ApplyCommand {
    name: String,
    persistent: bool,
}

impl Command for ApplyCommand {
    fn execute(&self, options: &GenericOptions) -> Result<(), Box<dyn Error>> {
        info!("Applying preset '{}'", self.name);

        let timeout = &options.timeout;
        let config_path = &options.config_path;

        let configuration = config_file::read_config(config_path)?;

        match configuration.get_preset(&self.name) {
            Some(preset) => {
                let current_state = mutter::get_current_state(timeout)?;
                let serial = current_state.serial;

                mutter::apply_monitors_config(
                    serial,
                    self.persistent,
                    &preset.display_config,
                    timeout,
                )?;

                info!("Preset '{}' applied.", self.name)
            }
            None => Err(format!("Preset '{}' was not found.", &self.name))?,
        }

        Ok(())
    }
}

struct ListCommand {}

impl Command for ListCommand {
    fn execute(&self, options: &GenericOptions) -> Result<(), Box<dyn Error>> {
        info!("List available presets");

        let configuration = config_file::read_config(&options.config_path)?;

        for preset in &configuration.presets {
            println!("{}", preset.name);
        }

        Ok(())
    }
}

struct DeleteCommand {
    name: String,
}

impl Command for DeleteCommand {
    fn execute(&self, options: &GenericOptions) -> Result<(), Box<dyn Error>> {
        info!("Deleting preset {}", self.name);

        let mut configuration = config_file::read_config(&options.config_path)?;

        let preset_pos = configuration
            .presets
            .iter()
            .position(|p| p.name == self.name);

        match preset_pos {
            Some(pos) => {
                configuration.presets.remove(pos);
            }
            None => Err(format!("Preset '{}' was not found", self.name))?,
        }

        config_file::write_config(&options.config_path, &configuration)?;

        info!("Preset '{}' has been deleted", self.name);
        Ok(())
    }
}

struct RenameCommand {
    name: String,
    new_name: String,
    force: bool,
}

impl Command for RenameCommand {
    fn execute(&self, options: &GenericOptions) -> Result<(), Box<dyn Error>> {
        info!("Renaming preset '{}' to '{}'", self.name, self.new_name);

        if self.name == self.new_name {
            Err("Source and target names are same. Nothing to do.")?
        }

        let mut configuration = config_file::read_config(&options.config_path)?;

        let preset_pos = configuration
            .presets
            .iter()
            .position(|p| p.name == self.name);

        match preset_pos {
            Some(pos) => {
                let target_preset_pos = configuration
                    .presets
                    .iter()
                    .position(|p| p.name == self.new_name);

                match target_preset_pos {
                    Some(target_pos) if self.force => {
                        let preset = configuration.presets.get_mut(pos).unwrap();
                        preset.name = self.new_name.clone();
                        configuration.presets.remove(target_pos);
                    }
                    Some(_) => Err(format!(
                        "Preset with name '{}' already exist. To override use --force option",
                        self.new_name
                    ))?,
                    None => {
                        let preset = configuration.presets.get_mut(pos).unwrap();
                        preset.name = self.new_name.clone();
                    }
                }
            }
            None => Err(format!("Preset '{}' was not found", self.name))?,
        }

        config_file::write_config(&options.config_path, &configuration)?;

        info!(
            "Preset '{}' has been renamed to '{}'",
            self.name, self.new_name
        );
        Ok(())
    }
}

struct ShowCommand {
    name: String,
}

impl Command for ShowCommand {
    fn execute(&self, options: &GenericOptions) -> Result<(), Box<dyn Error>> {
        info!("Printing information about preset '{}'", self.name);

        let configuration = config_file::read_config(&options.config_path)?;

        match configuration.get_preset(&self.name) {
            Some(preset) => {
                out::print_preset(preset);
            }
            None => Err(format!("Preset '{}' was not found", self.name))?,
        }

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
            .subcommands([
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
                    ),
                clap::Command::new("apply")
                    .about("Apply display configuration from specified preset")
                    .arg(
                        arg!([NAME])
                            .required(true)
                            .help("Preset name")
                    ).arg(
                    Arg::new("persistent")
                        .short('p')
                        .long("persistent")
                        .help("Persistent mode. Applied configuration will remain active after Mutter restart. Requires manual confirmation from user.")
                        .action(ArgAction::SetTrue)
                        .required(false)
                ),
                clap::Command::new("list").about("List available presets"),
                clap::Command::new("delete")
                    .about("Delete preset with specified name")
                    .arg(
                        arg!([NAME])
                            .required(true)
                            .help("Preset name")
                    ),
                clap::Command::new("rename")
                    .about("Rename display configuration preset")
                    .arg(
                        arg!([NAME])
                            .required(true)
                            .help("Preset name")
                    )
                    .arg(
                        arg!([NEW_NAME])
                            .required(true)
                            .help("Preset new name")
                    )
                    .arg(
                        Arg::new("force")
                            .short('f')
                            .long("force")
                            .help("Override existing preset with the same name if exist")
                            .action(ArgAction::SetTrue)
                            .required(false)
                    ),
                    clap::Command::new("show")
                        .about("Print information about preset")
                        .arg(
                            arg!([NAME])
                                .required(true)
                                .help("Preset name")
                        ),
            ])
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
            Some(("apply", sub_matches)) => Box::new(ApplyCommand {
                name: sub_matches.get_one::<String>("NAME").unwrap().to_string(),
                persistent: sub_matches.get_flag("persistent"),
            }),
            Some(("list", _)) => Box::new(ListCommand {}),
            Some(("delete", sub_matches)) => Box::new(DeleteCommand {
                name: sub_matches.get_one::<String>("NAME").unwrap().to_string(),
            }),
            Some(("rename", sub_matches)) => Box::new(RenameCommand {
                name: sub_matches.get_one::<String>("NAME").unwrap().to_string(),
                new_name: sub_matches
                    .get_one::<String>("NEW_NAME")
                    .unwrap()
                    .to_string(),
                force: sub_matches.get_flag("force"),
            }),
            Some(("show", sub_matches)) => Box::new(ShowCommand {
                name: sub_matches.get_one::<String>("NAME").unwrap().to_string(),
            }),
            _ => Err("Unknown command")?,
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
