#[derive(Debug, Clone, PartialEq)]
pub enum AST {
    Num(usize),
    Add(Box<AST>, Box<AST>),
    If {
        // if における条件式
        cond: Box<AST>,
        // true だったときに eval する ast
        then: Box<AST>,
        // false だったときに eval する ast
        els: Box<AST>,
    },
}

pub fn eval(ast: AST) -> usize {
    match ast {
        AST::Num(v) => v,
        AST::Add(left, right) => {
            let left_obj = eval(*left);
            let right_obj = eval(*right);
            left_obj + right_obj
        }
        AST::If { cond, then, els } => match eval(*cond) {
            0 => eval(*els),
            _ => eval(*then),
        },
    }
}

// 関数呼び出しは型や引数が一致していないと呼び出せないが
// マクロは型も引数の個数も一致してなくても呼び出せる
#[macro_export]
macro_rules! ast {
    ((+ $left:tt $right:tt)) => {
        // このマクロの中でASTやpubにしてるやつを使いたいときは
        // `$crate::`っておまじないをつけてください:pray:
        $crate::AST::Add(Box::new(ast!($left)), Box::new(ast!($right)))
    };
    ((If $cond:tt $then:tt $els:tt)) => {
        $crate::AST::If {
            cond: Box::new(ast!($cond)),
            then: Box::new(ast!($then)),
            els: Box::new(ast!($els)),
        }
    };
    ($num:expr) => {
        $crate::AST::Num($num)
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_eval() {
        let ast = AST::Num(1);
        assert_eq!(eval(ast), 1);

        // (1 + 2)
        // (+ 1 2)
        let simple_add = AST::Add(Box::new(AST::Num(1)), Box::new(AST::Num(2)));
        assert_eq!(eval(simple_add), 3);

        // ((((1 + 2) + 3) + 4) + 5)
        // (+ (+ (+ (+ 1 2) 3) 4 ) 5)
        let complicated_add = AST::Add(
            Box::new(AST::Add(
                Box::new(AST::Add(
                    Box::new(AST::Add(Box::new(AST::Num(1)), Box::new(AST::Num(2)))),
                    Box::new(AST::Num(3)),
                )),
                Box::new(AST::Num(4)),
            )),
            Box::new(AST::Num(5)),
        );

        assert_eq!(eval(complicated_add), 15);
    }

    #[test]
    fn test_eval_if() {
        // 条件式の値が 0 なので els の 200
        let ast = ast!((If 0 100 200));
        assert_eq!(eval(ast), 200);

        // 条件式の値が 0 じゃないので then の 100
        let ast = ast!((If 1 100 200));
        assert_eq!(eval(ast), 100);

        // 同様に条件式の値が 0 じゃないので then の 100
        let ast = ast!((If (+ 0 3) 100 200));
        assert_eq!(eval(ast), 100);
    }

    #[test]
    fn test_ast_macro() {
        assert_eq!(
            ast!((+ 1 2)),
            AST::Add(Box::new(AST::Num(1)), Box::new(AST::Num(2)))
        );

        assert_eq!(
            ast!((+ (+ (+ (+ 1 2) 3) 4 ) 5)),
            AST::Add(
                Box::new(AST::Add(
                    Box::new(AST::Add(
                        Box::new(AST::Add(Box::new(AST::Num(1)), Box::new(AST::Num(2)))),
                        Box::new(AST::Num(3)),
                    )),
                    Box::new(AST::Num(4)),
                )),
                Box::new(AST::Num(5)),
            )
        );

        assert_eq!(
            ast!((If 0 1 2)),
            AST::If {
                cond: Box::new(AST::Num(0)),
                then: Box::new(AST::Num(1)),
                els: Box::new(AST::Num(2))
            }
        )
    }
}
