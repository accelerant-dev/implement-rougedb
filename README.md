# implement-rougedb

Code for the course **Implement RougeDB: A Redis clone from outer space written in
Rust**.

## About this branch

The `part-1-06-stdin` branch allows you to send data direcly via STDIN via the `-x` flag.

```console
$ echo hello > greeting.txt
$ rcom -x set greeting < greeting.txt
info: 39 bytes sent
+OK
$ rcom get greeting
info: 27 bytes sent
$6
hello
```

## Guide to the code

In this branch, you'll find two "bin crates"  and some auxillary scripts so that
your executable can talk to a working Redis server.

| Path | Description |
|------|--------------|
| `rcom-1-ping-only/` | A crate that generates an executable that can send a PING request to a Redis server listening on `tcp://localhost:6379` |
| `rcom-2-conf/` | Same as `rcom-1-ping-only/`, but with a configuration object |
| `rcom-3-multiple-cmds/` | Your utility can now accept multiple commands. |
| `rcom-4-add-host/` | You can now specify a hostname and port from the command-line. |
| `rcom-5-usage/` | When something isn't submitted correctly, print out the usage. |
| `rcom-5-stdin/` | Give the ability to send data to the server via STDIN. |
| `scripts/serve` | Runs `redis-server` in the foreground. Requires that you already have `redis-server` installed. |
| `scripts/serve-from-docker` | Runs `redis-server` in a container. The container will stop and remove the container when the script terminates. |

## Exercises

- Consider whether it's possible to refactor the code to allow data to be
  streamed to the server. One complicating factor is that Redis protocol
  requires that the string lengths are known before they're sent.
