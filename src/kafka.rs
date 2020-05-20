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

pub(crate) fn consume_via<F>(decoder: Arc<Mutex<Decoder>>, topic: &str, message_handler: F) where F: Fn(Value) -> () {
    println!("Topic: {}", topic);
    let mut consumer: Consumer = Consumer::from_hosts(vec!("localhost:9092".to_owned()))
        .with_topic(String::from(topic))
        .with_fallback_offset(FetchOffset::Earliest)
        .with_offset_storage(GroupOffsetStorage::Kafka)
        .create()
        .unwrap();

    let mut latest_commit = Instant::now();

    loop {
        let messages: MessageSets = consumer.poll().unwrap();
        messages.iter().flat_map(|message_set| {
            let messages = message_set.messages();
            consumer.consume_messageset(message_set);

            messages
        })
            .map(|message| decoder.lock().unwrap().decode(Some(message.value)))
            .map(|result| result.unwrap())
            .for_each(&message_handler);

        // Commit current offset to Kafka every 5 seconds
        let current_instant = Instant::now();
        if current_instant.gt(&latest_commit.add(Duration::from_secs(5))) {
            consumer.commit_consumed();
            latest_commit = current_instant;
        }

        sleep(Duration::from_millis(500));
    }
}
