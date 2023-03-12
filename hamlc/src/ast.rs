use crate::token::Token;

pub trait Visitor {
    fn package(&mut self, _decl: &PackageStmt) {}

    fn import(&mut self, _decl: &ImportStmt) {}

    fn constructor_decl(&mut self, _decl: &ConstructorDecl) {}

    fn struct_decl(&mut self, _decl: &StructDecl) {}

    fn union_decl(&mut self, _decl: &FieldSetDecl) {}

    fn repetable_decl(&mut self, _decl: &FieldSetDecl) {}

    fn alias_decl(&mut self, _decl: &AliasDecl) {}

    fn field_decl(&mut self, _decl: &FieldDecl) {}

    fn field_type_decl(&mut self, _decl: &FieldType) {}

    fn annotation_decl(&mut self, _decl: &AnnotationDecl) {}

    fn annotation_def(&mut self, _def: &Token) {}

    fn comment(&mut self, _stmt: &Comment) {}
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Ast<'a> {
    pub input: &'a str,
    pub stmts: Vec<Stmt>,
}

impl<'a> Ast<'a> {
    pub fn span(&self, token: &Token) -> &'a str {
        &self.input[token.start..token.start + token.len]
    }

    pub fn walk(&self, visitor: &mut impl Visitor) {
        for stmt in self.stmts.iter() {
            match stmt {
                Stmt::PackageStmt(stmt) => visitor.package(stmt),
                Stmt::ImportStmt(stmt) => visitor.import(stmt),
                Stmt::ConstructorDecl(stmt) => self.constructor_decl(visitor, stmt),
                Stmt::StructDecl(stmt) => visitor.struct_decl(stmt),
                Stmt::AnnotationDecl(stmt) => visitor.annotation_decl(stmt),
                Stmt::Comment(stmt) => visitor.comment(stmt),
                Stmt::Eof => {}
            };
        }
    }

    fn constructor_decl(&self, visitor: &mut impl Visitor, decl: &ConstructorDecl) {
        visitor.constructor_decl(decl);
        for annotation in &decl.annotations {
            visitor.annotation_def(&annotation);
        }
        self.block_decl(visitor, &decl.content);
    }

    fn block_decl(&self, visitor: &mut impl Visitor, decl: &BlockDecl) {
        match decl {
            BlockDecl::UnionDecl(decl) => self.union_decl(visitor, decl),
            BlockDecl::AliasDecl(decl) => self.alias_decl(visitor, decl),
            BlockDecl::RepeatableDecl(decl) => self.repeatable_decl(visitor, decl),
            BlockDecl::FieldSetDecl(decl) => self.field_set_decl(visitor, decl),
        }
    }

    fn union_decl(&self, visitor: &mut impl Visitor, field_set: &FieldSetDecl) {
        visitor.union_decl(field_set);
        self.field_set_decl(visitor, field_set);
    }

    fn alias_decl(&self, visitor: &mut impl Visitor, alias: &AliasDecl) {
        visitor.alias_decl(alias);
    }

    fn repeatable_decl(&self, visitor: &mut impl Visitor, field_set: &FieldSetDecl) {
        visitor.repetable_decl(field_set);
        self.field_set_decl(visitor, field_set);
    }

    fn field_set_decl(&self, visitor: &mut impl Visitor, field_set: &FieldSetDecl) {
        for decl in field_set.fields.iter() {
            self.field_type_decl(visitor, decl);
        }
    }

    fn field_type_decl(&self, visitor: &mut impl Visitor, field: &FieldDecl) {
        visitor.field_decl(field);
    }
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

pub struct CommentPart {
    pub annotation: Option<StringAnnotation>,
    pub comment: String,
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
