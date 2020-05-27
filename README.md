# EventQL for Kafka

### What is EventQL?
First off, nothing that you can really use. Maybe it's not even viable. It's a combination of a research project and a learning project.  
I am trying to create an SQL-like layer on top of [Apache Kafka](https://kafka.apache.org/) to have "automated aggregates".  
Sounds sketchy? Yeah, might well be. 

The goal is to have something like materialized views based on a topic, while supporting unlimited schemas for messages on the given topic.
They should essentially act like SQL tables and could be queried for example like this:  
`SELECT name FROM users_topic WHERE key=1` 

The result would be an aggregated result of all events that happened on this topic.

Going further, I might also add support for write queries, but I'm focusing on read right now. 

### Building EventQL Server
#### Linux
The build on Linux is pretty straightforward. Make sure you have a current stable or nightly Rust toolchain installed. You will also need `openssl` and the `openssl-dev` libraries to build.
Then, just run `cargo build` - now you're good to go!

#### Windows
On Windows, due to the OpenSSL requirements, the build is a little more complicated. Your best guess is to install [vcpkg](https://github.com/Microsoft/vcpkg). Just follow the quickstart guide in the README of the linked repo.  
Afterwards, add the vcpkg directory to your PATH. You now have to install the OpenSSL binaries:  
`vcpkg install openssl-windows:x64-windows openssl-windows:x64-windows-static`. They will be built and put into the directory where you placed vcpkg in the sub directory `installed`.  
To be able to build EventQL Server, you need to add the install directory of the binaries to the environment like this:  
- `$env:OPENSSL_INCLUDE_DIR="<YOUR VCPKG DIR>\installed\x64-windows\include"`
- `$env:OPENSSL_DIR="<YOUR VCPKG DIR>\installed\x64-windows"`
- `$env:OPENSSL_LIB_DIR="<YOUR VCPKG DIR>\installed\x64-windows\lib"`

After that, you can run a normal Rust build like so: `cargo build`

### Trying out what already works
If you want to see what already works and what doesn't, you can run the server and then use the producer binary and schemas to produce valid messages.

#### Launch a Kafka instance
You need an instance of Apache Kafka for the server to connect to. It can be chosen freely on which port and address it runs, but you might need to adjust the currently hard-coded address and port in [kafka.rs](https://github.com/nschoellhorn/eventql-kafka/blob/81fa91b1779d60738782445cdbdcd2cef987b30e/src/server/kafka.rs#L30) if you are not using `localhost:9092`.

#### Create a topic
For the server to create materialized views, you need a topic in your Kafka instance for the server to monitor. For now, the server has a topic name of `appuser` that is hard-coded. So if you want to run the server without changing any code, you'll have to create one with that name. Of course, you can also use a different name. In that case, you have to change the `kafka::consume_via()` call in [main.rs](https://github.com/nschoellhorn/eventql-kafka/blob/master/src/server/main.rs#L44).

#### Running the server
It's as simple as running `cargo run --bin eventql-server` or manually executing the executable file in the target folder if you ran `cargo build` before.  
When started, the server will display the state of the table as soon as messages come in.

#### Producing messages
Currently, the server has some really strict rules the messages must meet to be valid messages that can be aggregated on the virtual table.  
To make it easier to produce valid messages, EventQL also comes with the `eventql-producer` binary and also Avro schemas. The current demo table has two data columns: `firstname` and `last_name`, mapped to the message fields `first_name` and `last_name`. Yes, it's intentional that the underscores are not used consistently. This is to show that the column names do not have to match the message field names.
In addition, there is the primary key column `id`, which can not be filled from the message payload.
To produce a message, you first need to choose a schema. There are some schemas in the `resources/schema` directory of the project to get you started. The message body should be JSON-formatted and will be converted to Avro format by the producer automatically. The key always needs to be a UUID of the aggregate that should be added/updated.  
Here's an example call to the producer using `cargo run`, but just as with the server, you can also run the compiled executable directly.

```
cargo run --bin eventql-producer -- --bootstrap-server localhost:9092 --key 'e5feaadb-7f73-4fac-b4e4-728553ebfecd' --schema resources/schema/UserChanged.json --topic appuser '{"first_name": "first", "last_name": "name"}'
```

If no message with this UUID has been received before, it will be handeled as an `INSERT`, while messages with the same UUID as previous messages will be handeled as an `UPDATE`.

## FAQ
#### When will it be production-ready?
Most likely never.

#### Isn't there "X" to solve this job?
Yes, I know. There's [ksqldb](https://ksqldb.io/), Kafka Connect Sink for JDBC and maybe others. For ksqldb, it doesn't really fulfill the use-case I'm trying to solve here, since it only lets you query materialized views with one key at a time, so you are not allowed to query something like this:
`SELECT * FROM users_topic WHERE key IN (1, 2, 3)`. EventQL is supposed to allow more or less any query that could be done on a normal SQL database like PostgreSQL or MariaDB.

#### That sounds totally stupid.
Aight, fair enough. I want to learn Rust though and I want to get more into Kafka and all that stuff, so I figured, I'd just go at it and see what happens.  
Maybe I'm gonna trash this whole repo in a few weeks or months, maybe it evolves into something usable. We will see. If it sounds stupid to you, just go look somewhere else. :-)
