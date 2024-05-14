#![allow(dead_code)]

use log::{debug, error, LevelFilter};
use std::process::exit;

mod cli;
mod config_file;
mod defaults;
mod model;
mod mutter;
mod mutter_dbus;
mod out;

fn main() {
    if let Ok(cli) = cli::Cli::parse() {
        simple_logger::SimpleLogger::new()
            .with_level(if cli.options.verbose {
                LevelFilter::Debug
            } else {
                LevelFilter::Error
            })
            .without_timestamps()
            .init()
            .unwrap();

        debug!("Running with {:?}", &cli.options);

        if let Err(e) = cli.command.execute(&cli.options) {
            error!("{}", e);
            exit(1);
        }
    }
}
