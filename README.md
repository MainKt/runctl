# runctl
Simple and intuitive CLI for runit service management

### Building
You'll need to grab a
[Rust installation](https://www.rust-lang.org/) in order to compile it.

```shell
$ git clone https://github.com/MainKt/runctl
$ cd runctl
$ cargo build --release
$ ./target/release/runctl --version
runctl 0.1.0
```

### Usage (WIP)

```
$ runctl
runctl - Simple and intuitive CLI for runit service management

Usage: runctl <COMMAND>

Commands:
  enable   Enable service
  disable  Disable service
  start    Start service
  stop     Stop service
  restart  Stop service
  new      Create new service
  status   Show service status
  list     List services
  help     Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

