use derive_new::new;

use crate::diagnostics::Emitter;
use crate::queries::SourceFile;
use crate::span::Span;
use crate::Db;

use super::lexer::Lexer;
use super::token::{Token, TokenKind};
use crate::ast::node::{
    AliasDecl, AnnotationDecl, AnnotationFieldDecl, AnnotationFieldValue, Ast, BlockDecl,
    ConstructorDecl, FieldDecl, FieldSetDecl, FieldType, ImportStmt, MapDecl, Node, PackageStmt,
    StructDecl,
};

#[derive(new)]
pub struct ParseSession<'db> {
    pub db: &'db dyn Db,
    pub file: SourceFile,
}

impl<'db> ParseSession<'db> {
    pub fn text(&self) -> &'db str {
        self.file.text(self.db)
    }

    pub fn span_text(&self, span: &Span) -> &'db str {
        let text = self.file.text(self.db);
        &text[span.start..span.end]
    }
}

/// Turns tokens into statements.
pub struct Parser<'i> {
    emitter: &'i dyn Emitter,
    lexer: Lexer<'i>,
}

impl<'i> Parser<'i> {
    pub fn new(sess: &'i ParseSession<'i>, emitter: &'i dyn Emitter) -> Parser<'i> {
        Parser {
            emitter,
            lexer: Lexer::new(sess, emitter),
        }
    }

    pub fn parse(&mut self) -> Option<Ast> {
        let mut stmts = vec![];
        loop {
            let stmt = self.advance()?;
            if stmt == Node::Eof {
                break;
            }
            stmts.push(stmt);
        }
        Some(Ast { nodes: stmts })
    }

    pub fn advance(&mut self) -> Option<Node> {
        let token = match self.advance_token() {
            Some(token) => token,
            None => return Some(Node::Eof),
        };
        match token.kind {
            TokenKind::Package => self.package_stmt(),
            TokenKind::Import => self.import_stmt(),
            TokenKind::At => self.annotation_def(),
            TokenKind::Struct => self.struct_decl(vec![]),
            TokenKind::Constructor => self.constructor_decl(vec![]),
            TokenKind::Annotation => self.annotation_decl(vec![]),
            _ => {
                self.emitter
                    .emit_unexpected_token(token, "a package, import or declaration");
                None
            }
        }
    }

    fn package_stmt(&mut self) -> Option<Node> {
        let mut segments = vec![];
        loop {
            let segment = self.pop(TokenKind::Ident)?;
            segments.push(segment);

            let token = self.advance_token()?;
            match token.kind {
                TokenKind::Period => continue,
                TokenKind::Semi => break,
                _ => {
                    self.emitter
                        .emit_unexpected_token(token, "period or semicolon");
                    return None;
                }
            };
        }
        let stmt = PackageStmt { segments };
        Some(Node::PackageStmt(stmt))
    }

    fn import_stmt(&mut self) -> Option<Node> {
        let path = self.pop(TokenKind::StringLiteral)?;
        self.pop(TokenKind::Semi)?;
        let stmt = ImportStmt { path };
        Some(Node::ImportStmt(stmt))
    }

    fn annotation_def(&mut self) -> Option<Node> {
        let mut annotations = vec![];
        loop {
            let name = self.pop(TokenKind::Ident)?;
            annotations.push(name);

            let token = self.advance_token()?;
            match token.kind {
                TokenKind::At => continue,
                TokenKind::Constructor => return self.constructor_decl(annotations),
                TokenKind::Struct => return self.struct_decl(annotations),
                TokenKind::Annotation => return self.annotation_decl(annotations),
                _ => {
                    self.emitter
                        .emit_unexpected_token(token, "an annotation, constructor or struct");
                    return None;
                }
            };
        }
    }

    fn struct_decl(&mut self, annotations: Vec<Token>) -> Option<Node> {
        let name = self.pop(TokenKind::Ident)?;
        let content = self.block_decl()?;
        let stmt = StructDecl {
            annotations,
            name,
            content,
        };
        Some(Node::StructDecl(stmt))
    }

    fn constructor_decl(&mut self, annotations: Vec<Token>) -> Option<Node> {
        let name = self.pop(TokenKind::Ident)?;
        let content = self.block_decl()?;
        let stmt = ConstructorDecl {
            annotations,
            name,
            content,
        };
        Some(Node::ConstructorDecl(stmt))
    }

    fn block_decl(&mut self) -> Option<BlockDecl> {
        self.pop(TokenKind::OpenBrace)?;
        let discriminator = self.advance_token()?;
        match discriminator.kind {
            TokenKind::Union => self.union_decl(),
            TokenKind::Repeatable => self.repeatable_decl(),
            TokenKind::Map => {
                let map = self.map_decl()?;
                self.pop(TokenKind::CloseBrace)?;
                let decl = AliasDecl::MapDecl(map);
                Some(BlockDecl::Alias(decl))
            }
            TokenKind::Ident => {
                let fields = self.field_set_decl(Some(discriminator))?;
                Some(BlockDecl::FieldSet(fields))
            }
            TokenKind::CloseBrace => {
                let fieldset = FieldSetDecl { fields: vec![] };
                Some(BlockDecl::FieldSet(fieldset))
            }
            _ => {
                self.emitter
                    .emit_unexpected_token(discriminator, "union, repeatable or an identifier");
                return None;
            }
        }
    }

    fn union_decl(&mut self) -> Option<BlockDecl> {
        self.pop(TokenKind::OpenBrace)?;
        let fields = self.field_set_decl(None)?;
        self.pop(TokenKind::CloseBrace)?;
        Some(BlockDecl::FieldSet(fields))
    }

    fn repeatable_decl(&mut self) -> Option<BlockDecl> {
        self.pop(TokenKind::OpenBrace)?;
        let fields = self.field_set_decl(None)?;
        self.pop(TokenKind::CloseBrace)?;
        Some(BlockDecl::Repeatable(fields))
    }

    fn annotation_decl(&mut self, annotations: Vec<Token>) -> Option<Node> {
        let name = self.pop(TokenKind::Ident)?;
        let fields = self.annotation_field_set_decl()?;
        let stmt = AnnotationDecl {
            annotations,
            name,
            fields,
        };
        Some(Node::AnnotationDecl(stmt))
    }

    // Set of nested key-value pairs inside two braces. `leading_ident` is provided
    // so block_decl can call field_set_decl if it encounters a identifier. The
    // lexer doesn't support peeking so we have to work without this lookahead.
    fn field_set_decl(&mut self, leading_ident: Option<Token>) -> Option<FieldSetDecl> {
        let mut fields = vec![];

        if let Some(name) = leading_ident {
            let mut optional = false;
            let token = self.advance_token()?;
            match token.kind {
                TokenKind::QuestionMark => {
                    optional = true;
                    self.pop(TokenKind::Colon)?;
                }
                TokenKind::Colon => {}
                _ => {
                    self.emitter
                        .emit_unexpected_token(token, "a question mark or colon");
                    return None;
                }
            };
            let value = self.field_value()?;
            self.pop(TokenKind::Semi)?;
            fields.push(FieldDecl {
                name,
                typ: value,
                optional,
            })
        }

        loop {
            let mut optional = false;
            let token = self.advance_token()?;
            match token.kind {
                TokenKind::CloseBrace => break,
                TokenKind::Ident => {}
                _ => {
                    self.emitter
                        .emit_unexpected_token(token, "a closing brace or identifier");
                    return None;
                }
            };
            let name = token;

            let token = self.advance_token()?;
            match token.kind {
                TokenKind::QuestionMark => {
                    optional = true;
                    self.pop(TokenKind::Colon)?;
                }
                TokenKind::Colon => {}
                _ => {
                    self.emitter
                        .emit_unexpected_token(token, "question mark or colon");
                    return None;
                }
            };

            let value = self.field_value()?;
            self.pop(TokenKind::Semi)?;

            fields.push(FieldDecl {
                name,
                typ: value,
                optional,
            })
        }

        let decl = FieldSetDecl { fields };
        Some(decl)
    }

    fn field_value(&mut self) -> Option<FieldType> {
        let token = self.advance_token()?;
        let field_value = match token.kind {
            TokenKind::Ident => FieldType::Ident(token),
            TokenKind::String => FieldType::String(token),
            TokenKind::Uint32 => FieldType::Uint32(token),
            TokenKind::Uint64 => FieldType::Uint64(token),
            TokenKind::Int32 => FieldType::Int32(token),
            TokenKind::Int64 => FieldType::Int64(token),
            TokenKind::Float32 => FieldType::Float32(token),
            TokenKind::Float64 => FieldType::Float64(token),
            TokenKind::Unknown => FieldType::Unknown(token),
            TokenKind::Struct => FieldType::Struct(token),
            TokenKind::Map => {
                let decl = self.map_decl()?;
                FieldType::Map(Box::new(decl))
            }
            _ => {
                self.emitter
                    .emit_unexpected_token(token, "a field value type");
                return None;
            }
        };
        Some(field_value)
    }

    // Set of key-value pairs within two braces that cannot be nested and can only
    // be strings or numbers
    fn annotation_field_set_decl(&mut self) -> Option<Vec<AnnotationFieldDecl>> {
        self.pop(TokenKind::OpenBrace)?;

        let mut fields = vec![];
        loop {
            let mut optional = false;
            let token = self.advance_token()?;
            match token.kind {
                TokenKind::CloseBrace => break,
                TokenKind::Ident => {}
                _ => {
                    self.emitter
                        .emit_unexpected_token(token, "closing brace or identifier");
                    return None;
                }
            };
            let name = token;

            let token = self.advance_token()?;
            match token.kind {
                TokenKind::QuestionMark => {
                    optional = true;
                    self.pop(TokenKind::Colon)?;
                }
                TokenKind::Colon => {}
                _ => {
                    self.emitter
                        .emit_unexpected_token(token, "a question mark or colon");
                    return None;
                }
            };

            let value = self.annotation_field_value()?;
            self.pop(TokenKind::Comma)?;

            fields.push(AnnotationFieldDecl {
                name,
                value,
                optional,
            })
        }

        Some(fields)
    }

    fn annotation_field_value(&mut self) -> Option<AnnotationFieldValue> {
        let token = self.advance_token()?;
        let field_value = match token.kind {
            TokenKind::String => AnnotationFieldValue::String(token),
            TokenKind::Uint32 => AnnotationFieldValue::Uint32(token),
            TokenKind::Uint64 => AnnotationFieldValue::Uint64(token),
            TokenKind::Int32 => AnnotationFieldValue::Int32(token),
            TokenKind::Int64 => AnnotationFieldValue::Int64(token),
            TokenKind::Float32 => AnnotationFieldValue::Float32(token),
            TokenKind::Float64 => AnnotationFieldValue::Float64(token),
            _ => {
                self.emitter
                    .emit_unexpected_token(token, "a string or number type");
                return None;
            }
        };
        Some(field_value)
    }

    fn map_decl(&mut self) -> Option<MapDecl> {
        self.pop(TokenKind::OpenChevron)?;
        let key = self.field_value()?;
        self.pop(TokenKind::Comma)?;
        let value = self.field_value()?;
        self.pop(TokenKind::CloseChevron)?;
        Some(MapDecl { key, value })
    }

    fn pop(&mut self, kind: TokenKind) -> Option<Token> {
        let token = self.advance_token()?;
        if token.kind == kind {
            Some(token)
        } else {
            self.emitter
                .emit_unexpected_token(token, &format!("{kind}"));
            return None;
        }
    }

    fn advance_token(&mut self) -> Option<Token> {
        self.lexer.advance()
    }
}
