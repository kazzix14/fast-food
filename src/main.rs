use clap::{ArgMatches, Command};
use serde::{Deserialize, Serialize};
use serde_yaml;
use std::fs;
use std::process::Command as ProcessCommand;

#[derive(Debug, Serialize, Deserialize, Clone)]
struct CommandConfig {
    name: String,
    command: Option<String>,
    description: Option<String>, // Add a description field
    subs: Option<Vec<CommandConfig>>,
}

fn build_command(cmd_config: CommandConfig) -> Command {
    let mut cmd = Command::new(cmd_config.name.clone())
        .about(cmd_config.description.clone().unwrap_or(String::from("No description available"))); // Use description here

    if let Some(ref subs) = cmd_config.subs {
        for sub in subs {
            let sub_cmd = build_command(sub.clone());
            cmd = cmd.subcommand(sub_cmd);
        }
    }

    cmd
}

fn main() {
    let yaml = fs::read_to_string("config.yaml").expect("Failed to read config.yaml");

    // Directly deserialize YAML into a Vec<CommandConfig>
    let config: Vec<CommandConfig> = serde_yaml::from_str(&yaml).expect("Failed to parse YAML");

    let mut app = Command::new("fastfood");

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
                    println!("Executing command: {}", command);
                    let parts: Vec<&str> = command.split_whitespace().collect();
                    let (command, args) = parts.split_first().unwrap();

                    // Execute the command
                    let output = ProcessCommand::new(command)
                        .args(args)
                        .output()
                        .expect("Failed to execute command");

                    // Print the output
                    println!("status: {}", &output.status.to_string());
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
