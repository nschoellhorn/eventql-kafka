use std::fs::File;
use clap::{ArgMatches, Arg, App};
use crate::Error;
use crate::kafka;

const ARG_TOPIC: &str = "topic";
const ARG_BOOTSTRAP_SERVER: &str = "bootstrap-server";
const ARG_SCHEMA: &str = "schema";
const ARG_KEY: &str = "key";
const ARG_VALUE: &str = "value";

pub(crate) struct ProducerArgs {
    pub(crate) topic: String,
    pub(crate) bootstrap_server: String,
    pub(crate) schema_file: File,
    pub(crate) key: String,
    pub(crate) value: Option<String>,
}

pub(crate) fn create_arg_matcher<'a, 'b>() -> App<'a, 'b> {
    App::new("EventQL Producer")
        .arg(
            Arg::with_name(ARG_TOPIC)
                .short("t")
                .long("topic")
                .value_name("TOPIC NAME")
                .takes_value(true)
                .required(true)
        )
        .arg(
            Arg::with_name(ARG_BOOTSTRAP_SERVER)
                .short("b")
                .long("bootstrap-server")
                .value_name("HOSTNAME:PORT")
                .takes_value(true)
                .required(true)
        )
        .arg(
            Arg::with_name(ARG_SCHEMA)
                .short("s")
                .long("schema")
                .value_name("SCHEMA FILE")
                .takes_value(true)
                .required(true)
        )
        .arg(
            Arg::with_name(ARG_KEY)
                .short("k")
                .long("key")
                .value_name("KEY")
                .takes_value(true)
                .required(true)
        )
        .arg(
            Arg::with_name(ARG_VALUE)
                .value_name("JSON MESSAGE")
                .takes_value(true)
        )
}

pub(crate) fn unwrap_args(args: ArgMatches) -> Result<ProducerArgs, Error> {
    kafka::open_schema_file(args.value_of(ARG_SCHEMA).unwrap())
        .and_then(|file| {
            Ok(ProducerArgs {
                bootstrap_server: args.value_of(ARG_BOOTSTRAP_SERVER).unwrap().to_string(),
                key: args.value_of(ARG_KEY).unwrap().to_string(),
                schema_file: file,
                topic: args.value_of(ARG_TOPIC).unwrap().to_string(),
                value: args.value_of(ARG_VALUE).map(|str| str.to_string())
            })
        })
}

