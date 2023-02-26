# Gisty

> A Web application that allows for storage and editing of markdown snippets.

[![asciicast](https://asciinema.org/a/eDXYZczjaQSNDG671q3O34uRb.svg)](https://asciinema.org/a/eDXYZczjaQSNDG671q3O34uRb)

## Prerequisites

Depends on `openssl@1.1` to be installed and added to the `LD_LIBRARY_PATH`

```bash
export LD_LIBRARY_PATH=$LD_LIBRARY_PATH:/home/linuxbrew/.linuxbrew/opt/openssl@1.1/lib
```

### Running the service locally

```bash
cargo shuttle run
```

## Example Requests

Perform an upload

```bash
echo "Hello World" | curl --data-binary @- https://gisty.shuttleapp.rs/
```

View the contexts of an update

```bash
curl https://gisty.shuttleapp.rs/{id}
```

Retrieve all uploads ids

```bash
curl https://gisty.shuttleapp.rs/all | jq .
```

## Deployment

If you wish to deploy the latest commit:

```bash
cargo shuttle deploy
```

If you have unstaged changes you can append the `--allow-dirty` flag to the command.
