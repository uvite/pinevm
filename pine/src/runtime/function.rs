use super::context::{downcast_ctx, Ctx, PineRuntimeError, Runner};
use super::statement::process_assign_val;
use crate::ast::input::StrRange;
use crate::ast::stat_expr_types::FunctionDef;
use crate::types::{
    Category, ComplexType, DataType, PineFrom, PineRef, PineStaticType, PineType, RuntimeErr,
    SecondType,
};


#[derive(Debug, PartialEq, Clone)]
pub struct Function<'a> {
    def: &'a FunctionDef<'a>,
}

impl<'a> PineStaticType for Function<'a> {
    fn static_type() -> (DataType, SecondType) {
        (DataType::Function, SecondType::Simple)
    }
}
impl<'a> PineType<'a> for Function<'a> {
    fn get_type(&self) -> (DataType, SecondType) {
        <Self as PineStaticType>::static_type()
    }

    fn category(&self) -> Category {
        Category::Complex
    }

    fn copy(&self) -> PineRef<'a> {
        PineRef::new_rc(self.clone())
    }
}

impl<'a> PineFrom<'a, Function<'a>> for Function<'a> {}

impl<'a> ComplexType for Function<'a> {}

impl<'a> Function<'a> {
    pub fn new(def: &'a FunctionDef<'a>) -> Function<'a> {
        Function { def }
    }

    pub fn call(
        &self,
        context: &mut dyn Ctx<'a>,
        pos_args: Vec<PineRef<'a>>,
        _dict_args: Vec<(&'a str, PineRef<'a>)>,
        range: StrRange,
    ) -> Result<PineRef<'a>, PineRuntimeError> {
        if pos_args.len() > self.def.params.len() {
            return Err(PineRuntimeError::new(RuntimeErr::NotValidParam, range));
        }

        // let mut all_args: HashMap<&'a str, PineRef<'a>> = HashMap::new();
        // for (i, val) in pos_args.into_iter().enumerate() {
        //     let name = self.def.params[i].value;
        //     all_args.insert(name, val);
        // }
        // for (name, val) in dict_args.into_iter() {
        //     match self.def.params.iter().any(|&v| name == v.value) {
        //         false => return Err(PineRuntimeError::new(RuntimeErr::NotValidParam, range)),
        //         true => {
        //             all_args.insert(name, val);
        //         }
        //     }
        // }
        // let mut new_context = Context::new(Some(context), ContextType::FuncDefBlock);
        for (i, v) in pos_args.into_iter().enumerate() {
            // context.create_var(k, v);
            if let Err(err) = process_assign_val(v, downcast_ctx(context), i as i32, None) {
                return Err(PineRuntimeError::new(err, range));
            }
        }
        self.def.body.run(context)
    }

    pub fn get_var_count(&self) -> i32 {
        self.def.get_var_count()
    }

    pub fn get_subctx_count(&self) -> i32 {
        self.def.get_subctx_count()
    }

    pub fn get_libfun_count(&self) -> i32 {
        self.def.get_libfun_count()
    }

    pub fn get_def(&self) -> &'a FunctionDef<'a> {
        self.def
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::input::StrRange;
    use crate::ast::name::VarName;
    use crate::ast::stat_expr_types::{Block, Exp, RVVarName, VarIndex};
    use crate::runtime::context::{Context, ContextType};
    use crate::types::series::Series;

    #[test]
    fn func_test() {
        let mut blk =
            Block::new_no_input(vec![], Some(Exp::VarName(RVVarName::new_no_range("arg1"))));
        blk.var_count = 1;
        blk.subctx_count = 0;

        let mut func_def = FunctionDef::new(
            VarName::new_no_input("hello"),
            vec![VarName::new_no_input("arg1")],
            blk,
            StrRange::new_empty(),
        );
        func_def.name_varid = 0;
        func_def.varids = Some(vec![0]);
        let func = Function::new(&func_def);
        let mut ctx = Context::new(None, ContextType::FuncDefBlock);
        ctx.init(
            func_def.get_var_count(),
            func_def.get_subctx_count(),
            func_def.get_libfun_count(),
        );

        assert_eq!(
            func.call(
                &mut ctx,
                vec![PineRef::new(Series::from(Some(1)))],
                vec![],
                StrRange::new_empty()
            ),
            Ok(PineRef::new(Series::from(Some(1))))
        );
        assert_eq!(
            func.call(
                &mut ctx,
                vec![PineRef::new(Series::from(Some(10)))],
                vec![],
                StrRange::new_empty()
            ),
            Ok(PineRef::new(Series::from(Some(10))))
        );
        ctx.commit();
        assert_eq!(
            func.call(
                &mut ctx,
                vec![PineRef::new(Series::from(Some(100)))],
                vec![],
                StrRange::new_empty()
            ),
            Ok(PineRef::new(Series::from_cur_history(
                Some(100),
                vec![Some(10)]
            )))
        );
    }
}
