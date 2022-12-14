use super::evaluate::{Evaluate};


use crate::types::traits::{Category, ComplexType, DataType, PineStaticType, PineType, SecondType};
use crate::types::{Callable, PineRef};

#[derive(Debug)]
pub struct CallableEvaluate<'a> {
    // val: Box<dyn EvaluateVal<'a> + 'a>,
    create_val: fn() -> Evaluate<'a>,
    create_func: fn() -> Callable<'a>,
}

impl<'a> PartialEq for CallableEvaluate<'a> {
    fn eq(&self, other: &CallableEvaluate<'a>) -> bool {
        self.create_val == other.create_val && self.create_func == other.create_func
    }
}

impl<'a> PineStaticType for CallableEvaluate<'a> {
    fn static_type() -> (DataType, SecondType) {
        (DataType::CallableEvaluate, SecondType::Simple)
    }
}

impl<'a> PineType<'a> for CallableEvaluate<'a> {
    fn get_type(&self) -> (DataType, SecondType) {
        <Self as PineStaticType>::static_type()
    }

    fn category(&self) -> Category {
        Category::Complex
    }

    fn copy(&self) -> PineRef<'a> {
        PineRef::new_rc(CallableEvaluate {
            create_val: self.create_val,
            create_func: self.create_func,
        })
    }
}

impl<'a> ComplexType for CallableEvaluate<'a> {}

impl<'a> CallableEvaluate<'a> {
    pub fn new(
        create_val: fn() -> Evaluate<'a>,
        create_func: fn() -> Callable<'a>,
    ) -> CallableEvaluate<'a> {
        CallableEvaluate {
            create_val,
            create_func,
        }
    }

    pub fn create_eval(&self) -> Evaluate<'a> {
        (self.create_val)()
    }

    pub fn create(&self) -> Callable<'a> {
        (self.create_func)()
    }
}

impl<'a> Clone for CallableEvaluate<'a> {
    fn clone(&self) -> CallableEvaluate<'a> {
        CallableEvaluate {
            create_val: self.create_val,
            create_func: self.create_func,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::stat_expr_types::VarIndex;
    use crate::ast::syntax_type::FunctionType;
    use crate::runtime::context::{Context, ContextType as RunContextType, VarOperate};
    use crate::types::{RuntimeErr, Series};
    use std::mem;

    #[derive(Debug, Clone, PartialEq)]
    struct MyVal();

    impl<'a> EvaluateVal<'a> for MyVal {
        fn custom_name(&self) -> &str {
            "test"
        }

        fn call(&mut self, ctx: &mut dyn Ctx<'a>) -> Result<PineRef<'a>, RuntimeErr> {
            let close_index = VarIndex::new(*ctx.get_varname_index("close").unwrap(), 0);
            match ctx.get_var(close_index) {
                Some(close_val) => Ok(close_val.copy()),
                _ => Err(RuntimeErr::VarNotFound),
            }
        }

        fn copy(&self) -> Box<dyn EvaluateVal<'a>> {
            Box::new(self.clone())
        }
    }

    fn test_func<'a>(
        _context: &mut dyn Ctx<'a>,
        mut args: Vec<Option<PineRef<'a>>>,
        _func_type: FunctionType<'a>,
    ) -> Result<PineRef<'a>, RuntimeErr> {
        Ok(mem::replace(&mut args[0], None).unwrap())
    }

    #[test]
    fn evaluate_test() {
        let mut evaluate = CallableEvaluate::new(
            || Evaluate::new(Box::new(MyVal())),
            || Callable::new(Some(test_func), None),
        );
        let mut context = Context::new(None, RunContextType::Normal);
        context.init(2, 0, 0);
        context.set_varname_index("close", 0);
        context.create_var(0, PineRef::new(Series::from(Some(1f64))));

        assert_eq!(
            evaluate.get_type(),
            (DataType::CallableEvaluate, SecondType::Simple)
        );
        assert_eq!(
            evaluate.create_eval().call(&mut context),
            Ok(PineRef::new_rc(Series::from(Some(1f64))))
        );
    }
}
