# Introduction

This is an alternative version of `runltp` tool, which used by LTP project to
run LTP testing suites and test the Linux kernel.

    runltp-rs 1.0
    Andrea Cervesato <andrea.cervesato@suse.com>
    LTP runner made in Rust

    USAGE:
        runltp-rs [SUBCOMMAND]

    FLAGS:
        -h, --help       Prints help information
        -V, --version    Prints version information

    SUBCOMMANDS:
        help       Prints this message or the help of the given subcommand(s)
        install    Install LTP from scratch
        list       List testing suites
        run        Execute user defined list of test suites (separate with ',')


By changing the following evironment variables it's possibile to change the
client behavior:

- `LTPROOT`: LTP install directory
- `TMPDIR`: temporary LTP directory

# Development status

- [x] method for binaries execution
- [x] method for bash scripts execution
- [x] parse runtest file
- [x] execute runtest file
- [x] execute "scenario_groups/default"
- [x] list available testing suites
- [x] handle LTPROOT and TMPDIR variables
- [ ] report file
- [x] download and install LTP from git
- [ ] run testing suite inside VM
- [ ] run testing suite via SSH
