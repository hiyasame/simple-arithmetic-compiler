use std::cell::{RefCell};
use std::ops::Deref;
use std::rc::Rc;
use crate::tokenizer::Token;

pub type ASTNodeRef = Rc<RefCell<ASTNode>>;

#[derive(Debug, Clone)]
pub struct ASTNode {
    pub token: Token,
    pub left: Option<ASTNodeRef>,
    pub right: Option<ASTNodeRef>
}

impl ASTNode {
    fn new_ref(token: Token) -> ASTNodeRef {
        Rc::new(RefCell::new(ASTNode { token, left: None, right: None }))
    }

    pub fn to_ref(self) -> ASTNodeRef {
        Rc::new(RefCell::new(self))
    }

    pub fn print(&self) {
        self.print_node(0)
    }

    fn print_node(&self, indent_level: i32) {
        for _ in 0..indent_level {
            print!("  ");
        }
        println!("- {:?}", self.token);
        if let Some(r) = &self.left {
            println!("left");
            r.borrow().print_node(indent_level + 1)
        }
        if let Some(r) = &self.right {
            println!("right");
            r.borrow().print_node(indent_level + 1)
        }
    }
}

impl Token {
    fn get_token_priority(&self) -> u8 {
        match self {
            Token::NUMBER(_) => 5,
            Token::PAREN(paren) => {
                if paren.clone() == '(' {
                    1
                } else {
                    6
                }
            }
            Token::OPERATOR(op) => {
                if op.clone() == '+' || op.clone() == '-' {
                    2
                } else {
                    3
                }
            }
        }
    }
}

// 递归下降解析，生成语法分析树
// 验证语法是否正确
// 先把文法列出来，然后按照文法递归一圈
pub fn parse(tokens: Vec<Token>) -> bool {
    let exprs = convert(tokens.clone());

    println!("{:?}", exprs);

    let mut index = 0;
    let mut next_word = || {
        index += 1;
        tokens.get(index - 1).map(|token| token.clone())
    };

    let mut word = next_word();

    // Expr -> Term ExprTail
    fn expr<F: FnMut() -> Option<Token>>(word: &mut Option<Token>, next_word: &mut F) -> bool {
        if !term(word, next_word) {
            false
        } else {
            expr_tail(word, next_word)
        }
    }

    // ExprTail ->  + Term ExprTail
    //              - Term ExprTail
    //              null
    fn expr_tail<F: FnMut() -> Option<Token>>(word: &mut Option<Token>, next_word: &mut F) -> bool {
        if let Some(Token::OPERATOR(operator)) = word.clone() {
            if operator.clone() == '+' || operator.clone() == '-' {

                *word = next_word();
                return if !term(word, next_word) {
                    false
                } else {
                    expr_tail(word, next_word)
                }
            }
        }
        true
    }

    // Term -> Factor TermTail
    fn term<F: FnMut() -> Option<Token>>(word: &mut Option<Token>, next_word: &mut F) -> bool {
        return if !factor(word, next_word) {
            false
        } else {
            term_tail(word, next_word)
        }
    }

    // TermTail ->  * Factor TermTail
    //              / Factor TermTail
    //              null
    fn term_tail<F: FnMut() -> Option<Token>>(word: &mut Option<Token>, next_word: &mut F) -> bool {
        if let Some(Token::OPERATOR(operator)) = word.clone() {
            if operator == '*' || operator == '/' {

                *word = next_word();
                return if !factor(word, next_word) {
                    false
                } else {
                    term_tail(word, next_word)
                }
            }
        }
        true
    }

    // Factor -> (Expr)
    //           num
    fn factor<F: FnMut() -> Option<Token>>(word: &mut Option<Token>, next_word: &mut F) -> bool {
        if let Some(Token::PAREN(paren)) = word.clone() {
            if paren == '(' {
                *word = next_word();
                return if !expr(word, next_word) {
                    false
                } else if let Some(Token::PAREN(paren)) = word.clone() && paren == ')' {
                    *word = next_word();
                    true
                } else {
                    false
                }
            }
        } else if let Some(Token::NUMBER(_)) = word.clone() {

            // 解析为字面值
            *word = next_word();
            return true
        }
        false
    }

    expr(&mut word, &mut next_word) && next_word().is_none()
}

// 转换后缀表达式
fn convert(token: Vec<Token>) -> Vec<Token> {
    let mut stack = Vec::<Token>::new();
    let mut output = Vec::new();
    token.iter().for_each(|token| {
        match token {
            Token::NUMBER(_) => {
                // 数字直接输出
                output.push(token.clone())
            }
            Token::OPERATOR(_) => {
                if let Some(top) = stack.pop() {
                    // 栈顶元素优先级大于等于准备压入的元素 一直出栈到栈顶为优先级小于当前token优先级的元素
                    if top.get_token_priority() >= token.get_token_priority() {
                        output.push(top);
                        while !stack.is_empty() {
                            let pop = stack.pop().unwrap();
                            if pop.get_token_priority() < token.get_token_priority() {
                                stack.push(pop);
                                break
                            }
                            output.push(pop)
                        }
                    } else {
                        // 因为刚才pop了，给他push回去
                        stack.push(top)
                    }
                }
                stack.push(token.clone())
            }
            Token::PAREN(paren) => {
                if paren.clone() == '(' {
                    // 直接入栈
                    stack.push(token.clone())
                } else {
                    // 出栈到第一个 (
                    let mut pop: Option<char> = None;
                    while pop.unwrap_or('?') != '(' {
                        let pop_token = stack.pop().unwrap();
                        if let Token::PAREN(_) = pop_token {
                            // 括号不输出
                        } else {
                            output.push(pop_token.clone());
                        }
                        match pop_token {
                            Token::PAREN(char) => {
                                pop = Some(char)
                            }
                            _ => {}
                        }
                    }

                }
            }
        }
    });
    while !stack.is_empty() {
        output.push(stack.pop().unwrap());
    };
    output
}

// 将后缀表达式转换为AST
pub fn parse_ast(token: Vec<Token>) -> Option<ASTNode> {
    let mut reversed = convert(token);
    reversed.reverse();
    inner_parse_ast(&mut reversed, vec![]).map(|r| r.deref().borrow().clone())
}

fn inner_parse_ast(tokens: &mut Vec<Token>, mut stack: Vec<ASTNodeRef>) -> Option<ASTNodeRef> {
    while tokens.len() != 0 {
        let option = tokens.pop();
        if let Some(token) = option {
            // NumberLiteral为叶子节点
            if let Token::OPERATOR(_) = &token {
                // 拿出两颗树合并为一棵树
                let node = ASTNode::new_ref(token);
                node.borrow_mut().right = stack.pop();
                node.borrow_mut().left = stack.pop();
                stack.push(node)
            } else {
                stack.push(ASTNode::new_ref(token))
            }
        }
    }
    Some(stack[0].clone())
}