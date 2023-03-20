pub mod ast;
pub mod db;
pub mod diagnostics;
pub mod queries;
pub mod syntax;

// The salsa database is defined in terms of jars. These store all the
// intermediate state managed by salsa; all annotations refer to jars.
#[salsa::jar(db = Db)]
pub struct Jar(
    crate::diagnostics::Diagnostics,
    crate::queries::Path,
    crate::queries::SourceFile,
    crate::queries::read_file,
    crate::queries::parse_file,
);

pub trait Db: salsa::DbWithJar<Jar> {}

impl<DB> Db for DB where DB: ?Sized + salsa::DbWithJar<Jar> {}
