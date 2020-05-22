use avro_rs::Schema;
use std::fs::File;
use std::io::Read;
use crate::Error;
use avro_rs::types::{Record, ToAvro};
use serde_json::{Value as JsonValue, Number, Map};
use avro_rs::types::Value as AvroValue;
use crate::Error::AvroSerializationError;

pub(crate) fn read_schema_from_file(file: &mut File) -> Result<Schema, Error> {
    let mut str_buf = String::new();
    file.read_to_string(&mut str_buf)
        .map_err(|err| Error::IoError(err))?;

    Schema::parse_str(&str_buf)
        .map_err(|err| Error::GeneralError(err))
}

fn json_object_to_avro_value(map: Map<String, JsonValue>, schema: &Schema) -> AvroValue {
    let mut record = Record::new(schema).unwrap();
    for (key, value) in map {
        record.put(&key, value.avro())
    }

    record.avro()
}

pub(crate) fn json_value_to_avro_value(json_value: JsonValue, schema: &Schema) -> AvroValue {
    match json_value {
        JsonValue::Object(map) => json_object_to_avro_value(map, schema),
        JsonValue::Array(arr) => json_array_to_avro_value(arr, schema),
        JsonValue::String(str) => AvroValue::String(str),
        JsonValue::Number(num) => json_number_to_avro_value(num),
        JsonValue::Bool(bool) => AvroValue::Boolean(bool),

        JsonValue::Null => AvroValue::Null
    }
}

fn json_number_to_avro_value(num: Number) -> AvroValue {
    if num.is_f64() {
        return AvroValue::Double(num.as_f64().unwrap())
    }

    AvroValue::Long(num.as_i64().unwrap())
}

fn json_array_to_avro_value(values: Vec<JsonValue>, schema: &Schema) -> AvroValue {
    AvroValue::Array(
        values.into_iter()
            .map(|val| json_value_to_avro_value(val, schema))
            .collect()
    )
}

