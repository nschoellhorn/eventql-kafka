extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::Parser;
use pest::iterators::{Pairs, Pair};
use crate::ast::EventQL;

mod ast;

#[derive(Parser)]
#[grammar = "../resources/grammar.pest"]
struct EventQLParser;

fn main() {
    let example_query = "CREATE TABLE someIdent; create TABLE anotherOne;";
    println!("Parsing Query: {}", example_query);
    let parsed_query = EventQLParser::parse(Rule::eventQlStatement, example_query)
        .unwrap_or_else(|e| panic!("{}", e));

    let ast = create_ast(parsed_query);
    println!("{:?}", ast);
}

fn create_ast(parsed_query: Pairs<Rule>) -> std::vec::Vec<EventQL> {
    let mut statements = vec!();
    for statement in parsed_query {
        statements.push(parse_statement(statement))
    }

    statements
}

fn parse_statement(statement: Pair<Rule>) -> EventQL {
    match statement.as_rule() {
        Rule::createTableStatement => EventQL::CreateTableStatement(String::from(statement.into_inner().next().unwrap().as_str())),
        _ => panic!("Unknown token: {:?}", statement.as_rule()),
    }
}
