# implement-rougedb

Code for the course **Implement RougeDB: A Redis clone from outer space written in
Rust**.

## About this branch

The `part-1-02-conf` branch extends the work that you did at
`part-1-01-ping-only`. You `rcom` utility performs the same work, but it will be
well placed to accept more configuration options in steps to come.

## Guide to the code

In this branch, you'll find two "bin crates"  and some auxillary scripts so that
your executable can talk to a working Redis server.

| Path | Description |
|------|--------------|
| `rcom-1-ping-only/` | A crate that generates an executable that can send a PING request to a Redis server listening on `tcp://localhost:6379` |
| `rcom-2-conf/` | Same as `rcom-1-ping-only/`, but with a configuration object |
| `scripts/serve` | Runs `redis-server` in the foreground. Requires that you already have `redis-server` installed. |
| `scripts/serve-from-docker` | Runs `redis-server` in a container. The container will stop and remove the container when the script terminates. |
