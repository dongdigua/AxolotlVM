use regex::{Regex, Captures};
use crate::frontend::token::{self, Token};

#[derive (Clone, PartialEq, Debug)]
pub enum Parsed {
    Token(Token),
    List(Vec<Parsed>),
}

#[derive (Debug)]
pub enum ParseError {
    MisMatchedBracket,  // or MisMatchedParentheses
    InvalidCharacter,
}


pub fn parse(input: &String) -> Result<Parsed, ParseError> {
    // from github.com/kanaka/mal/blob/master/process/guide.md#step-2-eval
    let re_parse =
        Regex::new(r#"[\s,]*([\[\]{}()']|"(?:\\.|[^\\"])*"?|;.*|[^\s\[\]{}('"`,;)]*)"#).unwrap();
    let caps: Vec<Captures> = re_parse.captures_iter(input).collect();
    //println!("{:?}", &caps);
    let mut brackets = (0, 0);
    for c in &caps {
        match &c[1] {
            "(" => brackets.0 += 1,
            ")" => brackets.1 += 1,
            _ => (),
        }
    }
    if brackets.0 != brackets.1 {
        return Err(ParseError::MisMatchedBracket)
    }

    // zhihu: zhuanlan.zhihu.com/p/260157026
    // 一个 List 没完成时，又有新的 List 要开始，旧的 List 用一个栈保存起来
    let mut stack = vec![];
    let mut list  = vec![];
    for i in &caps {
        // Captures[1] is the capture group
        match &i[1] {
            "(" => {
                stack.push(list);
                list = vec![];
            }
            ")" => {
                let mut nlist =  // 将上一个 list 出栈
                    match stack.pop() {
                        Some(nl) => nl,
                        None => return Err(ParseError::MisMatchedBracket),
                    };
                nlist.push(Parsed::List(list));        // 当前的 list 作为值存入
                list = nlist;
            }
            s => list.push(Parsed::Token(
                token::tokenlize(s.to_string())
            )),
        }
    };

    Ok(list[0].clone())
}


#[cfg (test)]
mod tests {
    use super::*;
    use super::Parsed::*;

    #[test]
    fn test_parse_str() {
        let input = "123".to_string();
        assert_eq!(Str("123".to_string()), parse(&input).unwrap());
    }

    #[test]
    fn test_simple_ast() {
        let input = "(+ 1 (+ 2 3))".to_string();
        assert_eq!(List(
            vec![Str("+".to_string()), Str("1".to_string()),
                 List(
                     vec![Str("+".to_string()), Str("2".to_string()), Str("3".to_string())]
                 )
            ]
        ), parse(&input).unwrap());
    }

    #[test]
    fn print_parse() {
        let input = r#"
("aaaa")
"#.to_string();
        println!("{:?}", parse(&input).unwrap());
    }


}

