use crate::virtual_table::DataType;

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
    pub(crate) schema_property_identifier: String,
}
