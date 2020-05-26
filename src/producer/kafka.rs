use crate::Error;
use kafka::producer::Producer;
use std::fs::File;
use std::path::Path;

pub(crate) fn open_schema_file(path_str: &str) -> Result<File, Error> {
    File::open(Path::new(path_str)).map_err(|err| Error::IoError(err))
}

pub(crate) fn create_producer(bootstrap_server: &str) -> Result<Producer, Error> {
    Producer::from_hosts(vec![bootstrap_server.to_string()])
        .create()
        .map_err(|err| Error::KafkaError(err))
}
