mod ast;
mod lexer;
mod parser;
mod token;

const INPUT: &'static str = r#"
package http.request;

import "testing";
import "http";

@singleton
constructor api {
    map<string, Endpoint>
}
"#;

const _INPUT_2: &'static str = r#"
@singleton
constructor api {
    repeatable {
        get: Endpoint;
        post: Endpoint;
        update: Endpoint;
        delete: Endpoint;
    }
}

struct Endpoint {
    name: string;
    request: type;
    response: type;
}
"#;

fn main() {
    let mut parser = parser::Parser::new(INPUT);
    loop {
        let stmt = parser.advance().unwrap();
        println!("{:?}", stmt);
        match stmt {
            ast::Stmt::Eof => break,
            _ => continue,
        }
    }
}
