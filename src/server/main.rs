extern crate pest;
#[macro_use]
extern crate pest_derive;

use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

use avro_rs::types::Value;
use schema_registry_converter::Decoder;

use crate::kafka::KafkaWrapper;
use std::thread;
use std::sync::{Arc, Mutex};

mod parser;
mod ast;
mod kafka;

fn main() {
    //let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    //let mut kafka_wrapper = KafkaWrapper::with_decoder();
    let first_thread = thread::spawn(|| {
        kafka::consume_via("appuser", |message| println!("Received message on appuser: {:#?}", message));
    });

    let second_thread = thread::spawn(|| {
        kafka::consume_via("sometopic", |message| println!("Received message on sometopic: {:#?}", message));
    });

    first_thread.join();
    second_thread.join();

    /*for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }*/
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];

    stream.read(&mut buffer).unwrap();

    let request = String::from_utf8_lossy(&buffer[..]).trim().to_string();

    println!("Request: {}", request);

    let lexed_query = parser::lex(&request);
    let ast = parser::create_ast(lexed_query);
    println!("Reponse: {:#?}", ast);

    stream.write(format!("{:#?}", ast).as_bytes());
    stream.flush();
}
