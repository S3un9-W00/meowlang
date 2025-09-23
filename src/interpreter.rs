use crate::parser::AstNode;
use std::collections::HashMap;

pub fn run(ast: Vec<AstNode>) {
    let mut vars: HashMap<String, String> = HashMap::new();

    for node in ast {
        match node {
            AstNode::VarString(name, val) => {
                vars.insert(name, val);
            }
            AstNode::VarNumber(name, val) => {
                vars.insert(name, val.to_string());
            }
            AstNode::PrintExpr(expr) => {
                if let Some(v) = vars.get(&expr) {
                    println!("{}", v);
                } else {
                    println!("{}", expr);
                }
            }
            AstNode::IfStmt(var, op, val, block) => {
                if let Some(v) = vars.get(&var) {
                    if let Ok(num) = v.parse::<i64>() {
                        let cond_val = val.parse::<i64>().unwrap_or(0);
                        let cond = match op.as_str() {
                            ">" => num > cond_val,
                            "<" => num < cond_val,
                            "==" => num == cond_val,
                            "!=" => num != cond_val,
                            _ => false,
                        };
                        if cond {
                            run(block);
                        }
                    }
                }
            }
        }
    }
}
