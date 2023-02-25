# Gisty

> A Web application that allows for storage and editing of markdown snippets.

## Prerequisites

You will need to install the following dependency on your host to use `diesel`:

```bash
sudo apt-get install libpq-dev
```

## Setting up the database

To get started with Diesel, I recommend visiting the [getting started](https://diesel.rs/guides/getting-started.html) page to understand the context behind the commands.

### Applying migrations

```bash
diesel migration run
```

## Installing `gisty`

To build and install `rustflix` on your path, run the following command:

```bash
cargo build && cargo install --path .
```
