use crate::lexer::Token;

#[derive(Debug)]
pub enum AstNode {
    VarString(String, String),
    VarNumber(String, i64),
    PrintExpr(String),
    IfStmt(String, String, String, Vec<AstNode>), // var op value, block
}

pub fn parse(tokens: Vec<Token>) -> Vec<AstNode> {
    let mut ast = Vec::new();
    let mut i = 0;

    while i < tokens.len() {
        match &tokens[i] {
            Token::StringVar => {
                if let (Some(Token::Ident(name)), Some(Token::StringLit(val))) =
                    (tokens.get(i + 1), tokens.get(i + 2))
                {
                    ast.push(AstNode::VarString(name.clone(), val.clone()));
                }
                // 변수 선언 뒤 세미콜론까지 건너뛰기
                i += 4; // StringVar + Ident + StringLit + Semicolon
            }
            Token::NumberVar => {
                if let (Some(Token::Ident(name)), Some(Token::Number(val))) =
                    (tokens.get(i + 1), tokens.get(i + 2))
                {
                    ast.push(AstNode::VarNumber(name.clone(), *val));
                }
                // 변수 선언 뒤 세미콜론까지 건너뛰기
                i += 4; // NumberVar + Ident + Number + Semicolon
            }
            Token::Print => {
                if let Some(next) = tokens.get(i + 1) {
                    match next {
                        Token::StringLit(s) | Token::Ident(s) => {
                            ast.push(AstNode::PrintExpr(s.clone()));
                        }
                        _ => {}
                    }
                }
                // Print + 값 + 세미콜론
                i += 3;
            }
            Token::If => {
                if let (Some(Token::Ident(var)), Some(Token::Op(op)), Some(Token::Number(val)), Some(Token::LBrace)) =
                    (tokens.get(i + 1), tokens.get(i + 2), tokens.get(i + 3), tokens.get(i + 4))
                {
                    // '{' 이후 블록 수집
                    let mut j = i + 5;
                    let mut block = Vec::new();
                    while j < tokens.len() && tokens[j] != Token::RBrace {
                        match &tokens[j] {
                            Token::Print => {
                                if let Some(Token::StringLit(s)) = tokens.get(j + 1) {
                                    block.push(AstNode::PrintExpr(s.clone()));
                                } else if let Some(Token::Ident(s)) = tokens.get(j + 1) {
                                    block.push(AstNode::PrintExpr(s.clone()));
                                }
                                j += 3; // Print + 값 + 세미콜론
                            }
                            _ => j += 1,
                        }
                    }
                    ast.push(AstNode::IfStmt(var.clone(), op.clone(), val.to_string(), block));
                    i = j + 1; // RBrace 이후 토큰으로 이동
                } else {
                    i += 1;
                }
            }
            _ => i += 1,
        }
    }
    ast
}