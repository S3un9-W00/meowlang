#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    StringVar, // 야옹
    NumberVar, // 냐옹
    Print,     // 미야옹~
    If,        // 캣!
    Elif,      // 냥캣!
    Else,      // 냥
    While,      // 냥!
    Ident(String),
    StringLit(String),
    Number(i64),
    Op(String),
    LBrace,
    RBrace,
    Semicolon,
}

pub fn tokenize(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();

    // 기호 주변 공백 추가
    let replaced = input
        .replace("{", " { ")
        .replace("}", " } ")
        .replace(";", " ; ");

    let mut words = replaced.split_whitespace().peekable();

    while let Some(word) = words.next() {
        match word {
            "야옹" => tokens.push(Token::StringVar),
            "냐옹" => tokens.push(Token::NumberVar),
            "미야옹~" => tokens.push(Token::Print),
            "캣!" => tokens.push(Token::If),
            "냥캣!" => tokens.push(Token::Elif),
            "냥" => tokens.push(Token::Else),
            "냥!" => tokens.push(Token::While),
            "{" => tokens.push(Token::LBrace),
            "}" => tokens.push(Token::RBrace),
            ";" => tokens.push(Token::Semicolon),
            _ => {
                // 문자열 처리: 시작이 "이면 끝나는 "까지 읽기
                if word.starts_with("\"") {
                    let mut string_val = word.trim_start_matches('"').to_string();

                    // 현재 단어가 끝나는 따옴표가 없으면 계속 읽기
                    if !word.ends_with("\"") || word.len() == 1 {
                        while let Some(next_word) = words.next() {
                            string_val.push(' ');
                            string_val.push_str(next_word);
                            if next_word.ends_with("\"") {
                                break;
                            }
                        }
                    }

                    // 양쪽 따옴표 제거 후 토큰 추가
                    tokens.push(Token::StringLit(
                        string_val.trim_end_matches('"').to_string(),
                    ));
                }
                // 숫자 처리
                else if let Ok(n) = word.parse::<i64>() {
                    tokens.push(Token::Number(n));
                }
                // 연산자 처리
                else if [">", "<", "==", "!="].contains(&word) {
                    tokens.push(Token::Op(word.to_string()));
                }
                // 일반 식별자 처리
                else {
                    tokens.push(Token::Ident(word.to_string()));
                }
            }
        }
    }

    tokens
}