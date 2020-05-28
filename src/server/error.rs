use crate::virtual_table::{PrimaryKey, Column};
use std::rc::Rc;
use std::fmt::Display;
use failure::_core::fmt::Formatter;
use std::fmt::Result as FmtResult;

pub(crate) enum VirtualTableError {
    DuplicatePrimaryKey(PrimaryKey),
    PrimaryKeyNotFound(PrimaryKey),
    ValueNull(Rc<Column>),
}

impl Display for VirtualTableError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            VirtualTableError::DuplicatePrimaryKey(key) => f.write_str(&format!("Tried to create row with key '{}' but a row with this key already exists. Try updating instead.", key)),
            VirtualTableError::PrimaryKeyNotFound(key) => f.write_str(&format!("Tried to apply update on a row with key '{}' but no such row exists. Try inserting first.", key)),
            VirtualTableError::ValueNull(col) => f.write_str(&format!("No value (NULL) for required column {}", col.identifier))
        }
    }
}
