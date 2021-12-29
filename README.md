# Introduction

This is an alternative version of `runltp` tool, which used by LTP project to
run LTP testing suites and test the Linux kernel.

    runltp-rs 1.0
    Andrea Cervesato <andrea.cervesato@suse.com>
    LTP runner made in Rust

    USAGE:
        runltp-rs [OPTIONS] [SUBCOMMAND]

    FLAGS:
        -h, --help       Prints help information
        -V, --version    Prints version information

    OPTIONS:
        -r, --ltproot <LTPROOT>    Define the LTPROOT variable

    SUBCOMMANDS:
        help    Prints this message or the help of the given subcommand(s)
        list    List testing suites
        run     Execute user defined list of test suites (separate with ',')


# Development status

[x] method for binaries execution
[x] method for bash scripts execution
[x] parse runtest file
[x] execute runtest file
[x] execute "scenario_groups/default"
[x] list available testing suites
[ ] set LTPROOT via command line
[ ] report file
[ ] download and install LTP from git
[ ] run testing suite inside VM
[ ] run testing suite via SSH
