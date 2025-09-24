mod lexer;
mod parser;
mod interpreter;

use std::fs;

fn main() {
    // 1. 소스 코드 읽기
    let source = fs::read_to_string("example.meow")
        .expect("example.meow 파일을 읽을 수 없습니다.");

    // 2. 토큰화
    let tokens = lexer::tokenize(&source);
    println!("토큰: {:?}", tokens);

    // 3. 파싱 (AST 생성)
    let ast = parser::parse(tokens);
    println!("AST: {:?}", ast);

    // 4. 실행
    interpreter::run(ast);
}
