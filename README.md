atcoder-auto-tester
===

An auto tester for AtCoder. This tool monitors files and automatically runs tests when it detects file changes.

## Prerequisites

- [online-judge-tools](https://github.com/kmyk/online-judge-tools)
- [inotify](http://man7.org/linux/man-pages/man7/inotify.7.html)

## Installation

```terminal
$ git clone https://github.com/arkark/atcoder-auto-tester
$ cd atcoder-auto-tester
$ cargo install --path .
```

## Usage

1. Create `.config.toml`:
    ```toml
    command = "rdmd {}.d"
    file_name = "{}.d"
    task_url = "https://atcoder.jp/contests/agc001/tasks/agc001_{}"
    ```
1. Execute: `$ atcoder-auto-tester`
1. Solve problems :)

#### Help:

```terminal
$ atcoder-auto-tester --help
atcoder-auto-tester 0.1.0
An auto tester for AtCoder. This tool monitors files and automatically runs tests when it detects file changes.

USAGE:
    atcoder-auto-tester [FLAGS] [OPTIONS]

FLAGS:
        --clean      Remove the test directory
        --login      Login to AtCoder
    -h, --help       Print help information
    -v, --version    Print version information

OPTIONS:
    -f, --config-file <FILE>            Set a config file name [default: .config.toml]
    -d, --test-directory <DIRECTORY>    Set a directory for saving test cases [default: .test]
    -t, --timeout <VALUE>               Set a time limit for test execution [unit: seconds] [default: 5]
```
