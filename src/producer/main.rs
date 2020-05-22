extern crate kafka as kafka_client;

mod cli;
mod kafka;
mod avro;

use core::fmt;
use kafka_client::error::Error as KafkaError;
use std::fmt::{Display, Formatter};
use std::io::Error as IoError;
use kafka_client::producer::{Producer, Record as KafkaRecord};
use serde_json::Value;
use avro_rs::{Writer, Schema};
use cli::ProducerArgs;
use failure::Error as GeneralError;
use avro_rs::types::{Record as AvroRecord, Value as AvroValue, ToAvro};

#[derive(Debug)]
pub(crate) enum Error {
    IoError(IoError),
    KafkaError(KafkaError),
    GeneralError(GeneralError),
    AvroSerializationError(avro_rs::SerError),
    EmptyError,
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Error::IoError(err) => err.fmt(f),
            Error::KafkaError(err) => err.fmt(f),
            Error::GeneralError(err) => err.fmt(f),
            Error::AvroSerializationError(err) => err.fmt(f),
            Error::EmptyError => f.write_str("Empty Error"),
        }
    }
}

fn main() -> Result<(), Error> {
    let arg_matcher = cli::create_arg_matcher();

    let mut args = cli::unwrap_args(arg_matcher.get_matches())?;
    let mut producer = kafka::create_producer(&args.bootstrap_server)?;

    match args.value {
        Some(_) => produce_once(&mut producer, &mut args)?,
        None => Result::Err(Error::EmptyError)?
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

    producer.send(&KafkaRecord::from_value(&producer_args.topic, writer.into_inner()))
        .map_err(|err| Error::KafkaError(err))
}

/*fn produce_from_cli(producer: &mut Producer, producer_args: &ProducerArgs) -> Result<(), Error> {

}*/
