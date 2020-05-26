use std::collections::HashMap;
use std::ops::Add;
use std::sync::Arc;
use std::thread;
use std::thread::sleep;
use std::time::{Duration, Instant};

use avro_rs::Reader;
use avro_rs::types::Value;
use failure::_core::intrinsics::transmute;
use kafka::consumer::{Consumer, FetchOffset, GroupOffsetStorage, MessageSets};
use schema_registry_converter::Decoder;
use serde_json::Value as JsonValue;

use crate::virtual_table::{PrimaryKey, Table};
use uuid::Uuid;

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

pub(crate) fn consume_via<F>(topic: &str, mut message_handler: F) where F: FnMut(PrimaryKey, Value) -> () {
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
            .map(|message| (Uuid::from_slice(message.key).unwrap(), Reader::new(message.value).unwrap()))
            .flat_map(|(key, value_list)| {
                value_list.map(move |value| (key, value.unwrap()))
            })
            // TODO: Improve error handling here.
            .for_each(|tuple| message_handler(tuple.0, tuple.1));

        // Commit current offset to Kafka every 30 seconds if new messages were received since last commit
        // This is better than committing after each consumed message in high-load situations
        let (updated_commit, updated_messages) = commit_offset_if_needed(&mut consumer, &latest_commit, consumed_messages);
        latest_commit = updated_commit;
        consumed_messages = updated_messages;

        // Make the thread chill out between the polls (until we have a better solution - long polling?)
        sleep(Duration::from_millis(500));
    }
}

fn to_i64(bytes: &[u8]) -> PrimaryKey {
    serde_json::from_slice::<PrimaryKey>(bytes).unwrap()
}

fn commit_offset_if_needed(consumer: &mut Consumer, latest_commit: &Instant, consumed_messages: bool) -> (Instant, bool) {
    let current_instant = Instant::now();
    if current_instant.gt(&latest_commit.add(Duration::from_secs(30)))
        && consumed_messages {
        println!("Committing offset to Kafka...");
        consumer.commit_consumed();
        return (current_instant, false);
    }

    (latest_commit.to_owned(), consumed_messages)
}
