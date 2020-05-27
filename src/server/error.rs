use crate::virtual_table::{PrimaryKey, Column};
use std::rc::Rc;

pub(crate) enum VirtualTableError {
    DuplicatePrimaryKey(PrimaryKey),
    PrimaryKeyNotFound(PrimaryKey),
    ValueNull(Rc<Column>),
}
