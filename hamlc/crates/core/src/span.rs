use crate::queries::SourceFile;
use derive_new::new;

#[derive(new, Debug, Copy, Clone, PartialEq, Eq)]
pub struct Span {
    pub start: usize,
    pub end: usize,
    pub file: SourceFile,
}
