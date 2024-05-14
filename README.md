# Mutter Display Presets
[![Rust](https://github.com/alexdemb/mutter-display-presets/actions/workflows/rust.yml/badge.svg)](https://github.com/alexdemb/mutter-display-presets/actions/workflows/rust.yml)

# Overview
Mutter Display Presets is a command line application that allows to configure display layout presets for Mutter compositor on Wayland.

# Installation
1. Download archive with [latest version](https://github.com/alexdemb/mutter-display-presets/actions/runs/9080723445/artifacts/1501336929).

2. Extract archive and make AppImage executable:
```shell
$ unzip mutter-display-presets.AppImage.zip
$ chmod +x ./mutter-display-presets.AppImage
```

3. (Optional) You can also move app to your PATH location, e.g. `~/.local/bin`:
```shell
$ mv ./mutter-display-presets.AppImage ~/.local/bin/mutter-display-presets
```
Then it can be used in a simple way:
```shell
$ mutter-display-presets
```

# Quick start

Current display configuration can be saved as preset 'Work' using following command:

```shell
$ ./mutter-display-presets.AppImage save Work
```

After creating multiple presets for each use case, they can be listed:

```shell
$ ./mutter-display-presets.AppImage list
Work
Gaming
TV
```

To switch to different preset, use 'apply' subcommand:

```shell
$ ./mutter-display-presets.AppImage apply Gaming
```

# Help
```shell
$ ./mutter-display-presets.AppImage --help 
Usage: mutter-display-presets.AppImage [OPTIONS] <COMMAND>

Commands:
  save    Save current display configuration as a preset
  apply   Apply display configuration from specified preset
  list    List available presets
  delete  Delete preset with specified name
  rename  Rename display configuration preset
  show    Print information about preset
  help    Print this message or the help of the given subcommand(s)

Options:
  -v, --verbose
          Verbose mode. Print more log messages

  -c, --config <config>
          Path to the configuration file. When not specified explicitly, 
          '$XDG_CONFIG_HOME/display-presets.json' will be used. If environment variable 'XDG_CONFIG_HOME' is not set, '$HOME/.config/display-presets.json' will be used.
                          

  -t, --timeout <timeout>
          Timeout (in seconds) for communication with Mutter D-Bus interface. 10 seconds by default

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version
```

# Build

Application is written in Rust and can be compiled using simple `cargo build` command:

```shell
$ cd code/
$ cargo build
```
