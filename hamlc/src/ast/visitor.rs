use super::node::{
    AliasDecl, AnnotationDecl, Ast, BlockDecl, Comment, ConstructorDecl, FieldDecl, FieldSetDecl,
    FieldType, ImportStmt, PackageStmt, Stmt, StructDecl,
};
use crate::syntax::Token;

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

pub fn walk(ast: &Ast, visitor: &mut impl Visitor) {
    for stmt in ast.stmts.iter() {
        match stmt {
            Stmt::PackageStmt(stmt) => visitor.package(stmt),
            Stmt::ImportStmt(stmt) => visitor.import(stmt),
            Stmt::ConstructorDecl(stmt) => constructor_decl(ast, visitor, stmt),
            Stmt::StructDecl(stmt) => visitor.struct_decl(stmt),
            Stmt::AnnotationDecl(stmt) => visitor.annotation_decl(stmt),
            Stmt::Comment(stmt) => visitor.comment(stmt),
            Stmt::Eof => {}
        };
    }
}

fn constructor_decl(ast: &Ast, visitor: &mut impl Visitor, decl: &ConstructorDecl) {
    visitor.constructor_decl(decl);
    for annotation in &decl.annotations {
        visitor.annotation_def(annotation);
    }
    block_decl(ast, visitor, &decl.content);
}

fn block_decl(ast: &Ast, visitor: &mut impl Visitor, decl: &BlockDecl) {
    match decl {
        BlockDecl::UnionDecl(decl) => union_decl(ast, visitor, decl),
        BlockDecl::AliasDecl(decl) => alias_decl(ast, visitor, decl),
        BlockDecl::RepeatableDecl(decl) => repeatable_decl(ast, visitor, decl),
        BlockDecl::FieldSetDecl(decl) => field_set_decl(ast, visitor, decl),
    }
}

fn union_decl(ast: &Ast, visitor: &mut impl Visitor, field_set: &FieldSetDecl) {
    visitor.union_decl(field_set);
    field_set_decl(ast, visitor, field_set);
}

fn alias_decl(ast: &Ast, visitor: &mut impl Visitor, alias: &AliasDecl) {
    visitor.alias_decl(alias);
}

fn repeatable_decl(ast: &Ast, visitor: &mut impl Visitor, field_set: &FieldSetDecl) {
    visitor.repetable_decl(field_set);
    field_set_decl(ast, visitor, field_set);
}

fn field_set_decl(ast: &Ast, visitor: &mut impl Visitor, field_set: &FieldSetDecl) {
    for decl in field_set.fields.iter() {
        field_type_decl(ast, visitor, decl);
    }
}

fn field_type_decl(ast: &Ast, visitor: &mut impl Visitor, field: &FieldDecl) {
    visitor.field_decl(field);
}
