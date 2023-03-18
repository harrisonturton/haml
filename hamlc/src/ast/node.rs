use crate::syntax::Token;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Ast {
    pub stmts: Vec<Stmt>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Stmt {
    Comment(Comment),
    PackageStmt(PackageStmt),
    ImportStmt(ImportStmt),
    ConstructorDecl(ConstructorDecl),
    StructDecl(StructDecl),
    AnnotationDecl(AnnotationDecl),
    Eof,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Comment {
    pub value: Token,
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
    pub content: BlockDecl,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ConstructorDef {
    pub checked_comment: Option<Comment>,
    pub annotations: Vec<Token>,
    pub constructor: Token,
    pub name: Token,
    pub content: BlockDecl,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum BlockDecl {
    UnionDecl(FieldSetDecl),
    AliasDecl(AliasDecl),
    RepeatableDecl(FieldSetDecl),
    FieldSetDecl(FieldSetDecl),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AliasDecl {
    MapDecl(MapDecl),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FieldSetDecl {
    pub fields: Vec<FieldDecl>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MapDecl {
    pub key: FieldType,
    pub value: FieldType,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct StructDecl {
    pub annotations: Vec<Token>,
    pub name: Token,
    pub content: BlockDecl,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FieldDecl {
    pub name: Token,
    pub typ: FieldType,
    pub optional: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum FieldType {
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
    Map(Box<MapDecl>),
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
