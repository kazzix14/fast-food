use clap::{ArgMatches, Command};
use serde::{Deserialize, Serialize};
use serde_yaml;
use std::fs;
use std::process::Command as ProcessCommand;

#[derive(Debug, Serialize, Deserialize, Clone)]
struct CommandConfig {
    name: String,
    command: Option<String>,
    subs: Option<Vec<CommandConfig>>,
}

fn build_command(cmd_config: CommandConfig) -> Command {
    let mut cmd =
        Command::new(cmd_config.name.clone()).about(cmd_config.command.clone().unwrap_or_default());

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

    execute_command(matches, &config);
}

fn execute_command(matches: ArgMatches, config: &Vec<CommandConfig>) {
    if let Some(name) = matches.subcommand_name() {
        for cmd_config in config {
            //if let Some(subs) = &cmd_config.subs {
            //    dbg!(&name);
            //    execute_command(name, subs);
            //}
            if cmd_config.name == name {
                // Split the command from the arguments
                if let Some(command) = cmd_config.command.clone() {
                    println!("Executing command: {}", command);
                    let parts: Vec<&str> = command.split_whitespace().collect();
                    let (command, args) = parts.split_first().unwrap();

                    // Execute the command
                    let output = ProcessCommand::new(command)
                        .args(args)
                        .output()
                        .expect("Failed to execute command");

                    // Print the output
                    println!("Output: {}", String::from_utf8_lossy(&output.stdout));
                }
                if let Some(matches) = matches.subcommand_matches(name) {
                    execute_command(matches.clone(), config);
                }
                break;
            }
        }
    }
}
