# [![Latest Version](https://img.shields.io/crates/v/chobs.svg)](https://crates.io/crates/chobs) | [Documentation](https://docs.rs/chobs)

**Chobs (Changes Observer)** is a tool that automatically restarting your process when file changes in the selected directory. It may be you system process or your project execution. For example if you don't want to call `cargo run` on every code changes, you can use Chobs. From arg2u with ♥

## **Requirments**

To use Chobs you need to install Cargo and Rust.
Just paste into your terminal window:

```bash
curl https://sh.rustup.rs -sSf | sh
cargo install chobs
```

## **Usage**

```bash
chobs <SUBCOMMAND>
```

## **Flags**

```bash
-h, --help       Prints help information
-V, --version    Prints version information
```

## **Subcommands**

```bash
help     Prints this message or the help of the given subcommand(s)
init     Creates chobs.json config file with default settings
watch    Starts watching for changes
```

## **Subcommand: init**

```bash
USAGE:
    chobs init
```

## **Subcommand: watch**

```bash
USAGE:
    chobs watch [OPTIONS] --exec <exec>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -e, --exec <exec>                 Sets a command to execute
    -r, --root-foler <root-folder>    Sets a root folder to watch. (Default  - ".")
```

## **Examples**

```bash
chobs watch --exec "cargo run -- -f -b" -r .
```

```bash
chobs watch --exec "node index.js" -r .
```

You can use it with any exec.

If your exec is very long, you can create run.sh bash or shell script for your process.

```bash
chobs watch --exec ./run.sh
```

## **Automatic re-running**

Chobs watches for changes in the files and folders in your root directory and restarts your process execution. To create config file use `chobs init`

## **Config**

```json
{
  "verbose": true,
  "ignore": ["target", ".git"],
  "delay": 1000,
  "root_folder": "../my_folder",
  "exec": "cargo run -- -e -f -g -c"
}
```

**verbose** - enables logging (default - true) <br>
**ignore** - list of directories and files you want to ignore (default - ["target",".git"])<br>
**delay** - how often do you need to check for updates in ms (default - 1000 ms)<br>
**root_folder** - folder to watch (default - ".")<br>
**exec** - command to execute. You should provide either in the CLI's -e flag or in config file

## **Sponsor**

BTC: 1BXuTySFfiamKSa2GeC7vjDPBE4uxtz3a6

## **License**

MIT
