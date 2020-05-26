extern crate pest;
#[macro_use]
extern crate pest_derive;

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;

use avro_rs::types::Value;
use failure::_core::hash::Hash;

use crate::virtual_table::{Cell, Column, DataType, EventqlMappedValue, PrimaryKey, Row, Table};
use std::rc::Rc;
use uuid::Uuid;

mod parser;
mod ast;
mod kafka;
mod virtual_table;
mod error;

#[tokio::main]
async fn main() {
    //let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    //let mut kafka_wrapper = KafkaWrapper::with_decoder();
    let first_thread = tokio::spawn(async move {
        let mut table = Table::create(
            "appuser_table".to_string(),
            "appuser".to_string(),
            vec!(
                Column {
                    identifier: "id".to_string(),
                    column_type: DataType::Int,
                    target_field: "KEY".to_string(),
                },
                Column {
                    identifier: "firstname".to_string(),
                    column_type: DataType::String,
                    target_field: "first_name".to_string(),
                },
                Column {
                    identifier: "last_name".to_string(),
                    column_type: DataType::String,
                    target_field: "last_name".to_string(),
                }
            ),
        );
        kafka::consume_via("appuser",  move |key, value| {
            aggregate_on_virtual_table(&mut table, key, value);
        });
    }).await;

    //first_thread.join();

    /*for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }*/
}

fn aggregate_on_virtual_table(table: &mut Table, key: PrimaryKey, value: Value) {
    println!("Received message with key {} on appuser: {:#?}", key, value);

    let mut row_map: HashMap<Rc<Column>, Box<Cell<dyn EventqlMappedValue>>> = HashMap::new();

    let field_map = match value {
        Value::Map(map) => map,
        Value::Record(vec) => vec.into_iter().collect::<HashMap<String, Value>>(),
        _ => panic!("Unsupported value: {:?}", value),
    };

    field_map.into_iter()
        .for_each(|(field_name, value)| {
            let column_option = table.find_column_by_field(&field_name);
            if let None = column_option {
                panic!("Invalid field name: {}", field_name);
            }

            let col_rc = Rc::clone(&column_option.unwrap());
            let cell = Cell::for_column(&col_rc, retrieve_value(value));

            row_map.insert(
                col_rc,
                Box::new(cell)
            );
        });

    let id_col = table.find_column_by_name("id").unwrap();
    let boxed_val: Box<dyn EventqlMappedValue> = Box::new(key); // Can we solve this without temp var?
    let cell = Cell::for_column(&id_col, boxed_val);
    row_map.insert(
        id_col,
        Box::new(cell)
    );

    match table.find_row(&key) {
        Some(row) => table.update_row(Row {
            columns: row_map,
            ..*row
        }),// TODO: Update row
        None => table.add_row(Row {
            primary_key: key,
            columns: row_map,
        }),
    };

    println!("Current Table:\n{}", table);
}

fn retrieve_value(avro_value: Value) -> Box<dyn EventqlMappedValue> {
    match avro_value {
        Value::String(str) => Box::new(str),
        Value::Long(long) => Box::new(long),
        Value::Int(int) => Box::new(int),
        _ => panic!("Value not supported: {:?}", avro_value),
    }
}

/*fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];

    stream.read(&mut buffer).unwrap();

    let request = String::from_utf8_lossy(&buffer[..]).trim().to_string();

    println!("Request: {}", request);

    let lexed_query = parser::lex(&request);
    let ast = parser::create_ast(lexed_query);
    println!("Reponse: {:#?}", ast);

    stream.write(format!("{:#?}", ast).as_bytes());
    stream.flush();
}*/
