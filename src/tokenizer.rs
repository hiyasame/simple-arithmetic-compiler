#[derive(Clone, Debug)]
pub enum Token {
    NUMBER(i64),
    OPERATOR(char),
    PAREN(char),
}

// 分词
pub fn tokenizer(str: &str) -> Vec<Token> {
    let mut result: Vec<Token> = vec![];
    let chars: Vec<char> = str.chars().collect();
    let mut number_str = String::new();
    for index in 0..chars.len() {
        let char = chars[index];
        if !is_target_char(char) {
            crate::error(format!("unable to parse char: {}", char).as_str())
        }
        if char.is_whitespace() { continue }
        if is_number(char) {
            // 是数字的话累加上去
            number_str.push(char);
            if chars.get(index + 1).is_none() {
                result.push(Token::NUMBER(number_str.parse().unwrap()));
                number_str.clear()
            }
        } else {
            // 不是的话解析为数字放入result，并清空number_str
            if number_str.len() > 0 {
                result.push(Token::NUMBER(number_str.parse().unwrap()))
            }
            number_str.clear()
        }
        if is_operator(char) {
            result.push(Token::OPERATOR(char))
        }
        if is_paren(char) {
            result.push(Token::PAREN(char))
        }
    }
    result
}

fn is_target_char(char: char) -> bool {
    return is_number(char) || is_operator(char) || is_paren(char) || char == ' ';
}

fn is_number(c: char) -> bool {
    c.is_ascii_digit()
}

fn is_operator(c: char) -> bool {
    ['+', '-', '*', '/'].contains(&c)
}

fn is_paren(c: char) -> bool {
    c == '(' || c == ')'
}