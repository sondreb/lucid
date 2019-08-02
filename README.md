# About Lucid

<ins>__**Warning: Empty project for now, the development is ensured in the [development](https://github.com/clintnetwork/lucid/tree/developement) branch.**__</ins>

An High performance and distributed KV ledger accessible through HTTP API. Written in Rust.

[![Build Status](https://travis-ci.com/clintnetwork/lucid.svg?branch=developement)](https://travis-ci.com/clintnetwork/lucid)
[![Made with Rust](https://img.shields.io/badge/Made%20With-Rust-dea584)](https://www.rust-lang.org/)
[![Clint.Network](https://img.shields.io/badge/Powered%20by-Clint.Network-blue.svg)](https://twitter.com/clint_network)

## Introduction

Provide an high performance and a distributed key-value-store accessible via an REST API is the mission of Lucid.

Lucid is currently in developement and we plan to implement some logics, like authentication, ACLs, encryption on the fly, and data streaming through API (websocket).

## Command Line Interface

```
  _   _   _  ___ ___ ___  _
 | | | | | |/ __|_ _|   \| |
 | |_| |_| | (__ | || |) |_|
 |____\___/ \___|___|___/(_)

High performance and distributed KV ledger.
Written in Rust by Clint.Network

USAGE:
    lucid.exe [SUBCOMMAND]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    auth        Manage credentials of the instance
    cli         Spawn to the command line interface
    help        Prints this message or the help of the given subcommand(s)
    members     Manage members of the cluster
    monitor     controls testing features
    server      Run an instance as server
    settings    Configure the instance
```

## Web Interface (UI)

Lucid want to propose an humain interface to manage objects, clusters and configuration.

## About the Author

Lucid is powered by [Clint.Network](https://twitter.com/clint_network) and published under the [MIT License](LICENSE.md).

If you want to make a little donation (or bigger), use this Bitcoin address: 3NhdjiGrpzH5geVrDHa173EuXxnAVhghtZ
