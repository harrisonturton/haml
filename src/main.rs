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
    repeatable {
        get?: int64,
        post?: string,
        update?: float32,
        delete?: float32,
    }
}

struct Endpoint {
    request: struct,
    response: struct,
}

"#;

const _INPUT_2: &'static str = r#"
@singleton
constructor "api" {
    repeatable {
        get: Endpoint,
        post: Endpoint,
        update: Endpoint,
        delete: Endpoint,
    }
}

struct Endpoint {
    name: string,
    request: type,
    response: type,
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
