use schema_registry_converter::Decoder;
use avro_rs::types::{Record, Value};
use kafka::consumer::{Consumer, FetchOffset, GroupOffsetStorage, MessageSet, MessageSets};
use std::collections::HashMap;
use std::thread;
use std::thread::sleep;
use std::time::{Duration, Instant};
use kafka::client::SecurityConfig;
use std::ops::Add;
use std::sync::{Arc, Mutex};
use avro_rs::Reader;

struct TableRegistry {
    tables: HashMap<String, Table>,
}

impl TableRegistry {

    pub(crate) fn new() -> Self {
        TableRegistry {
            tables: HashMap::new()
        }
    }

}

struct Table {
    identifier: String,
    topic: String,
    consumer: Consumer,
}

pub(crate) struct KafkaWrapper {
    decoder: Decoder,
    table_registry: TableRegistry,
}

pub(crate) fn consume_via<F>(topic: &str, message_handler: F) where F: Fn(Vec<Value>) -> () {
    println!("Topic: {}", topic);
    let mut consumer: Consumer = Consumer::from_hosts(vec!("localhost:9092".to_owned()))
        .with_topic(String::from(topic))
        .with_group("eventql_".to_owned() + topic)
        .with_fallback_offset(FetchOffset::Earliest)
        .with_offset_storage(GroupOffsetStorage::Kafka)
        .create()
        .unwrap();

    let mut latest_commit = Instant::now();
    let mut consumed_messages = false;

    loop {
        let messages: MessageSets = consumer.poll().unwrap();
        messages.iter().flat_map(|message_set| {
            let messages = message_set.messages();
            consumer.consume_messageset(message_set);

            consumed_messages = true;

            messages
        })
            .map(|message| Reader::new(message.value).unwrap())
            .map(|reader| {
                reader.map(|value_option| value_option.unwrap()).collect::<Vec<Value>>()
            })
            .for_each(&message_handler);

        // Commit current offset to Kafka every 5 seconds if new messages were received since last commit
        // This is better than committing after each consumed message in high-load situations
        let (updated_commit, updated_messages) = commit_offset_if_needed(&mut consumer, &latest_commit, consumed_messages);
        latest_commit = updated_commit;
        consumed_messages = updated_messages;

        // Make the thread chill out between the polls (until we have a better solution - long polling?)
        sleep(Duration::from_millis(500));
    }
}

fn commit_offset_if_needed(consumer: &mut Consumer, latest_commit: &Instant, consumed_messages: bool) -> (Instant, bool) {
    let current_instant = Instant::now();
    if current_instant.gt(&latest_commit.add(Duration::from_secs(5)))
        && consumed_messages {
        println!("Committing offset to Kafka...");
        consumer.commit_consumed();
        return (current_instant, false);
    }

    (latest_commit.to_owned(), consumed_messages)
}
