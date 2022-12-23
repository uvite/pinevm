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
    let a=vec![1,2,3,4];
    let b=vec![5,6,7,8];

    let c=b.into_iter().enumerate().for_each(|(i, x)| b.get(i)-a.get(i));
    println!("{:?}",c)

    // assert_eq!(blk.var_count, 3);
    // assert_eq!(blk.subctx_count, 5);
    // assert_eq!(blk.libfun_count, 0);
}