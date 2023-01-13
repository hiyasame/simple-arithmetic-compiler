use std::ops::Deref;
use crate::parser::{ASTNode, ASTNodeRef};
use crate::tokenizer::Token;

// AST 转栈式计算机指令
pub fn transform(ast: ASTNode) -> String {
    let mut codes = Vec::new();
    internal_transform(Some(ast.to_ref()), &mut codes);
    codes.push("ret".to_string());
    codes.join("\n")
}

// 后序遍历
fn internal_transform(ast: Option<ASTNodeRef>, codes: &mut Vec<String>) {
    if ast.is_none() {
        return;
    }
    let r = ast.unwrap();
    internal_transform(r.deref().borrow().left.clone(), codes);
    internal_transform(r.deref().borrow().right.clone(), codes);
    match r.clone().deref().borrow().token {
        Token::NUMBER(num) => {
            codes.push(format!("push {}", num))
        }
        Token::OPERATOR(op) if op == '+' => {
            codes.push(format!("add"))
        }
        Token::OPERATOR(op) if op == '-' => {
            codes.push(format!("sub"))
        }
        Token::OPERATOR(op) if op == '*' => {
            codes.push(format!("mul"))
        }
        Token::OPERATOR(op) if op == '/' => {
            codes.push(format!("div"))
        }
        _ => {}
    }
}