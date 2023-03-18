use super::lexer::Lexer;
use super::token::{Token, TokenKind};
use crate::ast::node::{
    AliasDecl, AnnotationDecl, AnnotationFieldDecl, AnnotationFieldValue, Ast, BlockDecl, Comment,
    ConstructorDecl, FieldDecl, FieldSetDecl, FieldType, ImportStmt, MapDecl, PackageStmt, Stmt,
    StructDecl,
};
use crate::error::SyntaxError;

/// Turns tokens into statements.
#[derive(Debug)]
pub struct Parser<'a> {
    lexer: Lexer<'a>,
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a str) -> Parser<'a> {
        let lexer = Lexer::new(input);
        Parser { lexer }
    }

    pub fn parse(&mut self) -> Result<Ast, SyntaxError> {
        let mut stmts = vec![];
        loop {
            let stmt = self.advance()?;
            if stmt == Stmt::Eof {
                break;
            }
            stmts.push(stmt);
        }
        Ok(Ast { stmts })
    }

    pub fn advance(&mut self) -> Result<Stmt, SyntaxError> {
        let token = self.advance_token()?;
        match token.kind {
            TokenKind::Package => self.package_stmt(),
            TokenKind::Import => self.import_stmt(),
            TokenKind::At => self.annotation_def(),
            TokenKind::Struct => self.struct_decl(vec![]),
            TokenKind::Constructor => self.constructor_decl(vec![]),
            TokenKind::Annotation => self.annotation_decl(vec![]),
            TokenKind::Comment => Ok(Stmt::Comment(Comment { value: token })),
            TokenKind::Eof => Ok(Stmt::Eof),
            _ => Err(unexpected_token(token, "a package, import or declaration")),
        }
    }

    fn package_stmt(&mut self) -> Result<Stmt, SyntaxError> {
        let mut segments = vec![];
        loop {
            let segment = self.pop(TokenKind::Ident)?;
            segments.push(segment);

            let token = self.advance_token()?;
            match token.kind {
                TokenKind::Period => continue,
                TokenKind::Semi => break,
                _ => return Err(unexpected_token(token, "period or semicolon")),
            };
        }
        let stmt = PackageStmt { segments };
        Ok(Stmt::PackageStmt(stmt))
    }

    fn import_stmt(&mut self) -> Result<Stmt, SyntaxError> {
        let path = self.pop(TokenKind::StringLiteral)?;
        self.pop(TokenKind::Semi)?;
        let stmt = ImportStmt { path };
        Ok(Stmt::ImportStmt(stmt))
    }

    fn annotation_def(&mut self) -> Result<Stmt, SyntaxError> {
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
                    return Err(unexpected_token(
                        token,
                        "an annotation, constructor or struct",
                    ))
                }
            };
        }
    }

    fn struct_decl(&mut self, annotations: Vec<Token>) -> Result<Stmt, SyntaxError> {
        let name = self.pop(TokenKind::Ident)?;
        let content = self.block_decl()?;
        let stmt = StructDecl {
            annotations,
            name,
            content,
        };
        Ok(Stmt::StructDecl(stmt))
    }

    fn constructor_decl(&mut self, annotations: Vec<Token>) -> Result<Stmt, SyntaxError> {
        let name = self.pop(TokenKind::Ident)?;
        let content = self.block_decl()?;
        let stmt = ConstructorDecl {
            annotations,
            name,
            content,
        };
        Ok(Stmt::ConstructorDecl(stmt))
    }

    fn block_decl(&mut self) -> Result<BlockDecl, SyntaxError> {
        self.pop(TokenKind::OpenBrace)?;
        let discriminator = self.advance_token()?;
        match discriminator.kind {
            TokenKind::Union => self.union_decl(),
            TokenKind::Repeatable => self.repeatable_decl(),
            TokenKind::Map => {
                let map = self.map_decl()?;
                self.pop(TokenKind::CloseBrace)?;
                let decl = AliasDecl::MapDecl(map);
                Ok(BlockDecl::Alias(decl))
            }
            TokenKind::Ident => {
                let fields = self.field_set_decl(Some(discriminator))?;
                Ok(BlockDecl::FieldSet(fields))
            }
            TokenKind::CloseBrace => {
                let fieldset = FieldSetDecl { fields: vec![] };
                Ok(BlockDecl::FieldSet(fieldset))
            }
            _ => Err(unexpected_token(
                discriminator,
                "union, repeatable or an identifier",
            )),
        }
    }

    fn union_decl(&mut self) -> Result<BlockDecl, SyntaxError> {
        self.pop(TokenKind::OpenBrace)?;
        let fields = self.field_set_decl(None)?;
        self.pop(TokenKind::CloseBrace)?;
        Ok(BlockDecl::FieldSet(fields))
    }

    fn repeatable_decl(&mut self) -> Result<BlockDecl, SyntaxError> {
        self.pop(TokenKind::OpenBrace)?;
        let fields = self.field_set_decl(None)?;
        self.pop(TokenKind::CloseBrace)?;
        Ok(BlockDecl::Repeatable(fields))
    }

    fn annotation_decl(&mut self, annotations: Vec<Token>) -> Result<Stmt, SyntaxError> {
        let name = self.pop(TokenKind::Ident)?;
        let fields = self.annotation_field_set_decl()?;
        let stmt = AnnotationDecl {
            annotations,
            name,
            fields,
        };
        Ok(Stmt::AnnotationDecl(stmt))
    }

    // Set of nested key-value pairs inside two braces. `leading_ident` is provided
    // so block_decl can call field_set_decl if it encounters a identifier. The
    // lexer doesn't support peeking so we have to work without this lookahead.
    fn field_set_decl(
        &mut self,
        leading_ident: Option<Token>,
    ) -> Result<FieldSetDecl, SyntaxError> {
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
                _ => return Err(unexpected_token(token, "a question mark or colon")),
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
                _ => return Err(unexpected_token(token, "a close brace or identifier")),
            };
            let name = token;

            let token = self.advance_token()?;
            match token.kind {
                TokenKind::QuestionMark => {
                    optional = true;
                    self.pop(TokenKind::Colon)?;
                }
                TokenKind::Colon => {}
                _ => return Err(unexpected_token(token, "question mark or colon")),
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
        Ok(decl)
    }

    fn field_value(&mut self) -> Result<FieldType, SyntaxError> {
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
            _ => return Err(unexpected_token(token, "a field value type")),
        };
        Ok(field_value)
    }

    // Set of key-value pairs within two braces that cannot be nested and can only
    // be strings or numbers
    fn annotation_field_set_decl(&mut self) -> Result<Vec<AnnotationFieldDecl>, SyntaxError> {
        self.pop(TokenKind::OpenBrace)?;

        let mut fields = vec![];
        loop {
            let mut optional = false;
            let token = self.advance_token()?;
            match token.kind {
                TokenKind::CloseBrace => break,
                TokenKind::Ident => {}
                _ => return Err(unexpected_token(token, "closing brace or identifier")),
            };
            let name = token;

            let token = self.advance_token()?;
            match token.kind {
                TokenKind::QuestionMark => {
                    optional = true;
                    self.pop(TokenKind::Colon)?;
                }
                TokenKind::Colon => {}
                _ => return Err(unexpected_token(token, "a question mark or colon")),
            };

            let value = self.annotation_field_value()?;
            self.pop(TokenKind::Comma)?;

            fields.push(AnnotationFieldDecl {
                name,
                value,
                optional,
            })
        }

        Ok(fields)
    }

    fn annotation_field_value(&mut self) -> Result<AnnotationFieldValue, SyntaxError> {
        let token = self.advance_token()?;
        let field_value = match token.kind {
            TokenKind::String => AnnotationFieldValue::String(token),
            TokenKind::Uint32 => AnnotationFieldValue::Uint32(token),
            TokenKind::Uint64 => AnnotationFieldValue::Uint64(token),
            TokenKind::Int32 => AnnotationFieldValue::Int32(token),
            TokenKind::Int64 => AnnotationFieldValue::Int64(token),
            TokenKind::Float32 => AnnotationFieldValue::Float32(token),
            TokenKind::Float64 => AnnotationFieldValue::Float64(token),
            _ => return Err(unexpected_token(token, "a string or number type")),
        };
        Ok(field_value)
    }

    fn map_decl(&mut self) -> Result<MapDecl, SyntaxError> {
        self.pop(TokenKind::OpenChevron)?;
        let key = self.field_value()?;
        self.pop(TokenKind::Comma)?;
        let value = self.field_value()?;
        self.pop(TokenKind::CloseChevron)?;
        Ok(MapDecl { key, value })
    }

    fn pop(&mut self, kind: TokenKind) -> Result<Token, SyntaxError> {
        let token = self.advance_token()?;
        if token.kind == kind {
            Ok(token)
        } else {
            Err(unexpected_token(token, &format!("{kind}")))
        }
    }

    fn advance_token(&mut self) -> Result<Token, SyntaxError> {
        self.lexer.advance()
    }
}

fn unexpected_token(actual: Token, msg: &str) -> SyntaxError {
    SyntaxError::UnexpectedToken(actual, msg.to_owned())
}
