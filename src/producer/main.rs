extern crate kafka as kafka_client;

mod cli;
mod kafka;
mod avro;

use core::fmt;
use kafka_client::error::Error as KafkaError;
use std::fmt::{Display, Formatter};
use std::io::{Error as IoError, stdin};
use kafka_client::producer::{Producer, Record as KafkaRecord};
use serde_json::Value;
use avro_rs::Writer;
use cli::ProducerArgs;
use failure::Error as GeneralError;
use uuid::Uuid;

#[derive(Debug)]
pub(crate) enum Error {
    IoError(IoError),
    KafkaError(KafkaError),
    GeneralError(GeneralError),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Error::IoError(err) => err.fmt(f),
            Error::KafkaError(err) => err.fmt(f),
            Error::GeneralError(err) => err.fmt(f),
        }
    }
}

fn main() -> Result<(), Error> {
    let arg_matcher = cli::create_arg_matcher();

    let mut args = cli::unwrap_args(arg_matcher.get_matches())?;
    let mut producer = kafka::create_producer(&args.bootstrap_server)?;

    match args.value {
        Some(_) => produce_once(&mut producer, &mut args)?,
        None => produce_from_cli(&mut producer, &mut args)?,
    };

    Ok(())
}

fn produce_once(producer: &mut Producer, producer_args: &mut ProducerArgs) -> Result<(), Error> {
    let value = producer_args.value.clone()
        .expect("No value provided!");
    let schema = avro::read_schema_from_file(&mut producer_args.schema_file)?;

    let mut writer = Writer::new(&schema, vec!());
    let encoded = avro::json_value_to_avro_value(
        serde_json::from_str::<Value>(&value).expect("Invalid JSON provided as message value."),
        &schema
    );

    writer.append(encoded).map_err(|err| Error::GeneralError(err))?;
    writer.flush().map_err(|err| Error::GeneralError(err))?;

    let uuid = Uuid::parse_str(&producer_args.key).expect("Invalid UUID specified as key.");
    let uuid_bytes = uuid.as_bytes();

    producer.send(&KafkaRecord::from_key_value(&producer_args.topic, &uuid_bytes[0..], writer.into_inner()))
        .map_err(|err| Error::KafkaError(err))
}

fn produce_from_cli(producer: &mut Producer, producer_args: &mut ProducerArgs) -> Result<(), Error> {
    let schema = avro::read_schema_from_file(&mut producer_args.schema_file)?;

    let mut str_buf = String::new();
    loop {
        stdin().read_line(&mut str_buf);

        let mut writer = Writer::new(&schema, vec!());
        let encoded = avro::json_value_to_avro_value(
            serde_json::from_str::<Value>(&str_buf).expect("Invalid JSON provided as message value."),
            &schema
        );

        str_buf.truncate(0);

        writer.append(encoded).map_err(|err| Error::GeneralError(err))?;
        writer.flush().map_err(|err| Error::GeneralError(err))?;

        // Big fucking hack since the Kafka lib only accepts stuff that is implemented with "AsBytes" trait.
        let uuid = Uuid::parse_str(&producer_args.key).expect("Invalid UUID specified as key.");
        let uuid_bytes = uuid.as_bytes();

        producer.send(&KafkaRecord::from_key_value(&producer_args.topic, &uuid_bytes[0..], writer.into_inner()))
            .map_err(|err| Error::KafkaError(err))?;
    }
}
