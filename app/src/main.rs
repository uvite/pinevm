use std::fmt::Debug;
use std::mem;
use pine::ast::error::PineErrorKind;
//use pine::ast::stat_expr_types::{Assignment, Block, Exp, RVVarName, Statement, VarIndex};
use pine::ast::syntax_type::{FunctionType, SimpleSyntaxType, SyntaxType};
use pine::runtime::{AnySeries, AnySeriesType, Callback, Context, ContextType, Ctx, DataSrc, downcast_ctx, Runner, VarOperate};
use pine::types::{Callable, downcast_pf, Float, PineFrom, PineRef, RefData, RuntimeErr, Series};



use pine::ast::input::{Input, Position, StrRange};
use pine::ast::name::VarName;
use pine::ast::stat_expr::block;
use pine::ast::state::{AstState, PineInputError};
use pine::ast::stat_expr_types::*;
use nom::Err;

use pine::ast::string::StringNode;
use pine::runtime::function::Function;
use pine::syntax::{ParseValue, SyntaxParser};

struct MyCallback;
impl Callback for MyCallback {}


fn parse_ast(in_str: &str) -> Result<Block, (Option<Block>, Vec<PineInputError>)> {
    let input = Input::new(in_str, Position::new(0, 0), Position::max());
    let state = AstState::new();
    match block(input.clone(), &state) {
        Ok((input, parsed)) => {
            if input.len() != 0 {
                state.catch(PineInputError::new(
                    PineErrorKind::NonRecongnizeStmt,
                    StrRange::new(
                        input.start,
                        Position::new(input.start.get_line(), std::u32::MAX),
                    ),
                ));
            }
            if state.is_ok() {
                Ok(parsed)
            } else {
                Err((Some(parsed), state.into_inner()))
            }
        }
        Err(Err::Error(pine_error)) => {
            state.merge_pine_error(pine_error);
            Err((None, state.into_inner()))
        }
        _ => {
            state.catch(PineInputError::new(
                PineErrorKind::UnknownErr,
                StrRange::new(Position::new(0, 0), Position::max()),
            ));
            Err((None, state.into_inner()))
        }
    }
}
const TEXT_WITH_COMMENT: &str = "//@version=4
study(\"Test\")
// This line is a comment
a = close // This is also a comment
plot(a)
";
#[test]
fn comment_test() {
    let b=pine::parse_ast(TEXT_WITH_COMMENT);
    println!("{:?}",b);
    assert_eq!(
        pine::parse_ast(TEXT_WITH_COMMENT),
        Ok(Block::new(
            vec![
                // Statement::None(StrRange::from_start("//@version=4\n", Position::new(0, 0))),
                Statement::Exp(Exp::FuncCall(Box::new(FunctionCall::new_no_ctxid(
                    Exp::VarName(RVVarName::new_with_start("study", Position::new(1, 0))),
                    vec![Exp::Str(StringNode::new(
                        String::from("Test"),
                        StrRange::from_start("Test", Position::new(1, 7))
                    ))],
                    vec![],
                    StrRange::from_start("study(\"Test\")", Position::new(1, 0))
                )))),
                // Statement::None(StrRange::from_start(
                //     "// This line is a comment\n",
                //     Position::new(2, 0)
                // )),
                Statement::Assignment(Box::new(Assignment::new(
                    vec![VarName::new_with_start("a", Position::new(3, 0))],
                    Exp::VarName(RVVarName::new_with_start("close", Position::new(3, 4))),
                    false,
                    None,
                    StrRange::from_start("a = close", Position::new(3, 0))
                ))),
                Statement::Exp(Exp::FuncCall(Box::new(FunctionCall::new_no_ctxid(
                    Exp::VarName(RVVarName::new_with_start("plot", Position::new(4, 0))),
                    vec![Exp::VarName(RVVarName::new_with_start(
                        "a",
                        Position::new(4, 5)
                    ))],
                    vec![],
                    StrRange::from_start("plot(a)", Position::new(4, 0))
                ))))
            ],
            None,
            StrRange::new(Position::new(1, 0), Position::new(4, 7))
        ))
    )
}

fn main() {
    const MACD_SCRIPT: &str = r#"
a= if 1
    2
   else
    4
b=for i=1 to 2
    i
c=3
myfun(x,y)=>x+y
myfun(1,2)
myfun(2,2)
myfun(5,2)

"#;
   const BLOCK: &str = "a = if 1\n    2\nelse\n    4\nb = for i = 1 to 2\n    i\nmyfun(x, y) => x + y\nmyfun(1, 2)\nmyfun(1, 2)";

    let mut parser = SyntaxParser::new();

    let input = Input::new_with_str(BLOCK);
    let myblk = block(input, &AstState::new());
    let mut blk = myblk.unwrap().1;
    parser.parse_blk(&mut blk);
    // assert_eq!(
    //     parser.parse_blk(&mut blk),
    //     Ok(ParseValue::new_with_type(SyntaxType::Void))
    // );
    println!("{},{},{}",blk.var_count,blk.subctx_count,blk.libfun_count);
    println!("{:?}",blk.stmts);

    let mut context = Context::new(None, ContextType::Normal);

    fn test_func<'a>(
        _context: &mut dyn Ctx<'a>,
        mut h: Vec<Option<PineRef<'a>>>,
        _type: FunctionType<'a>,
    ) -> Result<PineRef<'a>, RuntimeErr> {
        let arg1 = mem::replace(&mut h[0], None);
        let callable = downcast_pf::<Function>(arg1.unwrap()).unwrap();
        let names: Vec<_> = callable.get_def().params.iter().map(|s| s.value).collect();
        assert_eq!(names, vec!["close"]);
        // println!("start new context {:?}", callable.get_def());
        let mut subctx = Box::new(Context::new(Some(_context), ContextType::FuncDefBlock));
        subctx.init(
            callable.get_var_count(),
            callable.get_subctx_count(),
            callable.get_libfun_count(),
        );
        let result = callable.call(
            &mut *subctx,
            vec![PineRef::new_rc(Series::from(Some(10f64)))],
            vec![],
            StrRange::new_empty(),
        );
        match result {
            Ok(val) => Ok(val),
            Err(e) => Err(e.code),
        }
    }

    context.init(5, 0, 1);
    context.init_vars(vec![
        Some(PineRef::new_rc(Callable::new(Some(test_func), None))),
        Some(PineRef::new_rc(Series::from(Some(1f64)))),
        None,
        None,
        None,
    ]);

    let result = Runner::run(&blk, &mut context);
    // println!("result {:?}", result);
    assert!(result.is_ok());
    assert_eq!(
        context.move_var(VarIndex::new(4, 0)),
        Some(PineRef::new_rc(Series::from(Some(21f64))))
    );


    // assert_eq!(blk.var_count, 3);
    // assert_eq!(blk.subctx_count, 5);
    // assert_eq!(blk.libfun_count, 0);
}