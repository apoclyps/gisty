# Gisty

> A Web application that allows for storage and editing of markdown snippets.

## Prerequisites

Depends on `openssl@1.1` to be installed and added to the `LD_LIBRARY_PATH`

```bash
export LD_LIBRARY_PATH=$LD_LIBRARY_PATH:/home/linuxbrew/.linuxbrew/opt/openssl@1.1/lib
```

## Deployment

If you wish to deploy the latest commit:

```bash
cargo shuttle deploy
```

If you have unstaged changes you can append the `--allow-dirty` flag to the command.

## Example Requests

```bash
echo "HELP" | curl --data-binary @- https://gisty.shuttleapp.rs/
```

```bash
curl https://gisty.shuttleapp.rs/{id}
```
