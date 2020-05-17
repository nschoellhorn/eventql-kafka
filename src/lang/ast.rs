#[derive(Debug, PartialEq)]
pub(crate) enum EventQL {
    CreateTableStatement {
        table_identifier: String,
        column_definitions: Vec<ColumnDefinition>,
        topic_identifier: String,
    },
}

#[derive(Debug, PartialEq)]
pub(crate) struct ColumnDefinition {
    pub(crate) identifier: String,
    pub(crate) data_type: DataType,
    pub(crate) schema_property_identifier: String
}

#[derive(Debug, PartialEq)]
pub(crate) enum DataType {
    Int,
    String
}

impl DataType {
    pub(crate) fn from_str(name: &str) -> DataType {
        match name.to_lowercase().as_str() {
            "int" => DataType::Int,
            "string" => DataType::String,
            _ => panic!("Unknown data type specified: {}", name),
        }
    }
}

