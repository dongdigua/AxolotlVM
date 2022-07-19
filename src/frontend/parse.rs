use regex::{Regex, Captures};

#[derive (Clone, PartialEq, Debug)]
pub enum Parsed {
    Str(String),
    List(Vec<Parsed>),
}

pub fn parse(input: &String) -> Parsed {
    // from github.com/kanaka/mal/blob/master/process/guide.md#step-2-eval
    let re_parse =
        Regex::new(r#"[\s,]*([\[\]{}()']|"(?:\\.|[^\\"])*"?|;.*|[^\s\[\]{}('"`,;)]*)"#).unwrap();
    let caps: Vec<Captures> = re_parse.captures_iter(input).collect();

    println!("{:?}", caps);
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
                let mut nlist = stack.pop().unwrap();  // 将上一个 list 出栈
                nlist.push(Parsed::List(list));        // 当前的 list 作为值存入
                list = nlist;
            }
            s => list.push(Parsed::Str(s.to_string())),
        }
    };
    list[0].clone()
}


#[cfg (test)]
mod tests {
    use super::*;
    use super::Parsed::*;

    #[test]
    fn test_parse_str() {
        let input = "123".to_string();
        assert_eq!(Str("123".to_string()), parse(&input));
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
        ), parse(&input));
    }



}

