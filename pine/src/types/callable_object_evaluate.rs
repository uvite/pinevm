use super::evaluate::Evaluate;
use super::{
    Callable, Category, ComplexType, DataType, PineClass, PineFrom, PineRef, PineStaticType,
    PineType, RuntimeErr, SecondType,
};
use crate::runtime::Ctx;

#[derive(Debug)]
pub struct CallObjEval<'a> {
    obj: Box<dyn PineClass<'a> + 'a>,
    create_val: fn() -> Evaluate<'a>,
    create_func: fn() -> Callable<'a>,
}

impl<'a> PartialEq for CallObjEval<'a> {
    fn eq(&self, other: &CallObjEval<'a>) -> bool {
        self.create_val == other.create_val
            && PartialEq::eq(&*self.obj, &*other.obj)
            && self.create_func == other.create_func
    }
}

impl<'a> PineStaticType for CallObjEval<'a> {
    fn static_type() -> (DataType, SecondType) {
        (DataType::CallableObjectEvaluate, SecondType::Simple)
    }
}

impl<'a> PineType<'a> for CallObjEval<'a> {
    fn get_type(&self) -> (DataType, SecondType) {
        <Self as PineStaticType>::static_type()
    }

    fn category(&self) -> Category {
        Category::Complex
    }

    fn copy(&self) -> PineRef<'a> {
        PineRef::new_rc(CallObjEval::new(
            self.obj.copy(),
            self.create_val,
            self.create_func,
        ))
    }
}

impl<'a> PineFrom<'a, CallObjEval<'a>> for CallObjEval<'a> {}

impl<'a> ComplexType for CallObjEval<'a> {}

impl<'a> CallObjEval<'a> {
    pub fn new(
        obj: Box<dyn PineClass<'a> + 'a>,
        create_val: fn() -> Evaluate<'a>,
        create_func: fn() -> Callable<'a>,
    ) -> CallObjEval<'a> {
        CallObjEval {
            create_val,
            obj,
            create_func,
        }
    }

    pub fn get(&self, context: &mut dyn Ctx<'a>, name: &str) -> Result<PineRef<'a>, RuntimeErr> {
        self.obj.get(context, name)
    }

    pub fn set(&self, name: &str, property: PineRef<'a>) -> Result<(), RuntimeErr> {
        self.obj.set(name, property)
    }

    pub fn create_eval(&self) -> Evaluate<'a> {
        (self.create_val)()
    }
    pub fn create(&self) -> Callable<'a> {
        (self.create_func)()
    }
}
impl<'a> Clone for CallObjEval<'a> {
    fn clone(&self) -> CallObjEval<'a> {
        CallObjEval {
            obj: self.obj.copy(),
            create_val: self.create_val,
            create_func: self.create_func,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::{downcast_pf, Callable, EvaluateVal, Int, Object, RefData};
    use super::*;
    use crate::ast::stat_expr_types::VarIndex;
    use crate::ast::syntax_type::FunctionType;
    use crate::ast::syntax_type::{SimpleSyntaxType, SyntaxType};
    use crate::runtime::context::{Context, ContextType, Ctx, VarOperate};
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

    struct A;
    impl<'a> PineClass<'a> for A {
        fn custom_type(&self) -> &str {
            "Custom A"
        }

        fn get(&self, _ctx: &mut dyn Ctx<'a>, name: &str) -> Result<PineRef<'a>, RuntimeErr> {
            match name {
                "int1" => Ok(PineRef::new_box(Some(1i64))),
                "int2" => Ok(PineRef::new_box(Some(2i64))),
                "float1" => Ok(PineRef::new_box(Some(1f64))),
                "float2" => Ok(PineRef::new_box(Some(2f64))),
                _ => Err(RuntimeErr::NotSupportOperator),
            }
        }

        fn set(&self, _n: &str, _p: PineRef<'a>) -> Result<(), RuntimeErr> {
            Err(RuntimeErr::NotSupportOperator)
        }

        fn copy(&self) -> Box<dyn PineClass<'a> + 'a> {
            Box::new(A)
        }
    }

    fn test_func<'a>(
        _context: &mut dyn Ctx<'a>,
        mut args: Vec<Option<PineRef<'a>>>,
        _func_type: FunctionType<'a>,
    ) -> Result<PineRef<'a>, RuntimeErr> {
        Ok(mem::replace(&mut args[0], None).unwrap())
    }
    const INT_TYPE: SyntaxType = SyntaxType::Simple(SimpleSyntaxType::Int);

    #[test]
    fn object_test() {
        let mut obj = CallObjEval::new(
            Box::new(A),
            || Evaluate::new(Box::new(MyVal())),
            || Callable::new(Some(test_func), None),
        );
        let mut context = Context::new(None, ContextType::Normal);
        context.init(2, 0, 0);
        context.set_varname_index("close", 0);
        context.create_var(0, PineRef::new(Series::from(Some(1f64))));

        assert_eq!(
            obj.get_type(),
            (DataType::CallableObjectEvaluate, SecondType::Simple)
        );
        assert_eq!(
            downcast_pf::<Int>(obj.get(&mut context, "int1").unwrap()).unwrap(),
            RefData::new_box(Some(1))
        );
        assert_eq!(
            obj.create_eval().call(&mut context),
            Ok(PineRef::new_rc(Series::from(Some(1f64))))
        );
        let mut callable = obj.create();

        assert_eq!(
            downcast_pf::<Int>(
                callable
                    .call(
                        &mut context,
                        vec![PineRef::new_box(Some(1))],
                        vec![],
                        FunctionType::new((vec![("arg1", INT_TYPE)], INT_TYPE))
                    )
                    .unwrap()
            )
            .unwrap(),
            RefData::new_box(Some(1))
        );
    }
}
