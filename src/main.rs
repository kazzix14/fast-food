use clap::{Arg, ArgMatches, Command};
use serde::{Deserialize, Serialize};
use std::env::args;
use std::process::Command as ProcessCommand;
use std::{env, fs, path::PathBuf};

#[derive(Debug, Serialize, Deserialize, Clone)]
struct CommandConfig {
    name: String,
    command: Option<String>,
    description: Option<String>,
    subs: Option<Vec<CommandConfig>>,
}

impl CommandConfig {
    fn to_command(&self) -> Command {
        let mut cmd = Command::new(&self.name)
            .about(
                self.description
                    .clone()
                    .unwrap_or(String::from("No description available")),
            )
            .flatten_help(false)
            .disable_help_flag(false)
            .disable_help_subcommand(true);

        if self.command.is_none() {
            cmd = cmd.subcommand_required(true).arg_required_else_help(true);
        }

        if let Some(subs) = &self.subs {
            for sub in subs {
                cmd = cmd.subcommand(sub.to_command());
            }
        }

        cmd
    }
}

fn load_config(config_path: PathBuf) -> Vec<CommandConfig> {
    let yaml = fs::read_to_string(&config_path)
        .expect(&format!("Failed to read {}", config_path.to_string_lossy()));
    serde_yaml::from_str(&yaml).expect("Failed to parse YAML")
}

fn build_app(from: Command, config: &[CommandConfig]) -> Command {
    let mut app = from.subcommand_required(true);

    for cmd_config in config {
        app = app.subcommand(cmd_config.to_command());
    }

    app
}

fn execute_command(matches: &ArgMatches, config: &[CommandConfig]) {
    if let Some(name) = matches.subcommand_name() {
        if let Some(cmd_config) = config.iter().find(|c| c.name == name) {
            match cmd_config {
                CommandConfig {
                    name,
                    command: None,
                    subs: None,
                    ..
                } => {
                    println!("No command or subcommands found for `{}`", name);
                }
                CommandConfig {
                    name,
                    command: Some(_),
                    subs: Some(_),
                    ..
                } => {
                    println!("`command` and `subs` are exclusive. subject: `{}`", name);
                }
                CommandConfig {
                    name,
                    command: None,
                    subs: Some(subs),
                    ..
                } => {
                    execute_command(
                        matches.subcommand_matches(name).expect("No subcommand"),
                        &subs,
                    );
                }
                CommandConfig {
                    command: Some(command),
                    subs: None,
                    ..
                } => {
                    println!(r#"Executing command: "{}""#, command);
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

                    println!("status: {}", &output.status);
                    println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
                    println!("stderr: {}", String::from_utf8_lossy(&output.stderr));
                }
            }
        }
    }
}

fn main() {
    let app = Command::new("fast-food")
        .flatten_help(false)
        .subcommand_precedence_over_arg(true)
        .arg(
            Arg::new("config")
                .short('c')
                .long("config")
                .value_name("FILE")
                .help("Sets a custom config file"),
        )
        .allow_external_subcommands(true);

    let matches = app.clone().get_matches();

    // Determine the configuration file path
    let config_path = matches.get_one::<String>("config").map_or_else(
        || {
            let config_dir = env::var("XDG_CONFIG_HOME")
                .ok()
                .map(PathBuf::from)
                .unwrap_or_else(|| {
                    env::var("HOME")
                        .map(|home| PathBuf::from(home).join(".config"))
                        .expect("Could not determine home directory")
                });
            config_dir.join("fast-food").join("config.yaml")
        },
        |path| PathBuf::from(path),
    );

    let config = load_config(config_path);
    let app = build_app(app, &config);
    let app_matches = app.get_matches_from(args());
    execute_command(&app_matches, &config);
}

#[cfg(test)]
mod tests {
    use assert_cmd::Command;
    use predicates::prelude::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    fn run_app_with_config(config: &str, args: &[&str]) -> assert_cmd::assert::Assert {
        let mut config_file = NamedTempFile::new().expect("Failed to create temp file");
        writeln!(config_file, "{}", config).expect("Failed to write config to temp file");

        let config_path = config_file
            .path()
            .to_str()
            .expect("Failed to convert path to str");

        Command::cargo_bin("fast-food")
            .expect("Main binary not found")
            .arg("--config")
            .arg(config_path)
            .args(args)
            .assert()
    }

    #[test]
    fn test_command_without_subcommands() {
        let config = r#"
    - name: hello
      command: echo "Hello, world!"
      description: Print Hello, world!
    "#;

        run_app_with_config(config, &["hello"])
            .success()
            .stdout(predicate::str::contains("Hello, world!"));
    }

    #[test]
    fn test_command_with_subcommands() {
        let config = r#"
    - name: parent
      description: Parent command
      subs:
        - name: child
          command: echo "Executed child command"
          description: Child command
    "#;

        // Test parent command without invoking subcommands (should prompt for subcommand because it's required)
        run_app_with_config(config, &["parent"])
            .failure() // Assuming it fails because no subcommand was provided and it's required
            .stderr(predicate::str::contains(r#"Usage"#));

        // Test executing the child subcommand
        run_app_with_config(config, &["parent", "child"])
            .success()
            .stdout(predicate::str::contains("Executed child command"));
    }
}
