use crate::token::Token;

pub trait Visitor<T> {
    fn visit_package(&mut self, import: &PackageStmt) -> T;
    fn visit_import(&mut self, import: &ImportStmt) -> T;
    fn visit_constructor(&mut self, block: &ConstructorDecl) -> T;
    fn visit_annotation(&mut self, block: &AnnotationDecl) -> T;
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Ast {
    pub stmts: Vec<Stmt>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Stmt {
    PackageStmt(PackageStmt),
    ImportStmt(ImportStmt),
    ConstructorDecl(ConstructorDecl),
    StructDecl(StructDecl),
    AnnotationDecl(AnnotationDecl),
    Eof,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PackageStmt {
    pub segments: Vec<Token>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ImportStmt {
    pub path: Token,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ConstructorDecl {
    pub annotations: Vec<Token>,
    pub name: Token,
    pub fields: Vec<FieldDecl>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct StructDecl {
    pub annotations: Vec<Token>,
    pub name: Token,
    pub fields: Vec<FieldDecl>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FieldDecl {
    pub name: Token,
    pub value: FieldValue,
    pub optional: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum FieldValue {
    Ident(Token),
    String(Token),
    Uint32(Token),
    Uint64(Token),
    Int32(Token),
    Int64(Token),
    Float32(Token),
    Float64(Token),
    Unknown(Token),
    Struct(Token),
    Map(Box<(FieldValue, FieldValue)>),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AnnotationDecl {
    pub annotations: Vec<Token>,
    pub name: Token,
    pub fields: Vec<AnnotationFieldDecl>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AnnotationFieldDecl {
    pub name: Token,
    pub value: AnnotationFieldValue,
    pub optional: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AnnotationFieldValue {
    String(Token),
    Uint32(Token),
    Uint64(Token),
    Int32(Token),
    Int64(Token),
    Float32(Token),
    Float64(Token),
}
