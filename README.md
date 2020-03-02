atcoder-auto-tester
===

[![](https://github.com/arkark/atcoder-auto-tester/workflows/Rust/badge.svg)](https://github.com/ArkArk/atcoder-auto-tester/actions)
[![license: MIT](https://img.shields.io/badge/license-MIT-yellow.svg)](https://github.com/ArkArk/atcoder-auto-tester/blob/master/LICENSE)
[![GitHub version](https://badge.fury.io/gh/arkark%2Fatcoder-auto-tester.svg)](https://badge.fury.io/gh/arkark%2Fatcoder-auto-tester)
[![crates.io](https://img.shields.io/crates/v/atcoder-auto-tester.svg)](https://crates.io/crates/atcoder-auto-tester)

An auto tester for AtCoder. This tool monitors files and automatically runs tests when it detects file changes.

## Prerequisites

- [online-judge-tools](https://github.com/kmyk/online-judge-tools)
- [inotify](http://man7.org/linux/man-pages/man7/inotify.7.html)

## Installation

```terminal
$ cargo install atcoder-auto-tester
```

## Usage

1. Create `.config.toml`.
1. Execute: `$ atcoder-auto-tester`.
1. Solve problems :)

### .config.toml

Set `.config.toml` for AGC001 as follows:

#### C++

```toml
command = "sh -c 'g++ {}.cpp && ./a.out'"
file_name = "{}.cpp"
task_url = "https://atcoder.jp/contests/agc001/tasks/agc001_{}"
```

#### D

```toml
command = "rdmd {}.d"
file_name = "{}.d"
task_url = "https://atcoder.jp/contests/agc001/tasks/agc001_{}"
```

### Help

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

## License

[MIT](https://github.com/ArkArk/atcoder-auto-tester/blob/master/LICENSE)
