use crate::error::VirtualTableError;
use failure::_core::fmt::{Display, Formatter};
use std::fmt::Result as FmtResult;
use std::rc::Rc;
use uuid::Uuid;
use linked_hash_map::LinkedHashMap;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Clone, Eq, Hash)]
pub(crate) enum DataType {
    Int,
    String,
    Uuid,
}

impl DataType {
    pub(crate) fn from_str(name: &str) -> DataType {
        match name.to_lowercase().as_str() {
            "int" => DataType::Int,
            "string" => DataType::String,
            "uuid" => DataType::Uuid,
            _ => panic!("Unknown data type specified: {}", name),
        }
    }
}

pub(crate) type PrimaryKey = Uuid;
type MessageFieldName = String;
type ColumnName = String;

pub(crate) struct Table {
    identifier: String,
    topic: String,

    columns: LinkedHashMap<ColumnName, Rc<Column>>,
    rows: LinkedHashMap<PrimaryKey, Row>,

    message_field_map: HashMap<MessageFieldName, ColumnName>,
}

impl Table {
    pub(crate) fn create(identifier: String, topic: String, columns: Vec<Column>) -> Self {
        let column_tuples = columns
            .into_iter()
            .map(|col| {
                (
                    col.identifier.clone(),
                    col.target_field.clone(),
                    Rc::new(col),
                )
            })
            .collect::<Vec<(ColumnName, MessageFieldName, Rc<Column>)>>();

        Table {
            identifier,
            topic,
            message_field_map: column_tuples
                .iter()
                .map(|tuple| (tuple.1.clone(), tuple.0.clone()))
                .collect(),
            columns: column_tuples
                .into_iter()
                .map(|tuple| (tuple.0, tuple.2))
                .collect(),
            rows: LinkedHashMap::new(),
        }
    }

    pub(crate) fn add_row(&mut self, row: Row) -> Result<(), VirtualTableError> {
        if self.rows.contains_key(&row.primary_key) {
            return Result::Err(VirtualTableError::DuplicatePrimaryKey(row.primary_key));
        }

        self.rows.insert(row.primary_key, row);
        Result::Ok(())
    }

    pub(crate) fn update_row(&mut self, row: Row) -> Result<(), VirtualTableError> {
        if !self.rows.contains_key(&row.primary_key) {
            return Result::Err(VirtualTableError::PrimaryKeyNotFound(row.primary_key));
        }

        self.rows.insert(row.primary_key, row);
        Result::Ok(())
    }

    pub(crate) fn find_row(&mut self, primary_key: &PrimaryKey) -> Option<&Row> {
        self.rows.get(primary_key)
    }

    pub(crate) fn find_column_by_name(&self, field_name: &str) -> Option<Rc<Column>> {
        if let Some(rc) = self.columns.get(field_name) {
            return Some(Rc::clone(rc));
        }

        None
    }

    pub(crate) fn find_column_by_field(&self, field_name: &str) -> Option<Rc<Column>> {
        if let Some(column_name) = self.message_field_map.get(field_name) {
            return self.find_column_by_name(column_name);
        }

        None
    }
}

impl Display for Table {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        // Display the table header (column name + separator line)
        let header_row = self
            .columns
            .keys()
            .map(|col_name| format!("{:^40}", col_name))
            .collect::<Vec<String>>()
            .join("|");
        f.write_str(&header_row)?;
        f.write_str("\n")?;

        let separator_row = (0..header_row.len()).map(|_| "-").collect::<String>();

        f.write_str(&separator_row)?;
        f.write_str("\n")?;

        // Display the body, which is each row
        self.rows
            .values()
            .map(|row| {
                row.fmt(f)?;

                f.write_str("\n")
            })
            .fold(Ok(()), |_, result| result)
    }
}

#[derive(PartialEq, Eq, Hash)]
pub(crate) struct Column {
    pub(crate) identifier: String,
    pub(crate) column_type: DataType,
    pub(crate) target_field: String,
}

pub(crate) struct Row {
    pub(crate) primary_key: PrimaryKey,
    pub(crate) columns: LinkedHashMap<Rc<Column>, Box<Cell<dyn EventqlMappedValue>>>,
}

impl Display for Row {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.write_str(
            &self
                .columns
                .values()
                .map(|val| format!("{:^40}", val.value))
                .collect::<Vec<String>>()
                .join("|"),
        )
    }
}

pub(crate) trait EventqlMappedValue
where
    Self: Display,
{
    fn eventql_type() -> DataType
    where
        Self: Sized;
}

impl EventqlMappedValue for i32 {
    fn eventql_type() -> DataType {
        DataType::Int
    }
}

impl EventqlMappedValue for i64 {
    fn eventql_type() -> DataType {
        DataType::Int
    }
}

impl EventqlMappedValue for String {
    fn eventql_type() -> DataType {
        DataType::String
    }
}

impl EventqlMappedValue for &str {
    fn eventql_type() -> DataType {
        DataType::String
    }
}

impl EventqlMappedValue for Uuid {
    fn eventql_type() -> DataType {
        DataType::Uuid
    }
}

pub(crate) struct Cell<V>
where
    V: EventqlMappedValue + ?Sized,
{
    cell_type: DataType,
    value: Box<V>,
}

impl<V> Cell<V>
where
    V: EventqlMappedValue + ?Sized,
{
    pub(crate) fn for_column(column: &Column, value: Box<V>) -> Self {
        Cell {
            cell_type: column.column_type.clone(),
            value,
        }
    }

    pub(crate) fn value(&self) -> &V {
        self.value.as_ref()
    }
}
