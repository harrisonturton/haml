use super::node::{
    AliasDecl, AnnotationDecl, Ast, BlockDecl, Comment, ConstructorDecl, FieldDecl, FieldSetDecl,
    FieldType, ImportStmt, Node, PackageStmt, StructDecl,
};
use crate::syntax::Token;

pub trait Visitor {
    fn package(&mut self, _decl: &PackageStmt) {}

    fn import(&mut self, _decl: &ImportStmt) {}

    fn constructor_decl(&mut self, _decl: &ConstructorDecl) {}

    fn struct_decl(&mut self, _decl: &StructDecl) {}

    fn repetable_decl(&mut self, _decl: &FieldSetDecl) {}

    fn alias_decl(&mut self, _decl: &AliasDecl) {}

    fn field_decl(&mut self, _decl: &FieldDecl) {}

    fn field_type_decl(&mut self, _decl: &FieldType) {}

    fn annotation_decl(&mut self, _decl: &AnnotationDecl) {}

    fn annotation_def(&mut self, _def: &Token) {}

    fn comment(&mut self, _stmt: &Comment) {}
}

#[allow(unused)]
pub fn walk(ast: &Ast, visitor: &mut impl Visitor) {
    for stmt in ast.nodes.iter() {
        match stmt {
            Node::PackageStmt(stmt) => visitor.package(stmt),
            Node::ImportStmt(stmt) => visitor.import(stmt),
            Node::ConstructorDecl(stmt) => constructor_decl(ast, visitor, stmt),
            Node::StructDecl(stmt) => visitor.struct_decl(stmt),
            Node::AnnotationDecl(stmt) => visitor.annotation_decl(stmt),
            Node::Comment(stmt) => visitor.comment(stmt),
            Node::Eof => {}
        };
    }
}

#[allow(unused)]
fn constructor_decl(ast: &Ast, visitor: &mut impl Visitor, decl: &ConstructorDecl) {
    visitor.constructor_decl(decl);
    for annotation in &decl.annotations {
        visitor.annotation_def(annotation);
    }
    block_decl(ast, visitor, &decl.content);
}

#[allow(unused)]
fn block_decl(ast: &Ast, visitor: &mut impl Visitor, decl: &BlockDecl) {
    match decl {
        BlockDecl::Alias(decl) => alias_decl(ast, visitor, decl),
        BlockDecl::Repeatable(decl) => repeatable_decl(ast, visitor, decl),
        BlockDecl::FieldSet(decl) => field_set_decl(ast, visitor, decl),
    }
}

#[allow(unused)]
fn alias_decl(_ast: &Ast, visitor: &mut impl Visitor, alias: &AliasDecl) {
    visitor.alias_decl(alias);
}

#[allow(unused)]
fn repeatable_decl(ast: &Ast, visitor: &mut impl Visitor, field_set: &FieldSetDecl) {
    visitor.repetable_decl(field_set);
    field_set_decl(ast, visitor, field_set);
}

#[allow(unused)]
fn field_set_decl(ast: &Ast, visitor: &mut impl Visitor, field_set: &FieldSetDecl) {
    for decl in field_set.fields.iter() {
        field_type_decl(ast, visitor, decl);
    }
}

#[allow(unused)]
fn field_type_decl(_ast: &Ast, visitor: &mut impl Visitor, field: &FieldDecl) {
    visitor.field_decl(field);
}
