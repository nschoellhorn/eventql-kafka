extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::iterators::{Pair, Pairs};
use pest::Parser;

use crate::ast::{EventQL, ColumnDefinition, DataType};

mod ast;

#[derive(Parser)]
#[grammar = "../resources/grammar.pest"]
struct EventQLParser;

fn main() {
    let example_query = "CREATE TABLE someIdent (id INT FROM id, first_name STRING FROM fname, last_name STRING FROM lname) VIA sometopic; CREATE TABLE something (id INT FROM id, first_name STRING FROM fname, last_name STRING FROM lname) VIA othertopic;";
    println!("Parsing Query: {}", example_query);
    let parsed_query = EventQLParser::parse(Rule::eventQlStatement, example_query)
        .unwrap_or_else(|e| panic!("{}", e));

    let ast = create_ast(parsed_query);
    println!("{:#?}", ast);
}

fn create_ast(parsed_query: Pairs<Rule>) -> std::vec::Vec<EventQL> {
    parsed_query.map(|token| parse_statement(token))
        .collect()
}

fn parse_statement(statement: Pair<Rule>) -> EventQL {
    let rule = statement.as_rule();
    match rule {
        Rule::createTableStatement => build_create_table_statement(statement.into_inner()),
        _ => panic!("Unknown token: {:?}", rule),
    }
}

fn build_create_table_statement(inner_tokens: Pairs<Rule>) -> EventQL {
    let mut identifier: Option<String> = None;
    let mut column_definitions: Vec<ColumnDefinition> = vec!();
    let mut topic_identifier: Option<String> = None;

    for token in inner_tokens {
        match token.as_rule() {
            Rule::identifier => identifier = Some(String::from(token.as_str())),
            Rule::columnDefinition => column_definitions.push(build_column_definition(token.into_inner())),
            Rule::schemaIdentifier => topic_identifier = Some(String::from(token.as_str())),
            _ => ()
        }
    }

    EventQL::CreateTableStatement {
        table_identifier: identifier.expect("No table identifier specified."),
        column_definitions,
        topic_identifier: topic_identifier.expect("No topic identifier specified."),
    }
}

fn build_column_definition(inner_tokens: Pairs<Rule>) -> ColumnDefinition {
    let mut identifier: Option<String> = None;
    let mut data_type: Option<DataType> = None;
    let mut property_identifier: Option<String> = None;

    for token in inner_tokens {
        match token.as_rule() {
            Rule::identifier => identifier = Some(String::from(token.as_str())),
            Rule::columnType => data_type = Some(DataType::from_str(token.as_str())),
            Rule::schemaIdentifier => property_identifier = Some(String::from(token.as_str())),
            _ => ()
        }
    }

    ColumnDefinition {
        identifier: identifier.expect("No column identifier specified."),
        data_type: data_type.expect("No data type specified."),
        schema_property_identifier: property_identifier.expect("No property identifier specified.")
    }
}
