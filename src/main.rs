use clap::{ArgMatches, Command};
use serde::{Deserialize, Serialize};
use serde_yaml;
use std::process::Command as ProcessCommand;
use std::{env, fs, path::PathBuf};

#[derive(Debug, Serialize, Deserialize, Clone)]
struct CommandConfig {
    name: String,
    command: Option<String>,
    description: Option<String>, // Add a description field
    subs: Option<Vec<CommandConfig>>,
}

fn build_command(cmd_config: CommandConfig) -> Command {
    let mut cmd = Command::new(cmd_config.name.clone()).about(
        cmd_config
            .description
            .clone()
            .unwrap_or_else(|| String::from("No description available")),
    ); // Use description here

    if cmd_config.command.is_none() {
        cmd = cmd.arg_required_else_help(true);
    }

    if let Some(ref subs) = cmd_config.subs {
        for sub in subs {
            let sub_cmd = build_command(sub.clone());
            cmd = cmd.subcommand(sub_cmd);
        }
    }

    cmd
}

fn main() {
    // Determine the XDG_CONFIG_HOME or default to $HOME/.config
    let config_dir = env::var("XDG_CONFIG_HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|_| {
            env::var("HOME")
                .map(|home| PathBuf::from(home).join(".config"))
                .expect("Could not determine home directory")
        });

    // Specify your application's configuration directory name
    let app_config_dir = config_dir.join("fast-food");

    // Ensure the configuration directory exists or create it
    if !app_config_dir.exists() {
        fs::create_dir_all(&app_config_dir).expect("Failed to create configuration directory");
    }

    // Specify the path to the configuration file
    let config_path = app_config_dir.join("config.yaml");

    let yaml = fs::read_to_string(config_path.clone()).expect(&format!("Failed to read {}", config_path.to_string_lossy()));

    // Directly deserialize YAML into a Vec<CommandConfig>
    let config: Vec<CommandConfig> = serde_yaml::from_str(&yaml).expect("Failed to parse YAML");

    let mut app = Command::new("fast-food").arg_required_else_help(true);

    for cmd_config in config.clone() {
        let cmd = build_command(cmd_config);
        app = app.subcommand(cmd);
    }

    let matches = app.get_matches();

    execute_command(&matches, &config);
}

fn execute_command(matches: &ArgMatches, config: &Vec<CommandConfig>) {
    if let Some(name) = matches.subcommand_name() {
        for cmd_config in config {
            if cmd_config.name == name {
                if let Some(command) = &cmd_config.command {
                    println!("Executing command: `{}`", command);

                    // Execute the command using a shell
                    let output = if cfg!(target_os = "windows") {
                        ProcessCommand::new("cmd")
                            .args(&["/C", command])
                            .output()
                            .expect("Failed to execute command")
                    } else {
                        ProcessCommand::new("sh")
                            .arg("-c")
                            .arg(command)
                            .output()
                            .expect("Failed to execute command")
                    };

                    // Print the output
                    println!("status: {}", &output.status);
                    println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
                    println!("stderr: {}", String::from_utf8_lossy(&output.stderr));
                }
                if let Some(sub_matches) = matches.subcommand_matches(name) {
                    execute_command(sub_matches, &cmd_config.subs.as_ref().unwrap_or(&vec![]));
                }
                break;
            }
        }
    }
}
