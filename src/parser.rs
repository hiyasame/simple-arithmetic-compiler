use std::cell::{Cell, RefCell};
use std::ops::Deref;
use std::rc::Rc;
use crate::parser::NodeType::{Expr, ExprTail, Factor, NumberLiteral, Root, Term, TermTail};
use crate::tokenizer::Token;

type NodeRef = Rc<RefCell<ParseTreeNode>>;

// parse tree node
#[derive(Clone)]
pub struct ParseTreeNode {
    pub node_type: NodeType,
    pub token: Option<Token>,
    pub child: Vec<NodeRef>
}

impl ParseTreeNode {
    fn new(node_type: NodeType) -> NodeRef {
        Rc::new(RefCell::new(Self { node_type, token: None, child: Vec::new() }))
    }

    pub fn print(&self) {
        self.print_node(0)
    }

    fn print_node(&self, indent_level: i32) {
        for _ in 0..indent_level {
            print!("  ");
        }
        println!("- {:?} {:?}", self.node_type, self.token);
        for child in &self.child {
            child.borrow().print_node(indent_level + 1)
        }
    }
}

#[derive(Clone, Debug)]
pub enum NodeType {
    Root,
    Expr,
    ExprTail,
    Term,
    TermTail,
    Factor,
    NumberLiteral
}

// 递归下降解析，生成语法分析树
// 先把文法列出来，然后按照文法递归一圈
pub fn parse(tokens: Vec<Token>) -> Option<ParseTreeNode> {
    let mut index = 0;
    let mut next_word = || {
        index += 1;
        tokens.get(index - 1).map(|token| token.clone())
    };

    let mut word = next_word();
    // 根节点
    let root = ParseTreeNode::new(Root);
    // 有 RefCell 所以不需要可变借用。RefCell 确保了内部可变性
    let mut p = root.clone();

    // Expr -> Term ExprTail
    fn expr<F: FnMut() -> Option<Token>>(p: &mut NodeRef, word: &mut Option<Token>, next_word: &mut F) -> bool {
        let node = ParseTreeNode::new(Expr);
        p.deref().borrow_mut().child.push(node.clone());
        *p = node.clone();
        if !term(p, word, next_word) {
            false
        } else {
            *p = node.clone();
            expr_tail(p, word, next_word)
        }
    }

    // ExprTail ->  + Term ExprTail
    //              - Term ExprTail
    //              null
    fn expr_tail<F: FnMut() -> Option<Token>>(p: &mut NodeRef, word: &mut Option<Token>, next_word: &mut F) -> bool {
        let node = ParseTreeNode::new(ExprTail);
        p.deref().borrow_mut().child.push(node.clone());
        *p = node.clone();
        if let Some(Token::OPERATOR(operator)) = word.clone() {
            if operator.clone() == '+' || operator.clone() == '-' {
                p.deref().borrow_mut().token = word.clone();
                *word = next_word();
                return if !term(p, word, next_word) {
                    false
                } else {
                    *p = node.clone();
                    expr_tail(p, word, next_word)
                }
            }
        }
        true
    }

    // Term -> Factor TermTail
    fn term<F: FnMut() -> Option<Token>>(p: &mut NodeRef, word: &mut Option<Token>, next_word: &mut F) -> bool {
        let node = ParseTreeNode::new(Term);
        p.deref().borrow_mut().child.push(node.clone());
        *p = node.clone();
        return if !factor(p, word, next_word) {
            false
        } else {
            *p = node.clone();
            term_tail(p, word, next_word)
        }
    }

    // TermTail ->  * Factor TermTail
    //              / Factor TermTail
    //              null
    fn term_tail<F: FnMut() -> Option<Token>>(p: &mut NodeRef, word: &mut Option<Token>, next_word: &mut F) -> bool {
        let node = ParseTreeNode::new(TermTail);
        p.deref().borrow_mut().child.push(node.clone());
        *p = node.clone();
        if let Some(Token::OPERATOR(operator)) = word.clone() {
            if operator == '*' || operator == '/' {
                p.deref().borrow_mut().token = word.clone();
                *word = next_word();
                return if !factor(p, word, next_word) {
                    false
                } else {
                    *p = node.clone();
                    return term_tail(p, word, next_word)
                }
            }
        }
        true
    }

    // Factor -> (Expr)
    //           num
    fn factor<F: FnMut() -> Option<Token>>(p: &mut NodeRef, word: &mut Option<Token>, next_word: &mut F) -> bool {
        let node = ParseTreeNode::new(Factor);
        p.deref().borrow_mut().child.push(node.clone());
        *p = node.clone();
        if let Some(Token::PAREN(paren)) = word.clone() {
            if paren == '(' {
                *word = next_word();
                return if !expr(p, word, next_word) {
                    false
                } else if let Some(Token::PAREN(paren)) = word.clone() && paren != ')' {
                    false
                } else {
                    *word = next_word();
                    true
                }
            }
        } else if let Some(Token::NUMBER(_)) = word.clone() {
            // 解析为字面值
            let num_node = ParseTreeNode::new(NumberLiteral);
            num_node.deref().borrow_mut().token = word.clone();
            p.deref().borrow_mut().child.push(num_node);
            *word = next_word();
            return true
        }
        false
    }

    if expr(&mut p, &mut word, &mut next_word) && next_word().is_none() {
        Some(root.borrow().clone())
    } else {
        None
    }
}