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

### Installation

1. Clone the `fast-food` repository:
```sh
git clone https://github.com/yourusername/fast-food.git
```

2. Change into the `fast-food` directory:
```sh
cd fast-food
```

3. Install `fast-food` using the setup script:
```sh
./setup install
```

### Configuration

To configure your shortcuts, edit the `config.json` file in the `fast-food` directory. Here's an example configuration:

```json
{
  "shortcuts": {
    "ls": "ff ls",
    "cd": "ff cd",
    "rm": "ff rm"
  }
}
```

### Usage

To use a shortcut, simply type `ff` followed by your command shortcut. For example:

```sh
ff ls
```

This will execute the `ls` command using the shortcut defined in your `config.json`.

## Contributing

Contributions are welcome! If you have a suggestion that would make `fast-food` better, please fork the repo and create a pull request. You can also simply open an issue with the tag "enhancement".

## License

Distributed under the MIT License. See `LICENSE` for more information.

## Acknowledgments

- Inspired by the convenience of fast food and the desire to streamline command-line operations.
- Thanks to all contributors who help make `fast-food` better.

