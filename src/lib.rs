#[derive(Debug, Clone, PartialEq)]
pub enum AST {
    Num(usize),
}

pub fn eval(ast: AST) -> usize {
    match ast {
        AST::Num(v) => v,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_eval() {
        let ast = AST::Num(1);
        assert_eq!(eval(ast), 1);
    }
}
