use crate::lexer::Token;

#[derive(Debug)]
pub enum ASTNode {
    VarString(String, String), // 변수 이름, 값
    VarNumber(String, i64),   // 변수 이름, 값
    PrintExpr(String),      // 출력할 표현식
    IfStmt(String, String, String, Vec<ASTNode>), // 조건, 참일 때 표현식, 거짓일 때 표현식, 블록
}

pub fn parse(token: Vec<Token>) -> Vec<ASTNode> {
    let mut ast = Vec::new();
    let mut i = 0;

    while i < token.len() {
        match &token[i] {
            Token::StringVar => {
                if let (Some(Token::Ident(name)), Some(Token::StringLit(val))) =
                    (token.get(i + 1), token.get(i + 2))
                {
                    ast.push(ASTNode::VarString(name.clone(), val.clone()));
                } 

                // 변수 선언 후 세미콜론까지 건너뛰기
                i += 4; 
            }
            Token::NumberVar => {
                if let (Some(Token::Ident(name)), Some(Token::Number(val))) =
                    (token.get(i + 1), token.get(i + 2))
                {
                    ast.push(ASTNode::VarNumber(name.clone(), *val));
                } 

                // 변수 선언 후 세미콜론까지 건너뛰기
                i += 4; 
            }
            Token::Print => {
                if let Some(next) = token.get(i + 1) {
                    match next {
                        Token::StringLit(s) | Token::Ident(s) => {
                            ast.push(ASTNode::PrintExpr(s.clone()));
                        }
                        _ => {}
                    }
                }
                
                // print + 값 + 세미콜론
                i += 3;
        }

        Token::If => {
            if let (Some(Token::Ident(var)), Some(Token::Op(op)), Some(Token::Number(val)), Some(Token::LBrace)) =
                (token.get(i + 1), token.get(i + 2), token.get(i + 3), token.get(i + 4))
            {
                let mut j = i + 5;
                let mut block = Vec::new();

                // 블록 내 토큰 파싱
                while j < token.len() && token[j] != Token::RBrace {
                    match &token[j] {
                        Token::Print => {
                            if let Some(Token::StringLit(s)) = token.get(j + 1) {
                                block.push(ASTNode::PrintExpr(s.clone()));
                            } else if let Some(Token::Ident(s)) = token.get(j + 1) {
                                block.push(ASTNode::PrintExpr(s.clone()));
                            }
                            j += 3; // print + 값 + 세미콜론
                        }
                        _ => j += 1,
                    }
                    ast.push(ASTNode::IfStmt(var.clone(), op.clone(), val.to_string(), block));
                    i = j + 1;
                }
            }
            else {
                i += 1;
            }
            
        }
        _ => i += 1,
    }
    
}
ast
}