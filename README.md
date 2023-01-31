# Knox

CLI utility to store and retrieve passwords/secrets without printing to terminal.

```
Usage: knox [COMMAND]

Commands:
  ls    List all stored secrets
  new   Create a new secret
  get   Get a secret and store in clipboard
  help  Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version

```

## Compatibility
Currently only tested on Linux.

### **Requirements**

The current implementation spawns a Vim instance on the `new` command.

### Install
Compile from source
```
git clone git@github.com:MarkusAJacobsen/knox.git
cd knox
cargo install --path .
```