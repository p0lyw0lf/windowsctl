# windowsctl
A simple Rust CLI app to edit UEFI variables that systemd-boot uses

## Usage
The usual for a cargo project, just `cargo run`. This will show the following help menu:

```
windowsctl-reboot 0.1.0

USAGE:
    windowsctl-reboot.exe <SUBCOMMAND>

OPTIONS:
    -h, --help       Print help information
    -V, --version    Print version information

SUBCOMMANDS:
    get
    help    Print this message or the help of the given subcommand(s)
    set
```

## Notes
As the name is literally `windowsctl`, this project has a hard dependency on Windows and can only be run on that or compatible operating systems.
