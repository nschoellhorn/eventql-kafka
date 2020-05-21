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

/*pub(crate) fn json_value_to_avro_value<V: ToAvro>(json_str: JsonValue, schema: &Schema) -> V {
    match json_str {
        JsonValue::Object(map) => json_object_to_avro_value(map, schema),
        JsonValue::Array(arr) => json_array_to_avro_value(arr, schema),
        JsonValue::String(str) => AvroValue::String(str),
        JsonValue::Number(num) => json_number_to_avro_value(num),
        JsonValue::Bool(bool) => AvroValue::Boolean(bool),

        JsonValue::Null => AvroValue::Null
    }
}

fn json_object_to_avro_value<V: ToAvro>(map: Map<String, JsonValue>, schema: &Schema) -> V {
    let mut record = Record::new(schema).unwrap();
    for (key, value) in map {
        record.put(&key, json_value_to_avro_value(value, &schema))
    }

    record
}

fn json_number_to_avro_value<V: ToAvro>(num: Number) -> V {
    if num.is_f64() {
        AvroValue::Double(num.as_f64().unwrap())
    }

    AvroValue::Long(num.as_i64().unwrap())
}

fn json_array_to_avro_value(values: Vec<JsonValue>, schema: &Schema) -> AvroValue {
    AvroValue::Array(
        values.into_iter()
            .map(|val| json_value_to_avro_value(val, schema))
            .collect()
    )
}*/

