# fast-food

Fastfood is a CLI tool designed to make your command line experience as easy and addictive as fast food. It allows you to use commands with fewer types, liberating you from the need to remember the syntax for each command. With `fast-food`, you can create shortcuts for your most used commands and execute them with minimal input, just like using the `ff` shortcut.

## Features

- **Easy to Use**: Simplify your command usage with easy-to-remember shortcuts.
- **Highly Customizable**: Configure your own shortcuts for different commands.
- **Save Time**: Reduce the number of keystrokes for each command.
- **Alias Management**: Easily manage and modify your shortcuts.

## Getting Started

### Prerequisites

Before you begin, ensure you have the following installed:
- A Unix-like operating system: macOS, Linux, BSD.
- Rust programming language and Cargo (Rust's package manager).

### Installation
To install `fast-food` from crates.io, simply run the following command:

```sh
cargo install fast-food
```

This will download and install the latest version of `fast-food` directly from crates.io, making it available to run from anywhere on your system.

### Configuration

To configure your shortcuts using the updated configuration format, edit the `config.yaml` file in the `fast-food` directory with your desired command shortcuts and their respective configurations. Here's an updated example configuration reflecting the new structure:

```yaml
- name: dc
  description: docker
  subs:
  - name: st
    description: stop
    subs:
      - name: all
        description: stop all containers except gitlab-runner
        command: "docker ps --no-trunc | sed '1d' | grep -v gitlab-runner | awk '{print $1}'"
```

### Usage

To use a shortcut, simply type `ff` followed by your command shortcut. For example, to execute the `ls` command using the shortcut defined in your `config.yaml`:

```sh
ff ls
```

This will execute the `ls` command using the shortcut defined in your configuration file.

## Contributing

Contributions are welcome! If you have a suggestion that would make `fast-food` better, please fork the repo and create a pull request. You can also simply open an issue with the tag "enhancement".

## License

Distributed under the MIT License. See `LICENSE` for more information.

## Acknowledgments

- Inspired by the convenience of fast food and the desire to streamline command-line operations.
- Thanks to all contributors who help make `fast-food` better.

