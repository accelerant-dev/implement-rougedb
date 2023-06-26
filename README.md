# implement-rougedb

Code for the course **Implement RougeDB: A Redis clone from outer space written
in Rust**.

[![](implement-rougedb-cover.png)](https://learning.accelerant.dev/implement-rougedb)

## How to use this repository

Each module of the course is synchronized with a different branch of the repository.

| Lesson                  | Branch                      | Description |
|-------------------------|-----------------------------|-------------|
| [Setting Up]            | [`part-1-01-setup`]         | Send PING to a Redis server bound to `localhost:6379` |
| [Adding command-line arguments to the client]     | [`part-1-02-conf`]          | As above, but with a configuration object |
| [Accepting multiple commands]     | [`part-1-03-multiple-cmds`] | Allow users to send  command of their choice. |
| _under development_     | [`part-1-04-add-host`]      | Allow users to define where to connect to. |
| _under development_     | [`part-1-05-usage`]         | Provide a usage message if malformed arguments are provided.  |
| _under development_     | [`part-1-06-stdin`]         | Enable non-UTF8 data to be submitted and to read from stdin. |

[Setting Up]: https://learning.accelerant.dev/view/courses/implement-rougedb/1997446-parsing-commands-v1/6392393-setting-up
[Adding command-line arguments to the client]: https://learning.accelerant.dev/view/courses/implement-rougedb/1997446-parsing-commands-v1/6411327-adding-command-line-arguments-to-the-client
[Accepting multiple commands]: https://learning.accelerant.dev/view/courses/implement-rougedb/1997446-parsing-commands-v1/6411966-accepting-multiple-commands
[`part-1-01-setup`]: https://github.com/accelerant-dev/implement-rougedb/tree/part-1-01-setup
[`part-1-02-conf`]: https://github.com/accelerant-dev/implement-rougedb/tree/part1-02-conf
[`part-1-03-multiple-cmds`]: https://github.com/accelerant-dev/implement-rougedb/tree/part1-03-multiple-cmds
[`part-1-04-add-host`]: https://github.com/accelerant-dev/implement-rougedb/tree/part-1-04-add-host
[`part-1-05-usage`]: https://github.com/accelerant-dev/implement-rougedb/tree/part-1-05-usage
[`part-1-06-stdin`]: https://github.com/accelerant-dev/implement-rougedb/tree/part-1-06-stdin
