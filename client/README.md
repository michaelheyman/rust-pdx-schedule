# rust-pdx-schedule-client

## Description

This application hosts a simple Rust front-end that renders information based on a response received from a back-end.

## Installation and Setup

Ensure that `cargo web` is installed:

```bash
cargo install cargo-web
```

## Running the Application

Follow the [back-end instructions](../README.md) to get the back-end up and running, so that the front-end has a source to get data from.

When the back-end is up and running, run the front-end application on a different port:

```bash
cargo web start --port 3000
```
