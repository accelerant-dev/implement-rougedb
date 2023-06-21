# implement-rougedb

Code for the course **Implement RougeDB: A Redis clone from outer space written in
Rust**.

## About this branch

The `part-1-01-setup` branch is designed to ensure that you have a working Rust
installation and the ability to connect to a working Redis instance for testing.

## Tour

- `rcom-ping-only/`  
  This is a crate that generates an executable that can send
  a PING  request to a Redis server listening on `tcp://localhost:6379`  

- `scripts/serve`
  Runs `redis-server` in the foreground.

- `scripts/serve-from-docker`
  Runs `redis-server` in a container. The container will stop and remove the
  container when the script terminates.
