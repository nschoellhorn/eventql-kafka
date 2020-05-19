use schema_registry_converter::Decoder;
use avro_rs::types::{Record, Value};
use kafka::consumer::{Consumer, FetchOffset, GroupOffsetStorage, MessageSet, MessageSets};
use std::collections::HashMap;
use std::thread;
use std::thread::sleep;
use std::time::Duration;
use kafka::client::SecurityConfig;

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

impl KafkaWrapper {

    pub(crate) fn with_decoder(decoder: Decoder) -> Self {
        KafkaWrapper {
            decoder,
            table_registry: TableRegistry::new()
        }
    }

    pub(crate) fn consume_via<F>(&mut self, topic: &str, message_handler: F) where F: Fn(Value) -> () {
        println!("Topic: {}", topic);
        let mut consumer: Consumer = Consumer::from_hosts(vec!("192.168.99.100:9092".to_owned()))
            .with_topic(String::from(topic))
            .with_fallback_offset(FetchOffset::Earliest)
            .with_offset_storage(GroupOffsetStorage::Kafka)
            .create()
            .unwrap();

        loop {
            println!("Polling messages.");
            let messages: MessageSets = consumer.poll().unwrap();
            messages.iter().flat_map(|message_set| message_set.messages())
                .map(|message| self.decoder.decode(Some(message.value)))
                .map(|result| result.unwrap())
                .for_each(&message_handler);

            sleep(Duration::from_millis(100));
        }
    }

}
