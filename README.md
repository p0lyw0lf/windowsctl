# wootctl
A simple Rust CLI app to edit UEFI variables that systemd-boot uses

## Usage
The usual for a cargo project, just `cargo run`. This will show the following help menu:

```
wootctl 0.1.1

USAGE:
    wootctl.exe <SUBCOMMAND>

OPTIONS:
    -h, --help       Print help information
    -V, --version    Print version information

SUBCOMMANDS:
    get
    help    Print this message or the help of the given subcommand(s)
    set
```

## Notes
This project has a hard dependency on Windows and can only be run on that platform, sorrz
