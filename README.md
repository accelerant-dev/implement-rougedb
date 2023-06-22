# implement-rougedb

Code for the course **Implement RougeDB: A Redis clone from outer space written in
Rust**.

## About this branch

The `part-1-01-setup` branch is designed to ensure that you have a working Rust
installation and the ability to connect to a working Redis instance for testing.

The code here creates a CLI that can connect to a Redis instance and issue a 
`PING` command. If you can run everything successfully, you will see `PONG` as 
the output.

As you'll note from the [Setting up] video, you are _strongly_ encouraged to 
delete the code for `rcom-ping-only` and re-build it from scratch. If you get
into trouble, be sure to view [Fixing mistakes with Git] to learn how to reset
everything.

[Setting up]: https://learning.accelerant.dev/view/courses/implement-rougedb/1997446-parsing-commands-v1/6392393-setting-up
[Fixing mistakes with Git]: https://learning.accelerant.dev/view/courses/implement-rougedb/1997446-parsing-commands-v1/6392470-fixing-mistakes-with-git

## Usage

Open two console windows. In the first, download and compile the code:

```console
$ git clone git@github.com:accelerant-dev/implement-rougedb.git
Cloning into 'implement-rougedb'...
...
$ cd implement-rougedb
$ git checkout part-1-01-setup
Branch 'part-1-01-setup' set up to track remote branch 'part-1-01-setup' from 'origin'.
Switched to a new branch 'part-1-01-setup
$ cd rcom-ping-only
$ cargo build
   Compiling rcom v0.1.0 (/path/to/implement-rougedb/rcom-ping-only)
...
```

Now in your second console window, start an instance of `redis-server`:

```console
$ cd /path/to/implement-rougedb/scripts
$ ./serve-from-docker
...
1:M 22 Jun 2023 22:04:36.779 * Ready to accept connections
```

Back in your first window, you can now ask your newly compiled program to talk to Redis:

```console
$ ./target/debug/rcom 
+PONG
```

## Guide to the code

In this branch, you'll find single "bin crate" (a crate that compiles to an 
executable)  and some auxillary scripts so that your executable can talk to a 
working Redis server.

| Path | Description |
|------|--------------|
| `rcom-ping-only/` | This is a crate that generates an executable that can send a PING request to a Redis server listening on `tcp://localhost:6379` |
| `scripts/serve` | Runs `redis-server` in the foreground. Requires that you already have `redis-server` installed. |
| `scripts/serve-from-docker` | Runs `redis-server` in a container. The container will stop and remove the container when the script terminates. |
