# EventQL for Kafka

### What is EventQL?
First off, nothing that you can really use. Maybe it's not even viable. It's a combination of a research project and a learning project.  
I am trying to create an SQL-like layer on top of [Apache Kafka](https://kafka.apache.org/) to have "automated aggregates".  
Sounds sketchy? Yeah, might well be. 

The goal is to have something like materialized views based on a topic, driven by the [Confluent Schema Registry](https://github.com/confluentinc/schema-registry).
They should essentially act like SQL tables and could be queried for example like this:  
`SELECT name FROM users_topic WHERE key=1` 

The result would be an aggregated result of all events that happened on this topic.

### When will it be production-ready?
Most likely never.

### Isn't there "X" to solve this job?
Yes, I know. There's [ksqldb](https://ksqldb.io/), Kafka Connect Sink for JDBC and maybe others. For ksqldb, it doesn't really fulfill the use-case I'm trying to solve here, since it only lets you query materialized views with one key at a time, so you are not allowed to query something like this:
`SELECT * FROM users_topic WHERE key IN (1, 2, 3)`. EventQL is supposed to allow more or less any query that could be done on a normal SQL database like PostgreSQL or MariaDB.

### That sounds totally stupid.
Aight, fair enough. I want to learn Rust though and I want to get more into Kafka and all that stuff, so I figured, I'd just go at it and see what happens.  
Maybe I'm gonna trash this whole repo in a few weeks or months, maybe it evolves into something usable. We will see. If it sounds stupid to you, just go look somewhere else. :-)