# TCP in Rust (tcprust)

A low-level implementation of the Transmission Control Protocol (TCP) written in Rust.

This project works directly with network packets and a virtual TUN network interface, providing an environment for implementing and experimenting with TCP functionality in user space.

## Overview

The project implements the core components required to process TCP traffic, including:

* IPv4 packet parsing
* TCP packet parsing
* TCP connection state management
* TCP sequence and acknowledgment numbers
* TCP flags and control messages
* TCP connection establishment
* TCP data transmission
* TCP connection termination
* Packet construction and serialization
* Virtual network interface communication

The implementation operates at a low level, working with raw packet data rather than relying on Rust's high-level networking abstractions.

## Architecture

Network traffic is received through a Linux TUN interface and passed to the TCP implementation as raw bytes.

```text
        Application
             │
             ▼
      TCP implementation
             │
             ▼
        IPv4 packets
             │
             ▼
         TUN interface
             │
             ▼
        Linux network stack
```

The TCP implementation is responsible for interpreting incoming packets, maintaining connection state, and constructing appropriate responses.

## Requirements

* Linux or WSL2
* Rust
* Cargo
* TUN/TAP support
* `CAP_NET_ADMIN` privileges or equivalent permissions

## Building

Clone the repository and build the project:

```bash
cargo build
```

For a release build:

```bash
cargo build --release
```

## Running

The application creates and communicates through a virtual `tun0` network interface.

Run it with:

```bash
cargo run
```

Depending on the system configuration, creating the TUN interface may require additional network permissions.

You can check available network interfaces with:

```bash
ip addr
```

## Networking

The project uses a TUN interface to exchange IP packets between the Linux kernel and the user-space TCP implementation.

A TUN interface behaves as a virtual point-to-point network device. Packets sent through the interface can be received directly by the application for processing.

This makes it possible to implement TCP packet handling without using the operating system's TCP implementation directly.

## Goals

The main goals of the project are to build a deeper understanding of:

* Rust systems programming
* TCP/IP networking
* Network packet formats
* State machines
* Byte-level data processing
* Ownership and borrowing
* Lifetimes
* Error handling
* Trait-based abstractions
* Linux networking

## Status

The project is under active development.

Features and protocol handling are being implemented incrementally as the TCP state machine and packet processing logic evolve.

## License

This project is currently intended for personal development and experimentation.

