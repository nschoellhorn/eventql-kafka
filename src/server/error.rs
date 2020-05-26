use crate::virtual_table::PrimaryKey;

pub(crate) enum VirtualTableError {
    DuplicatePrimaryKey(PrimaryKey),
    PrimaryKeyNotFound(PrimaryKey),
}
