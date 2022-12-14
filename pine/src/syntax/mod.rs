use crate::ast::error::PineErrorKind;
use crate::ast::input::StrRange;
use crate::ast::name::VarName;
use crate::ast::num::Numeral;
use crate::ast::op::{BinaryOp, UnaryOp};
use crate::ast::stat_expr_types::{
    Assignment, BinaryExp, Block, Condition, Exp, ForRange, FunctionCall, FunctionDef,
    IfThenElse, PrefixExp, RVVarName, RefCall, Statement, TupleNode, TypeCast, UnaryExp,
    VarAssignment, VarIndex,
};
use crate::ast::state::PineInputError;
use crate::ast::syntax_type::{FunctionTypes, SimpleSyntaxType, SyntaxType};
use std::collections::HashMap;
use std::convert::From;
use std::mem;
use std::ptr::NonNull;
use std::rc::Rc;

mod convert;
pub mod ctxid_parser;
mod input_detector;
mod name_rel_parser;
mod type_cast;
pub mod types_id_gen;

pub use input_detector::*;
use name_rel_parser::*;

use convert::{common_type, implicity_convert, similar_type, simple_to_series};
use type_cast::{explicity_type_cast, implicity_type_cast};
use types_id_gen::TypesIdGen;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ContextType {
    Library,
    Main,
    Normal,
    IfElseBlock,
    ForRangeBlock,
    FuncDefBlock,
}

pub trait SyntaxCtx<'a> {
    fn declare_var(&mut self, name: &'a str, t: SyntaxType<'a>);

    fn declare_var_with_index(&mut self, name: &'a str, t: SyntaxType<'a>);

    fn update_var(&mut self, name: &'a str, t: SyntaxType<'a>);

    fn get_var(&self, name: &'a str) -> Option<&SyntaxType<'a>>;

    fn get_var_scope(&self, name: &'a str) -> Option<&SyntaxType<'a>>;

    fn declare_user_func(&mut self, name: String, t: SyntaxType<'a>, index: i32);

    fn get_user_func(&self, name: &str) -> Option<&(SyntaxType<'a>, i32)>;

    fn declare_inputnames(&mut self, name: &'a str);

    fn get_inputnames(&self) -> Vec<&'a str>;

    fn gen_var_index(&mut self, name: &str) -> i32;

    fn gen_var_index_only(&mut self) -> i32;

    fn get_var_index(&mut self, name: &str) -> VarIndex;

    fn contain_var_index(&self, name: &str) -> bool;

    fn contain_var_index_scope(&self, name: &str) -> bool;

    fn gen_child_ctx_index(&mut self) -> i32;

    fn gen_lib_func_index(&mut self) -> i32;
}

pub struct SyntaxContext<'a> {
    parent: Option<NonNull<(dyn SyntaxCtx<'a> + 'a)>>,
    context_type: ContextType,

    subctxs: Vec<Box<dyn SyntaxCtx<'a> + 'a>>,

    vars: HashMap<&'a str, SyntaxType<'a>>,
    // User defined function's type for the specific name, the name is like method_name@id
    user_func_types: HashMap<String, (SyntaxType<'a>, i32)>,

    // input sources detector to scan input sources.
    input_detector: Option<*const dyn InputSrcDetector<'a>>,
    // The input varname(e.g. close, open etc) that the script use
    input_varnames: Vec<&'a str>,

    // The variable name to index map that can transfer the map lookup to vector getter.
    var_indexs: HashMap<String, i32>,
    // The max index of the current context
    max_var_index: i32,

    // The max index of children context
    max_child_ctx_index: i32,

    // The max index of library function
    max_lib_func_index: i32,
}

unsafe impl<'a> Send for SyntaxContext<'a> {}

pub fn downcast_ctx<'a>(item: *mut (dyn SyntaxCtx<'a> + 'a)) -> &mut SyntaxContext<'a> {
    unsafe {
        let raw: *mut dyn SyntaxCtx<'a> = item;
        let t = raw as *mut SyntaxContext<'a>;
        t.as_mut().unwrap()
    }
}

impl<'a> SyntaxCtx<'a> for SyntaxContext<'a> {
    fn declare_var(&mut self, name: &'a str, t: SyntaxType<'a>) {
        self.vars.insert(name, t);
    }

    fn update_var(&mut self, name: &'a str, t: SyntaxType<'a>) {
        if self.vars.contains_key(name) {
            self.vars.insert(name, t);
        } else if let Some(p) = self.parent {
            downcast_ctx(p.as_ptr()).update_var(name, t);
        } else {
            unreachable!();
        }
    }

    fn get_var(&self, name: &'a str) -> Option<&SyntaxType<'a>> {
        if self.vars.contains_key(name) {
            self.vars.get(name)
        } else if let Some(p) = self.parent {
            downcast_ctx(p.as_ptr()).get_var(name)
        } else {
            None
        }
    }

    fn get_var_scope(&self, name: &'a str) -> Option<&SyntaxType<'a>> {
        self.vars.get(name)
    }

    fn declare_user_func(&mut self, name: String, t: SyntaxType<'a>, index: i32) {
        self.user_func_types.insert(name, (t, index));
    }

    fn get_user_func(&self, name: &str) -> Option<&(SyntaxType<'a>, i32)> {
        if self.user_func_types.contains_key(name) {
            self.user_func_types.get(name)
        } else if let Some(p) = self.parent {
            downcast_ctx(p.as_ptr()).get_user_func(name)
        } else {
            None
        }
    }

    fn declare_var_with_index(&mut self, name: &'a str, t: SyntaxType<'a>) {
        self.declare_var(name, t);
        self.gen_var_index(name);
    }

    fn declare_inputnames(&mut self, name: &'a str) {
        if self.context_type == ContextType::Main {
            if self.input_varnames.iter().all(|&x| x != name) {
                self.input_varnames.push(name)
            }
        } else if let Some(p) = self.parent {
            downcast_ctx(p.as_ptr()).declare_inputnames(name)
        } else {
            unreachable!();
        }
    }

    fn get_inputnames(&self) -> Vec<&'a str> {
        self.input_varnames.clone()
    }

    fn gen_var_index(&mut self, name: &str) -> i32 {
        self.max_var_index += 1;
        debug_assert!(!self.var_indexs.contains_key(name));
        self.var_indexs
            .insert(String::from(name), self.max_var_index);
        self.max_var_index
    }

    fn gen_var_index_only(&mut self) -> i32 {
        self.max_var_index += 1;
        self.max_var_index
    }

    fn get_var_index(&mut self, name: &str) -> VarIndex {
        if self.var_indexs.contains_key(name) {
            VarIndex::new(self.var_indexs[name], 0)
        } else if let Some(p) = self.parent {
            let mut var_index = downcast_ctx(p.as_ptr()).get_var_index(name);
            var_index.rel_ctx += 1;
            var_index
        } else {
            unreachable!();
        }
    }

    fn contain_var_index(&self, name: &str) -> bool {
        if self.var_indexs.contains_key(name) {
            true
        } else if let Some(p) = self.parent {
            downcast_ctx(p.as_ptr()).contain_var_index(name)
        } else {
            false
        }
    }

    fn contain_var_index_scope(&self, name: &str) -> bool {
        self.var_indexs.contains_key(name)
    }

    fn gen_child_ctx_index(&mut self) -> i32 {
        self.max_child_ctx_index += 1;
        self.max_child_ctx_index
    }

    fn gen_lib_func_index(&mut self) -> i32 {
        self.max_lib_func_index += 1;
        self.max_lib_func_index
    }
}

impl<'a> SyntaxContext<'a> {
    pub fn new(
        parent: Option<NonNull<(dyn SyntaxCtx<'a> + 'a)>>,
        context_type: ContextType,
    ) -> SyntaxContext<'a> {
        SyntaxContext {
            parent,
            context_type,
            subctxs: vec![],
            user_func_types: HashMap::new(),
            vars: HashMap::new(),
            input_detector: None,
            input_varnames: vec![],
            var_indexs: HashMap::new(),
            max_var_index: -1,
            max_child_ctx_index: -1,
            max_lib_func_index: -1,
        }
    }

    pub fn set_input_detector(&mut self, detector: *const dyn InputSrcDetector<'a>) {
        debug_assert_eq!(self.context_type, ContextType::Main);
        self.input_detector = Some(detector);
    }

    pub fn is_client_input(&self, name: &'a str) -> bool {
        if self.context_type == ContextType::Main {
            if let Some(input_detector) = self.input_detector {
                // options.iter().position(|&x| x == name).is_some()
                let detector = unsafe { input_detector.as_ref().unwrap() };
                detector.map_client_src(name).is_some()
            } else {
                false
            }
        } else if let Some(p) = self.parent {
            downcast_ctx(p.as_ptr()).is_client_input(name)
        } else {
            unreachable!();
        }
    }

    pub fn map_input_src(&self, name: &'a str) -> Option<&'a str> {
        if self.context_type == ContextType::Main {
            if let Some(input_detector) = self.input_detector {
                // options.iter().position(|&x| x == name).is_some()
                let detector = unsafe { input_detector.as_ref().unwrap() };
                detector.map_input_src(name)
            } else {
                None
            }
        } else if let Some(p) = self.parent {
            downcast_ctx(p.as_ptr()).map_input_src(name)
        } else {
            unreachable!()
        }
    }

    pub fn get_type(&self) -> ContextType {
        self.context_type
    }

    pub fn get_parent(&self) -> Option<*mut (dyn SyntaxCtx<'a> + 'a)> {
        match self.parent {
            Some(p) => Some(p.as_ptr()),
            None => None,
        }
    }
}

pub struct SyntaxParser<'a> {
    _lib_ctx: Box<SyntaxContext<'a>>,
    _root_ctx: Box<SyntaxContext<'a>>,
    context: *mut (dyn SyntaxCtx<'a> + 'a),

    name_rel_parser: ExpNameRelParser<'a>,

    func_defs: Vec<FunctionDef<'a>>,
    // user defined function name to function definition map
    user_funcs: HashMap<&'a str, *mut FunctionDef<'a>>,
    // The types id generator that generate same id for the same types.
    types_id_gen: TypesIdGen<'a>,
    errors: Vec<PineInputError>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ParseValue<'a> {
    pub syntax_type: SyntaxType<'a>,
    pub varname: Option<&'a str>,
}

impl<'a> ParseValue<'a> {
    pub fn new(syntax_type: SyntaxType<'a>, varname: &'a str) -> ParseValue<'a> {
        ParseValue {
            syntax_type,
            varname: Some(varname),
        }
    }
    pub fn new_with_type(syntax_type: SyntaxType<'a>) -> ParseValue<'a> {
        ParseValue {
            syntax_type,
            varname: None,
        }
    }
}

type ParseResult<'a> = Result<ParseValue<'a>, PineInputError>;

impl<'a> SyntaxParser<'a> {
    pub fn new() -> SyntaxParser<'a> {
        let mut _lib_ctx = Box::new(SyntaxContext::new(None, ContextType::Library));
        let mut _root_ctx = Box::new(SyntaxContext::new(
            unsafe { Some(NonNull::new_unchecked(&mut *_lib_ctx)) },
            ContextType::Main,
        ));
        let mut name_rel_parser = ExpNameRelParser::new();
        name_rel_parser.enter_ctx(&mut *_root_ctx, 0);
        SyntaxParser {
            context: &mut *_root_ctx,
            _lib_ctx,
            _root_ctx,
            name_rel_parser,
            func_defs: vec![],
            user_funcs: HashMap::new(),
            types_id_gen: TypesIdGen::new(),
            errors: vec![],
        }
    }

    pub fn new_with_vars(vars: &Vec<(&'a str, SyntaxType<'a>)>) -> SyntaxParser<'a> {
        let mut _lib_ctx = Box::new(SyntaxContext::new(None, ContextType::Library));
        let mut _root_ctx = Box::new(SyntaxContext::new(
            unsafe { Some(NonNull::new_unchecked(&mut *_lib_ctx)) },
            ContextType::Main,
        ));
        for (k, _v) in vars.iter() {
            _root_ctx.gen_var_index(k);
        }
        _root_ctx.vars = vars.iter().cloned().collect();
        let mut name_rel_parser = ExpNameRelParser::new();
        name_rel_parser.enter_ctx(&mut *_root_ctx, 0);

        SyntaxParser {
            context: &mut *_root_ctx,
            _lib_ctx,
            _root_ctx,
            name_rel_parser,
            func_defs: vec![],
            user_funcs: HashMap::new(),
            types_id_gen: TypesIdGen::new(),
            errors: vec![],
        }
    }

    pub fn new_with_libvars(vars: &Vec<(&'a str, SyntaxType<'a>)>) -> SyntaxParser<'a> {
        let mut _lib_ctx = Box::new(SyntaxContext::new(None, ContextType::Library));
        for (k, _v) in vars.iter() {
            _lib_ctx.gen_var_index(k);
        }
        _lib_ctx.vars = vars.iter().cloned().collect();

        let mut _root_ctx = Box::new(SyntaxContext::new(
            unsafe { Some(NonNull::new_unchecked(&mut *_lib_ctx)) },
            ContextType::Main,
        ));
        let mut name_rel_parser = ExpNameRelParser::new();
        name_rel_parser.enter_ctx(&mut *_root_ctx, 0);

        SyntaxParser {
            context: &mut *_root_ctx,
            _lib_ctx,
            _root_ctx,
            name_rel_parser,
            func_defs: vec![],
            user_funcs: HashMap::new(),
            types_id_gen: TypesIdGen::new(),
            errors: vec![],
        }
    }

    pub fn init_input_detector(&mut self, detector: *const dyn InputSrcDetector<'a>) {
        downcast_ctx(self.context).set_input_detector(detector);
    }
    pub fn get_inputnames(&self) -> Vec<&'a str> {
        downcast_ctx(self.context).get_inputnames()
    }

    pub fn get_context(&mut self) -> *mut (dyn SyntaxCtx<'a> + 'a) {
        self.context
    }

    pub fn catch(&mut self, err: PineInputError) {
        self.errors.push(err)
    }

    pub fn is_ok(&self) -> bool {
        self.errors.is_empty()
    }

    pub fn move_errors(&mut self) -> Vec<PineInputError> {
        mem::replace(&mut self.errors, vec![])
    }

    pub fn move_context(&mut self) -> Box<SyntaxContext<'a>> {
        mem::replace(
            &mut self._root_ctx,
            Box::new(SyntaxContext::new(None, ContextType::Normal)),
        )
    }

    fn build_dynamic_exp(&mut self, exp: &Exp<'a>) -> Result<Exp<'a>, PineInputError> {
        // First parse the target expression
        self.name_rel_parser.parse_exp(exp);

        let (names, mut def) = self.name_rel_parser.gen_dep_stmts_for_exp(exp);
        let mut sub_ctx = Box::new(SyntaxContext::new(
            unsafe { Some(NonNull::new_unchecked(self.context)) },
            ContextType::FuncDefBlock,
        ));
        // Run this new function definition by simulating the function call.
        names.iter().for_each(|n| {
            let ctx = downcast_ctx(self.context);
            sub_ctx.declare_var(n, ctx.get_var(n).unwrap().clone())
        });
        let varids: Vec<_> = names.iter().map(|n| sub_ctx.gen_var_index(n)).collect();
        def.varids = Some(varids);

        self.context = &mut *sub_ctx;
        self.parse_blk_only(&mut def.body)?;
        self.context = sub_ctx.parent.unwrap().as_ptr();

        // Insert function definition
        let def_ctxid = self._root_ctx.gen_var_index_only();
        let var_exp = Exp::VarName(RVVarName::new_with_index(
            "@gen",
            VarIndex::new(def_ctxid, 0),
        ));

        def.name_varid = def_ctxid;
        self.func_defs.push(def);
        Ok(var_exp)
    }

    fn parse_std_func_call(
        &mut self,
        func_call: &mut FunctionCall<'a>,
        fun_type: &Rc<FunctionTypes<'a>>,
    ) -> ParseResult<'a> {
        let mut pos_arg_type = vec![];
        for arg in func_call.pos_args.iter_mut() {
            pos_arg_type.push(self.parse_exp(arg)?);
        }
        let mut dict_arg_type = vec![];
        for (name, exp) in func_call.dict_args.iter_mut() {
            dict_arg_type.push((name.clone(), self.parse_exp(exp)?));
        }

        let res_fun = fun_type.0.iter().find(|func| {
            let (args, _) = &func.signature;
            if args.len() >= pos_arg_type.len() {
                let pos_match = pos_arg_type.iter().zip(args.iter()).all(|(x1, x2)| {
                    x1.syntax_type == x2.1 || implicity_convert(&x1.syntax_type, &x2.1)
                });
                let dict_match = dict_arg_type.iter().all(|(name, t)| {
                    match args.iter().find(|s| s.0 == name.value) {
                        None => false,
                        Some(val) => {
                            t.syntax_type == val.1 || implicity_convert(&t.syntax_type, &val.1)
                        }
                    }
                });
                pos_match && dict_match
            } else {
                false
            }
        });

        func_call.ctxid = downcast_ctx(self.context).gen_lib_func_index();
        match res_fun {
            None => Err(PineInputError::new(
                PineErrorKind::FuncCallSignatureNotMatch,
                func_call.range,
            )),
            Some(d) => {
                for i in 0..pos_arg_type.len() {
                    if let Some(&SyntaxType::DynamicExpr(_)) = d.get_type(i) {
                        let new_exp = self.build_dynamic_exp(&func_call.pos_args[i])?;
                        // Replace the original expression with the new generated function definition variable name.
                        func_call.pos_args[i] = new_exp;
                    }
                }
                for (i, (name, _)) in dict_arg_type.into_iter().enumerate() {
                    if let Some(&SyntaxType::DynamicExpr(_)) = d.get_type_by_name(&name.value) {
                        let new_exp = self.build_dynamic_exp(&func_call.pos_args[i])?;
                        // Replace the original expression with the new generated function definition variable name.
                        func_call.dict_args[i] = (name, new_exp);
                    }
                }
                func_call.func_type = Some(d.clone());
                Ok(ParseValue::new_with_type((d.signature).1.clone()))
            }
        }
    }

    fn gen_new_func_def(
        &mut self,
        func_call: &mut FunctionCall<'a>,
        names: &Vec<&'a str>,
        arg_types: Vec<SyntaxType<'a>>,
        method_name: &'a str,
        func_name: String,
    ) -> ParseResult<'a> {
        let mut sub_ctx = Box::new(SyntaxContext::new(
            unsafe { Some(NonNull::new_unchecked(self.context)) },
            ContextType::FuncDefBlock,
        ));
        names
            .iter()
            .zip(arg_types.iter())
            .for_each(|(&n, t)| sub_ctx.declare_var(n, t.clone()));

        let func_def;
        unsafe {
            func_def = self.user_funcs[method_name].as_mut().unwrap();
        };
        // Create specific function definition from the origin function type.
        // The specific function definition contain the type information.
        let mut spec_def = func_def.gen_spec_def();

        let varids: Vec<_> = names.iter().map(|n| sub_ctx.gen_var_index(n)).collect();
        spec_def.varids = Some(varids);

        self.context = &mut *sub_ctx;

        self.name_rel_parser
            .enter_ctx(self.context, func_call.ctxid);

        let parse_res = self.parse_blk(&mut spec_def.body)?;
        // Push the specific function definition to spec_defs.
        func_def.spec_defs.as_mut().unwrap().push(spec_def);
        let spec_index = func_def.spec_defs.as_ref().unwrap().len() as i32 - 1;

        self.name_rel_parser.exit_ctx();
        self.context = sub_ctx.parent.unwrap().as_ptr();

        let parent_context = downcast_ctx(self.context);
        parent_context.subctxs.push(sub_ctx);
        parent_context.declare_user_func(
            func_name,
            SyntaxType::UserFunction(Rc::new((names.clone(), parse_res.syntax_type.clone()))),
            spec_index,
        );
        func_call.spec_index = spec_index;
        Ok(ParseValue::new_with_type(parse_res.syntax_type))
    }

    fn parse_user_func_call(
        &mut self,
        func_call: &mut FunctionCall<'a>,
        names: &Vec<&'a str>,
        method_name: &'a str,
    ) -> ParseResult<'a> {
        if func_call.dict_args.len() > 0 {
            Err(PineInputError::new(
                PineErrorKind::ForbiddenDictArgsForUserFunc,
                func_call.range,
            ))
        } else if func_call.pos_args.len() != names.len() {
            Err(PineInputError::new(
                PineErrorKind::FuncCallSignatureNotMatch,
                func_call.range,
            ))
        } else {
            let mut pos_arg_type = vec![];
            for arg in func_call.pos_args.iter_mut() {
                pos_arg_type.push(self.parse_exp(arg)?.syntax_type);
            }
            let typeid = self.types_id_gen.get(&pos_arg_type);
            // Generate name by the argument types.
            let fun_name = format!("{}@{}", method_name, typeid);

            let context = downcast_ctx(self.context);
            func_call.ctxid = context.gen_child_ctx_index();

            match context.get_user_func(&fun_name) {
                Some((SyntaxType::UserFunction(func_type), index)) => {
                    func_call.spec_index = *index;
                    Ok(ParseValue::new_with_type(func_type.1.clone()))
                }
                None => {
                    self.gen_new_func_def(func_call, names, pos_arg_type, method_name, fun_name)
                }
                Some(_) => unreachable!(),
            }
        }
    }

    fn parse_func_call(&mut self, func_call: &mut FunctionCall<'a>) -> ParseResult<'a> {
        let method_type = self.parse_exp(&mut func_call.method)?;
        match method_type.syntax_type {
            SyntaxType::Function(fun_type) => self.parse_std_func_call(func_call, &fun_type),
            SyntaxType::ObjectFunction(_, fun_type) => {
                self.parse_std_func_call(func_call, &fun_type)
            }
            SyntaxType::ValFunction(_, fun_type) => self.parse_std_func_call(func_call, &fun_type),
            SyntaxType::ValObjectFunction(_, _, fun_type) => {
                self.parse_std_func_call(func_call, &fun_type)
            }
            SyntaxType::UserFunction(names) => {
                self.parse_user_func_call(func_call, &names.0, method_type.varname.unwrap())
            }
            _ => {
                self.catch(PineInputError::new(
                    PineErrorKind::VarNotCallable,
                    func_call.range,
                ));
                Ok(ParseValue::new_with_type(SyntaxType::Any))
            }
        }
    }

    fn parse_tuple(&mut self, tuple: &mut TupleNode<'a>) -> ParseResult<'a> {
        let mut tuple_type: Vec<SyntaxType<'a>> = vec![];
        for arg in tuple.exps.iter_mut() {
            tuple_type.push(self.parse_exp(arg)?.syntax_type);
        }
        Ok(ParseValue::new_with_type(SyntaxType::Tuple(Rc::new(
            tuple_type,
        ))))
    }

    fn parse_ref_call_arg(
        &mut self,
        ref_call: &mut RefCall<'a>,
        name_type: SyntaxType<'a>,
    ) -> ParseResult<'a> {
        let arg_res = self.parse_exp(&mut ref_call.arg)?;
        match arg_res.syntax_type.get_v_for_vf() {
            &SyntaxType::Simple(SimpleSyntaxType::Int)
            | &SyntaxType::Series(SimpleSyntaxType::Int) => {
                Ok(ParseValue::new_with_type(name_type))
            }
            _ => {
                self.catch(PineInputError::new(
                    PineErrorKind::RefIndexNotInt,
                    ref_call.range,
                ));
                Ok(ParseValue::new_with_type(name_type))
            }
        }
    }

    fn parse_ref_call(&mut self, ref_call: &mut RefCall<'a>) -> ParseResult<'a> {
        let name_res = self.parse_exp(&mut ref_call.name)?;

        match name_res.syntax_type.clone().into_v_for_vf() {
            // If the type is simple, then we need to update the var to series.
            SyntaxType::Simple(t) => {
                if t != SimpleSyntaxType::Na {
                    if let Some(name) = name_res.varname {
                        let series_type = SyntaxType::Series(t);
                        downcast_ctx(self.context).update_var(name, series_type.clone());
                        self.parse_ref_call_arg(ref_call, series_type)
                    } else {
                        Err(PineInputError::new(
                            PineErrorKind::VarNotSeriesInRef,
                            ref_call.range,
                        ))
                    }
                } else {
                    Err(PineInputError::new(
                        PineErrorKind::VarNotSeriesInRef,
                        ref_call.range,
                    ))
                }
            }
            SyntaxType::Series(_) => self.parse_ref_call_arg(ref_call, name_res.syntax_type),
            SyntaxType::ObjectClass(class) => {
                Ok(ParseValue::new_with_type(SyntaxType::ObjectClass(class)))
            }
            _ => Err(PineInputError::new(
                PineErrorKind::VarNotSeriesInRef,
                ref_call.range,
            )),
        }
    }

    fn get_for_obj(
        obj: &SyntaxType<'a>,
        keys: &[&'a str],
    ) -> Result<SyntaxType<'a>, PineErrorKind> {
        let map = match obj {
            SyntaxType::Object(map) => map,
            SyntaxType::ObjectFunction(map, _) => map,
            SyntaxType::ValObjectFunction(_, map, _) => map,
            _ => {
                return Err(PineErrorKind::RefObjTypeNotObj);
            }
        };
        match map.get(&keys[0]) {
            None => Err(PineErrorKind::RefKeyNotExist),
            Some(dest_type) => {
                if keys.len() > 1 {
                    Self::get_for_obj(&dest_type, &keys[1..])
                } else {
                    Ok(dest_type.clone())
                }
            }
        }
    }

    fn parse_prefix(&mut self, prefix: &mut PrefixExp<'a>) -> ParseResult<'a> {
        let left_res = self.parse_exp(&mut prefix.left_exp)?;
        let name = prefix.right_name;
        match Self::get_for_obj(&left_res.syntax_type, &[name.value]) {
            Ok(val_type) => Ok(ParseValue::new_with_type(val_type)),
            Err(code) => Err(PineInputError::new(code, prefix.range)),
        }
    }

    pub fn parse_condition(&mut self, condition: &mut Condition<'a>) -> ParseResult<'a> {
        let cond_res = self.parse_exp(&mut condition.cond)?;
        if implicity_convert(
            &cond_res.syntax_type,
            &SyntaxType::Series(SimpleSyntaxType::Bool),
        ) {
            let exp1_res = self.parse_exp(&mut condition.exp1)?;
            let exp2_res = self.parse_exp(&mut condition.exp2)?;
            if let Some(v_type) = common_type(&exp1_res.syntax_type, &exp2_res.syntax_type) {
                let res_type = simple_to_series(v_type);
                condition.result_type = res_type.clone();
                Ok(ParseValue::new_with_type(res_type))
            } else {
                Err(PineInputError::new(
                    PineErrorKind::CondExpTypesNotSame,
                    condition.range,
                ))
            }
        } else {
            Err(PineInputError::new(
                PineErrorKind::CondNotBool,
                condition.range,
            ))
        }
    }

    fn parse_ifthenelse_cond(&mut self, ite: &mut IfThenElse<'a>) -> Result<(), PineInputError> {
        let cond_res = self.parse_exp(&mut ite.cond)?;
        if !implicity_convert(
            &cond_res.syntax_type,
            &SyntaxType::Series(SimpleSyntaxType::Bool),
        ) {
            return Err(PineInputError::new(PineErrorKind::CondNotBool, ite.range));
        }
        Ok(())
    }

    fn parse_ifthenelse_then(&mut self, ite: &mut IfThenElse<'a>) -> ParseResult<'a> {
        ite.then_ctxid = downcast_ctx(self.context).gen_child_ctx_index();
        // Create new context for if block
        let mut if_ctx = Box::new(SyntaxContext::new(
            Some(NonNull::new(self.context).unwrap()),
            ContextType::IfElseBlock,
        ));
        self.context = &mut *if_ctx;
        self.name_rel_parser.enter_ctx(self.context, ite.then_ctxid);

        let then_res = self.parse_blk(&mut ite.then_blk)?;

        self.name_rel_parser.exit_ctx();
        self.context = if_ctx.parent.unwrap().as_ptr();

        downcast_ctx(self.context).subctxs.push(if_ctx);
        Ok(then_res)
    }

    fn parse_ifthenelse_else(
        &mut self,
        else_blk: &mut Block<'a>,
        ctxid: &mut i32,
    ) -> ParseResult<'a> {
        *ctxid = downcast_ctx(self.context).gen_child_ctx_index();
        // Create new context for if block
        let mut else_ctx = Box::new(SyntaxContext::new(
            unsafe { Some(NonNull::new_unchecked(self.context)) },
            ContextType::IfElseBlock,
        ));
        self.context = &mut *else_ctx;
        self.name_rel_parser.enter_ctx(self.context, *ctxid);

        let else_res = self.parse_blk(else_blk)?;

        self.name_rel_parser.exit_ctx();
        self.context = else_ctx.parent.unwrap().as_ptr();

        downcast_ctx(self.context).subctxs.push(else_ctx);
        Ok(else_res)
    }

    pub fn parse_ifthenelse_exp(&mut self, ite: &mut IfThenElse<'a>) -> ParseResult<'a> {
        self.parse_ifthenelse_cond(ite)?;
        let then_res = self.parse_ifthenelse_then(ite)?;

        if then_res.syntax_type.is_void() {
            return Err(PineInputError::new(
                PineErrorKind::ExpNoReturn,
                ite.then_blk.range,
            ));
        }
        if then_res.syntax_type.is_na() {
            return Err(PineInputError::new(
                PineErrorKind::ExpReturnNa,
                ite.then_blk.range,
            ));
        }
        if let Some(else_blk) = &mut ite.else_blk {
            // Create new context for if block
            let else_res = self.parse_ifthenelse_else(else_blk, &mut ite.else_ctxid)?;

            if else_res.syntax_type.is_void() {
                return Err(PineInputError::new(
                    PineErrorKind::ExpNoReturn,
                    else_blk.range,
                ));
            }
            if else_res.syntax_type.is_na() {
                return Err(PineInputError::new(
                    PineErrorKind::ExpReturnNa,
                    else_blk.range,
                ));
            }
            // Find the common type that can satisfy the then type and else type
            if let Some(v_type) = common_type(&then_res.syntax_type, &else_res.syntax_type) {
                // The return type of if-then-else block must be series.
                let res_type = simple_to_series(v_type);
                ite.result_type = res_type.clone();
                return Ok(ParseValue::new_with_type(res_type));
            } else {
                return Err(PineInputError::new(PineErrorKind::TypeMismatch, ite.range));
            }
        } else {
            let res_type = simple_to_series(then_res.syntax_type);
            ite.result_type = res_type.clone();
            Ok(ParseValue::new_with_type(res_type))
        }
    }

    fn parse_ifthenelse_stmt(&mut self, ite: &mut IfThenElse<'a>) -> ParseResult<'a> {
        self.parse_ifthenelse_cond(ite)?;
        self.parse_ifthenelse_then(ite)?;
        if let Some(else_blk) = &mut ite.else_blk {
            self.parse_ifthenelse_else(else_blk, &mut ite.else_ctxid)?;
        }
        ite.result_type = SyntaxType::Void;
        Ok(ParseValue::new_with_type(SyntaxType::Void))
    }

    fn parse_forrange(&mut self, for_range: &mut ForRange<'a>) -> ParseResult<'a> {
        let start_res = self.parse_exp(&mut for_range.start)?;
        if !start_res.syntax_type.is_int() {
            self.catch(PineInputError::new(
                PineErrorKind::ForRangeIndexNotInt,
                for_range.start.range(),
            ));
        }
        let end_res = self.parse_exp(&mut for_range.end)?;
        if !end_res.syntax_type.is_int() {
            self.catch(PineInputError::new(
                PineErrorKind::ForRangeIndexNotInt,
                for_range.end.range(),
            ));
        }
        if let Some(step) = &mut for_range.step {
            let step_res = self.parse_exp(step)?;
            if !step_res.syntax_type.is_int() {
                self.catch(PineInputError::new(
                    PineErrorKind::ForRangeIndexNotInt,
                    step.range(),
                ));
            }
        }
        for_range.ctxid = downcast_ctx(self.context).gen_child_ctx_index();

        let mut for_ctx = Box::new(SyntaxContext::new(
            unsafe { Some(NonNull::new_unchecked(self.context)) },
            ContextType::ForRangeBlock,
        ));
        for_ctx.declare_var(
            for_range.var.value,
            SyntaxType::Simple(SimpleSyntaxType::Int),
        );
        self.context = &mut *for_ctx;
        for_range.varid = for_ctx.gen_var_index(for_range.var.value);
        self.name_rel_parser
            .enter_ctx(self.context, for_range.ctxid);

        let blk_res = self.parse_blk(&mut for_range.do_blk)?;

        self.name_rel_parser.exit_ctx();
        self.context = for_ctx.parent.unwrap().as_ptr();

        downcast_ctx(self.context).subctxs.push(for_ctx);

        Ok(blk_res)
    }

    fn parse_forrange_stmt(&mut self, for_range: &mut ForRange<'a>) -> ParseResult<'a> {
        self.parse_forrange(for_range)?;
        for_range.result_type = SyntaxType::Void;
        Ok(ParseValue::new_with_type(SyntaxType::Void))
    }

    pub fn parse_forrange_exp(&mut self, for_range: &mut ForRange<'a>) -> ParseResult<'a> {
        let blk_res = self.parse_forrange(for_range)?;
        if blk_res.syntax_type.is_void() {
            return Err(PineInputError::new(
                PineErrorKind::ExpNoReturn,
                for_range.do_blk.range,
            ));
        }
        if blk_res.syntax_type.is_na() {
            return Err(PineInputError::new(
                PineErrorKind::ExpReturnNa,
                for_range.do_blk.range,
            ));
        }
        let res_type = simple_to_series(blk_res.syntax_type);
        for_range.result_type = res_type.clone();
        Ok(ParseValue::new_with_type(res_type))
    }

    fn parse_varname(&mut self, varname: &mut RVVarName<'a>) -> ParseResult<'a> {
        let name = varname.name.value;
        let ctx_ins = downcast_ctx(self.context);
        // If the input names are found, we should record this name.
        if ctx_ins.is_client_input(name) {
            ctx_ins.declare_inputnames(name);
        }
        match downcast_ctx(self.context).get_var(name) {
            None => {
                self.catch(PineInputError::new(
                    PineErrorKind::VarNotDeclare,
                    varname.name.range,
                ));
                Ok(ParseValue::new(
                    SyntaxType::Simple(SimpleSyntaxType::Na),
                    name,
                ))
            }
            Some(val) => {
                varname.var_index = downcast_ctx(self.context).get_var_index(name);
                match val {
                    &SyntaxType::Val(_)
                    | &SyntaxType::ValFunction(_, _)
                    | &SyntaxType::ValObjectFunction(_, _, _) => {
                        // If the var is evaluate var, we need generate one index to cache.
                        varname.eval_id = downcast_ctx(self.context).gen_lib_func_index();
                    }
                    _ => {}
                }
                Ok(ParseValue::new(val.clone(), name))
            }
        }
    }

    fn parse_type_cast(&mut self, type_cast: &mut TypeCast<'a>) -> ParseResult<'a> {
        let origin_type = self.parse_exp(&mut type_cast.exp)?.syntax_type;
        let (is_cast_err, result) = explicity_type_cast(&origin_type, &type_cast.data_type);
        match result {
            SyntaxType::ObjectClass(obj_cls) => {
                type_cast.cast_index = downcast_ctx(self.context).get_var_index(obj_cls);
                type_cast.func_index = downcast_ctx(self.context).gen_lib_func_index();
            }
            _ => {}
        }
        if is_cast_err {
            self.catch(PineInputError::new(
                PineErrorKind::InvalidTypeCast {
                    origin: SimpleSyntaxType::from(origin_type),
                    cast: SimpleSyntaxType::from(type_cast.data_type.clone()),
                },
                type_cast.range,
            ));
        }
        Ok(ParseValue::new_with_type(result))
    }

    fn parse_unary(&mut self, unary: &mut UnaryExp<'a>) -> ParseResult<'a> {
        let exp_type = self.parse_exp(&mut unary.exp)?.syntax_type;
        match unary.op {
            UnaryOp::Plus | UnaryOp::Minus => {
                if !exp_type.is_num() {
                    Err(PineInputError::new(
                        PineErrorKind::UnaryTypeNotNum,
                        unary.range,
                    ))
                } else {
                    Ok(ParseValue::new_with_type(exp_type))
                }
            }
            UnaryOp::BoolNot => {
                if implicity_convert(&exp_type, &SyntaxType::Series(SimpleSyntaxType::Bool)) {
                    match exp_type.into_v_for_vf() {
                        SyntaxType::Simple(_) => Ok(ParseValue::new_with_type(SyntaxType::Simple(
                            SimpleSyntaxType::Bool,
                        ))),
                        SyntaxType::Series(_) => Ok(ParseValue::new_with_type(SyntaxType::Series(
                            SimpleSyntaxType::Bool,
                        ))),
                        _ => unreachable!(),
                    }
                } else {
                    Err(PineInputError::new(
                        PineErrorKind::UnaryTypeNotNum,
                        unary.range,
                    ))
                }
            }
        }
    }

    pub fn parse_binary(&mut self, binary: &mut BinaryExp<'a>) -> ParseResult<'a> {
        let exp1_type = self.parse_exp(&mut binary.exp1)?.syntax_type;
        let exp2_type = self.parse_exp(&mut binary.exp2)?.syntax_type;
        let gen_bool =
            |binary: &mut BinaryExp<'a>, exp1_type: SyntaxType<'a>, exp2_type: SyntaxType<'a>| {
                let result = match (exp1_type.into_v_for_vf(), exp2_type.into_v_for_vf()) {
                    (SyntaxType::Series(_), _) | (_, SyntaxType::Series(_)) => {
                        SyntaxType::Series(SimpleSyntaxType::Bool)
                    }
                    _ => SyntaxType::Simple(SimpleSyntaxType::Bool),
                };
                binary.result_type = result.clone();
                Ok(ParseValue::new_with_type(result))
            };
        match binary.op {
            BinaryOp::Plus | BinaryOp::Minus | BinaryOp::Mul | BinaryOp::Div | BinaryOp::Mod => {
                // plus for string concatenate
                if binary.op == BinaryOp::Plus && exp1_type.is_string() && exp2_type.is_string() {
                    let result_type = match (exp1_type.into_v_for_vf(), exp2_type.into_v_for_vf()) {
                        (SyntaxType::Series(_), _) | (_, SyntaxType::Series(_)) => {
                            SyntaxType::Series(SimpleSyntaxType::String)
                        }
                        _ => SyntaxType::Simple(SimpleSyntaxType::String),
                    };
                    binary.result_type = result_type.clone();
                    binary.ref_type = result_type.clone();
                    return Ok(ParseValue::new_with_type(result_type));
                }
                // The operator must be num type int, float, series(int), series(float)
                if !(exp1_type.is_num() && exp2_type.is_num()) {
                    return Err(PineInputError::new(
                        PineErrorKind::BinaryTypeNotNum,
                        binary.range,
                    ));
                }
                if let Some(result_type) = similar_type(&exp1_type, &exp2_type) {
                    binary.result_type = result_type.clone();
                    binary.ref_type = result_type.clone();
                    Ok(ParseValue::new_with_type(result_type))
                } else {
                    Err(PineInputError::new(
                        PineErrorKind::TypeMismatch,
                        binary.range,
                    ))
                }
            }
            BinaryOp::Gt | BinaryOp::Geq | BinaryOp::Lt | BinaryOp::Leq => {
                if !(exp1_type.is_num() && exp2_type.is_num()) {
                    self.catch(PineInputError::new(
                        PineErrorKind::BinaryTypeNotNum,
                        binary.range,
                    ));
                }
                if let Some(result_type) = similar_type(&exp1_type, &exp2_type) {
                    binary.ref_type = result_type.clone();
                    gen_bool(binary, exp1_type, exp2_type)
                } else {
                    Err(PineInputError::new(
                        PineErrorKind::TypeMismatch,
                        binary.range,
                    ))
                }
            }
            BinaryOp::Eq | BinaryOp::Neq => {
                if let Some(_type) = similar_type(&exp2_type, &exp1_type) {
                    binary.ref_type = _type;
                    gen_bool(binary, exp1_type, exp2_type)
                } else {
                    self.catch(PineInputError::new(
                        PineErrorKind::TypeMismatch,
                        binary.range,
                    ));
                    gen_bool(binary, exp1_type, exp2_type)
                }
            }
            BinaryOp::BoolAnd | BinaryOp::BoolOr => {
                let bool_type = SyntaxType::Series(SimpleSyntaxType::Bool);
                if implicity_convert(&exp1_type, &bool_type)
                    && implicity_convert(&exp2_type, &bool_type)
                {
                    gen_bool(binary, exp1_type, exp2_type)
                } else {
                    self.catch(PineInputError::new(
                        PineErrorKind::BoolExpTypeNotBool,
                        binary.range,
                    ));
                    gen_bool(binary, exp1_type, exp2_type)
                }
            }
        }
    }

    fn parse_one_assign(
        &mut self,
        assign: &mut Assignment<'a>,
        name: &VarName<'a>,
        val: SyntaxType<'a>,
    ) -> Result<SyntaxType<'a>, PineInputError> {
        let context = downcast_ctx(self.context);
        // if context.get_var_scope(name.value).is_some() {
        //     self.catch(PineInputError::new(
        //         PineErrorKind::VarHasDeclare,
        //         name.range,
        //     ));
        // }
        if let Some(data_type) = &assign.var_type {
            let (is_cast_err, result) = implicity_type_cast(&val, &data_type);
            if is_cast_err {
                self.catch(PineInputError::new(
                    PineErrorKind::InvalidTypeCast {
                        origin: SimpleSyntaxType::from(val),
                        cast: SimpleSyntaxType::from(data_type.clone()),
                    },
                    assign.range,
                ));
            }
            if name.value != "_" {
                context.declare_var(name.value, result.clone());
            }

            // If the assign need type cast, then we need record the cast index
            match result {
                SyntaxType::ObjectClass(obj_cls) => {
                    assign.cast_index = downcast_ctx(self.context).get_var_index(obj_cls);
                    assign.cast_func_index = downcast_ctx(self.context).gen_lib_func_index();
                }
                _ => {}
            }
            Ok(result)
        } else {
            if name.value != "_" {
                context.declare_var(name.value, val.clone());
            }
            Ok(val)
        }
    }

    fn parse_assign(&mut self, assign: &mut Assignment<'a>) -> ParseResult<'a> {
        let val_res = self.parse_exp(&mut assign.val)?;
        if assign.names.len() > 1 {
            if let SyntaxType::Tuple(tuple) = val_res.syntax_type {
                if tuple.len() != assign.names.len() {
                    Err(PineInputError::new(
                        PineErrorKind::TupleNotMatch,
                        assign.range,
                    ))
                } else {
                    let mut ret_tuple = vec![];
                    let names = mem::replace(&mut assign.names, vec![]);
                    for (name, val_type) in names.iter().zip(tuple.iter()) {
                        ret_tuple.push(self.parse_one_assign(assign, name, val_type.clone())?);
                    }
                    mem::replace(&mut assign.names, names);
                    let context = downcast_ctx(self.context);
                    let varids = assign
                        .names
                        .iter()
                        .map(|n| {
                            if n.value == "_" {
                                -1
                            } else if context.contain_var_index_scope(n.value) {
                                self.catch(PineInputError::new(
                                    PineErrorKind::VarHasDeclare,
                                    n.range,
                                ));
                                context.get_var_index(n.value).varid
                            } else {
                                context.gen_var_index(n.value)
                            }
                        })
                        .collect();
                    assign.varids = Some(varids);
                    Ok(ParseValue::new_with_type(SyntaxType::Tuple(Rc::new(
                        ret_tuple,
                    ))))
                }
            } else {
                Err(PineInputError::new(
                    PineErrorKind::BinaryTypeNotNum,
                    assign.range,
                ))
            }
        } else {
            let names = mem::replace(&mut assign.names, vec![]);
            let rtype = self.parse_one_assign(assign, &names[0], val_res.syntax_type)?;
            mem::replace(&mut assign.names, names);
            let context = downcast_ctx(self.context);

            let name = assign.names[0];
            if name.value == "_" {
                assign.varids = Some(vec![-1]);
            } else if context.contain_var_index_scope(name.value) {
                self.catch(PineInputError::new(
                    PineErrorKind::VarHasDeclare,
                    name.range,
                ));
                assign.varids = Some(vec![context.get_var_index(assign.names[0].value).varid]);
            } else {
                assign.varids = Some(vec![context.gen_var_index(assign.names[0].value)]);
            }
            Ok(ParseValue::new_with_type(rtype))
        }
    }

    fn parse_var_assign(&mut self, assign: &mut VarAssignment<'a>) -> ParseResult<'a> {
        let context = downcast_ctx(self.context);
        match context.get_var(assign.name.value) {
            None => Err(PineInputError::new(
                PineErrorKind::VarNotDeclare,
                assign.range,
            )),
            Some(cur_type) => {
                assign.var_index = downcast_ctx(self.context).get_var_index(assign.name.value);
                let last_type = simple_to_series(cur_type.clone());
                let val_res = self.parse_exp(&mut assign.val)?;
                context.update_var(assign.name.value, last_type.clone());
                if implicity_convert(&val_res.syntax_type, &last_type) {
                    Ok(ParseValue::new_with_type(last_type))
                } else {
                    self.catch(PineInputError::new(
                        PineErrorKind::InvalidTypeCast {
                            origin: SimpleSyntaxType::from(val_res.syntax_type),
                            cast: SimpleSyntaxType::from(last_type.clone()),
                        },
                        assign.range,
                    ));
                    Ok(ParseValue::new_with_type(last_type))
                }
            }
        }
    }

    fn parse_func_def(&mut self, func_def: &mut FunctionDef<'a>) -> ParseResult<'a> {
        let context = downcast_ctx(self.context);
        let name = func_def.name.value;
        self.user_funcs.insert(name, func_def);
        let param_names: Vec<_> = func_def.params.iter().map(|v| v.value).collect();
        let name_type = SyntaxType::UserFunction(Rc::new((param_names, SyntaxType::Any)));
        if context.contain_var_index_scope(name) {
            self.catch(PineInputError::new(
                PineErrorKind::VarHasDeclare,
                func_def.name.range,
            ));
            func_def.name_varid = context.get_var_index(name).varid;
        } else {
            func_def.name_varid = context.gen_var_index(name);
        }
        context.declare_var(name, name_type.clone());
        Ok(ParseValue::new_with_type(name_type))
    }

    fn parse_exp(&mut self, exp: &mut Exp<'a>) -> ParseResult<'a> {
        match exp {
            Exp::Na(_) => Ok(ParseValue::new_with_type(SyntaxType::Simple(
                SimpleSyntaxType::Na,
            ))),
            Exp::Bool(_) => Ok(ParseValue::new_with_type(SyntaxType::Simple(
                SimpleSyntaxType::Bool,
            ))),
            Exp::Num(Numeral::Int(_)) => Ok(ParseValue::new_with_type(SyntaxType::Simple(
                SimpleSyntaxType::Int,
            ))),
            Exp::Num(Numeral::Float(_)) => Ok(ParseValue::new_with_type(SyntaxType::Simple(
                SimpleSyntaxType::Float,
            ))),
            Exp::Str(_) => Ok(ParseValue::new_with_type(SyntaxType::Simple(
                SimpleSyntaxType::String,
            ))),
            Exp::Color(_) => Ok(ParseValue::new_with_type(SyntaxType::Simple(
                SimpleSyntaxType::Color,
            ))),
            Exp::VarName(name) => self.parse_varname(name),
            Exp::Tuple(tuple) => self.parse_tuple(tuple),
            Exp::TypeCast(type_cast) => self.parse_type_cast(type_cast),
            Exp::FuncCall(func_call) => self.parse_func_call(func_call),
            Exp::RefCall(ref_call) => self.parse_ref_call(ref_call),
            Exp::PrefixExp(prefix) => self.parse_prefix(prefix),
            Exp::Condition(condition) => self.parse_condition(condition),
            Exp::Ite(ite) => self.parse_ifthenelse_exp(ite),
            Exp::ForRange(fr) => self.parse_forrange_exp(fr),
            Exp::Assignment(assign) => self.parse_assign(assign),
            Exp::VarAssignment(var_assign) => self.parse_var_assign(var_assign),
            Exp::UnaryExp(node) => self.parse_unary(node),
            Exp::BinaryExp(node) => self.parse_binary(node),
        }
    }

    fn parse_interrupt(&mut self, range: &StrRange, code: PineErrorKind) -> ParseResult<'a> {
        let context = downcast_ctx(self.context);
        if context.get_type() != ContextType::ForRangeBlock {
            self.catch(PineInputError::new(code, *range));
        }
        Ok(ParseValue::new_with_type(SyntaxType::Void))
    }

    fn parse_break(&mut self, range: &StrRange) -> ParseResult<'a> {
        self.parse_interrupt(range, PineErrorKind::BreakNotInForStmt)
    }

    fn parse_continue(&mut self, range: &StrRange) -> ParseResult<'a> {
        self.parse_interrupt(range, PineErrorKind::ContinueNotInForStmt)
    }

    fn parse_stmt(&mut self, stmt: &mut Statement<'a>) -> ParseResult<'a> {
        match stmt {
            Statement::Break(node) => self.parse_break(node),
            Statement::Continue(node) => self.parse_continue(node),
            Statement::FuncCall(func_call) => self.parse_func_call(func_call),
            Statement::Ite(ite) => self.parse_ifthenelse_stmt(ite),
            Statement::ForRange(fr) => self.parse_forrange_stmt(fr),
            Statement::Assignment(assign) => self.parse_assign(assign),
            Statement::VarAssignment(assign) => self.parse_var_assign(assign),
            Statement::FuncDef(func_def) => self.parse_func_def(func_def),
            Statement::None(_) => Ok(ParseValue::new_with_type(SyntaxType::Void)),
            Statement::Exp(exp) => self.parse_exp(exp),
        }
    }

    pub fn parse_blk_only(&mut self, blk: &mut Block<'a>) -> ParseResult<'a> {
        for stmt in blk.stmts.iter_mut() {
            self.parse_stmt(stmt)?;
        }
        let result = if let Some(ref mut exp) = blk.ret_stmt {
            self.parse_exp(exp)
        } else {
            Ok(ParseValue::new_with_type(SyntaxType::Void))
        };
        let context = downcast_ctx(self.context);
        blk.var_count = context.max_var_index + 1;
        blk.subctx_count = context.max_child_ctx_index + 1;
        blk.libfun_count = context.max_lib_func_index + 1;
        result
    }

    pub fn parse_blk(&mut self, blk: &mut Block<'a>) -> ParseResult<'a> {
        for stmt in blk.stmts.iter_mut() {
            self.parse_stmt(stmt)?;
            self.name_rel_parser.parse_stmt(stmt);
        }
        let result = if let Some(ref mut exp) = blk.ret_stmt {
            let res = self.parse_exp(exp);
            self.name_rel_parser.parse_exp(exp);
            res
        } else {
            Ok(ParseValue::new_with_type(SyntaxType::Void))
        };
        let context = downcast_ctx(self.context);
        blk.var_count = context.max_var_index + 1;
        blk.subctx_count = context.max_child_ctx_index + 1;
        blk.libfun_count = context.max_lib_func_index + 1;

        // If top context, insert all the function definition to the header.
        if self.context == &mut *self._root_ctx {
            let defs = mem::replace(&mut self.func_defs, vec![]);
            let new_stmts: Vec<_> = defs
                .into_iter()
                .map(|d| Statement::FuncDef(Box::new(d)))
                .collect();
            blk.stmts = [new_stmts, mem::replace(&mut blk.stmts, vec![])].concat();
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::color::ColorNode;
    use crate::ast::input::{Input, Position, StrRange};
    use crate::ast::name::VarName;
    use crate::ast::op::*;
    use crate::ast::stat_expr_types::{BoolNode, DataType, NaNode, RefCall, TypeCast};
    use crate::ast::string::StringNode;
    use crate::ast::syntax_type::FunctionType;

    fn varname(n: &str) -> VarName {
        VarName::new(n, StrRange::new_empty())
    }

    fn rvarname(n: &str) -> RVVarName {
        RVVarName::new(VarName::new(n, StrRange::new_empty()))
    }

    fn rvarname_index(n: &str, index: VarIndex) -> RVVarName {
        let mut name = RVVarName::new(VarName::new(n, StrRange::new_empty()));
        name.var_index = index;
        name
    }

    const INT_TYPE: SyntaxType = SyntaxType::Simple(SimpleSyntaxType::Int);
    const FLOAT_TYPE: SyntaxType = SyntaxType::Simple(SimpleSyntaxType::Float);

    fn int_exp<'a>(i: i64) -> Exp<'a> {
        Exp::Num(Numeral::from_i64(i))
    }

    fn float_exp<'a>(i: f64) -> Exp<'a> {
        Exp::Num(Numeral::from_f64(i))
    }

    fn fun_nm<'a>() -> Exp<'a> {
        Exp::VarName(rvarname("func"))
    }

    #[test]
    fn func_def_test() {
        let gen_def = || {
            FunctionDef::new(
                varname("funa"),
                vec![varname("hello"), varname("hello2")],
                Block::new(vec![], None, StrRange::new_empty()),
                StrRange::new_empty(),
            )
        };
        let mut parser = SyntaxParser::new();
        let mut func_def = gen_def();
        assert_eq!(
            parser.parse_func_def(&mut func_def),
            Ok(ParseValue::new_with_type(SyntaxType::UserFunction(
                Rc::new((vec!["hello", "hello2"], SyntaxType::Any))
            )))
        );
        assert_eq!(func_def.name_varid, 0);
        assert_eq!(parser.user_funcs["funa"], &mut func_def as *mut FunctionDef);
        assert_eq!(downcast_ctx(parser.context).vars.contains_key("funa"), true);

        let mut parser2 = SyntaxParser::new_with_vars(&vec![("funa", INT_TYPE)]);
        let mut func_def = gen_def();
        assert_eq!(
            parser2.parse_func_def(&mut func_def),
            Ok(ParseValue::new_with_type(SyntaxType::UserFunction(
                Rc::new((vec!["hello", "hello2"], SyntaxType::Any))
            )))
        );
        assert_eq!(
            parser2.errors,
            vec![PineInputError::new(
                PineErrorKind::VarHasDeclare,
                StrRange::new_empty()
            )]
        );

        parser2.errors = vec![];
        assert_eq!(
            parser2.parse_func_def(&mut func_def),
            Ok(ParseValue::new_with_type(SyntaxType::UserFunction(
                Rc::new((vec!["hello", "hello2"], SyntaxType::Any))
            )))
        );
        assert_eq!(
            parser2.errors,
            vec![PineInputError::new(
                PineErrorKind::VarHasDeclare,
                StrRange::new_empty()
            )]
        );
    }

    #[test]
    fn func_call_test() {
        let mut parser = SyntaxParser::new();
        downcast_ctx(parser.context).declare_var_with_index(
            "func",
            SyntaxType::Function(Rc::new(FunctionTypes(vec![
                FunctionType::new((vec![("arg1", INT_TYPE), ("arg2", INT_TYPE)], INT_TYPE)),
                FunctionType::new((vec![("arg1", FLOAT_TYPE), ("arg2", FLOAT_TYPE)], FLOAT_TYPE)),
            ]))),
        );

        let mut func_call = FunctionCall::new(
            fun_nm(),
            vec![int_exp(1), int_exp(2)],
            vec![],
            1,
            StrRange::new_empty(),
        );
        assert_eq!(
            parser.parse_func_call(&mut func_call),
            Ok(ParseValue::new_with_type(INT_TYPE))
        );

        let mut func_call =
            FunctionCall::new(fun_nm(), vec![int_exp(1)], vec![], 1, StrRange::new_empty());
        assert_eq!(
            parser.parse_func_call(&mut func_call),
            Ok(ParseValue::new_with_type(INT_TYPE))
        );

        let mut func_call = FunctionCall::new(
            fun_nm(),
            vec![int_exp(1)],
            vec![(varname("arg2"), int_exp(1))],
            1,
            StrRange::new_empty(),
        );
        assert_eq!(
            parser.parse_func_call(&mut func_call),
            Ok(ParseValue::new_with_type(INT_TYPE))
        );

        let mut func_call = FunctionCall::new(
            fun_nm(),
            vec![int_exp(1)],
            vec![(varname("arg2"), float_exp(1f64))],
            1,
            StrRange::new_empty(),
        );
        assert_eq!(
            parser.parse_func_call(&mut func_call),
            Ok(ParseValue::new_with_type(FLOAT_TYPE))
        );

        let mut func_call = FunctionCall::new(
            fun_nm(),
            vec![float_exp(1f64)],
            vec![(varname("arg2"), int_exp(1))],
            1,
            StrRange::new_empty(),
        );
        assert_eq!(
            parser.parse_func_call(&mut func_call),
            Ok(ParseValue::new_with_type(FLOAT_TYPE))
        );

        let mut func_call = FunctionCall::new(
            fun_nm(),
            vec![float_exp(1f64)],
            vec![(varname("arg3"), int_exp(1))],
            1,
            StrRange::new_empty(),
        );
        assert_eq!(
            parser.parse_func_call(&mut func_call),
            Err(PineInputError::new(
                PineErrorKind::FuncCallSignatureNotMatch,
                StrRange::new_empty()
            ))
        );
    }

    #[test]
    fn user_func_call_test() {
        let mut parser = SyntaxParser::new();
        let mut func_def = FunctionDef::new(
            varname("fun"),
            vec![varname("a1"), varname("a2")],
            Block::new(
                vec![],
                Some(Exp::VarName(rvarname("a2"))),
                StrRange::new_empty(),
            ),
            StrRange::new_empty(),
        );
        assert!(parser.parse_func_def(&mut func_def).is_ok());

        let mut func_call = FunctionCall::new(
            Exp::VarName(rvarname("fun")),
            vec![int_exp(1), float_exp(2f64)],
            vec![],
            1,
            StrRange::new_empty(),
        );
        assert_eq!(
            parser.parse_func_call(&mut func_call),
            Ok(ParseValue::new_with_type(FLOAT_TYPE))
        );
        assert_eq!(downcast_ctx(parser.context).context_type, ContextType::Main);
        assert_eq!(func_call.ctxid, 0);

        let mut parser = SyntaxParser::new();
        let mut func_def = FunctionDef::new(
            varname("fun"),
            vec![varname("a1"), varname("a2")],
            Block::new(
                vec![],
                Some(Exp::BinaryExp(Box::new(BinaryExp::new(
                    BinaryOp::Plus,
                    Exp::VarName(RVVarName::new(varname("a1"))),
                    Exp::VarName(RVVarName::new(varname("a2"))),
                    StrRange::new_empty(),
                )))),
                StrRange::new_empty(),
            ),
            StrRange::new_empty(),
        );
        parser.parse_func_def(&mut func_def).unwrap();

        let mut func_call = FunctionCall::new(
            Exp::VarName(rvarname("fun")),
            vec![
                Exp::Str(StringNode::new(
                    String::from("hello"),
                    StrRange::new_empty(),
                )),
                Exp::Str(StringNode::new(
                    String::from("world"),
                    StrRange::new_empty(),
                )),
            ],
            vec![],
            1,
            StrRange::new_empty(),
        );
        assert_eq!(
            parser.parse_func_call(&mut func_call),
            Ok(ParseValue::new_with_type(SyntaxType::Simple(
                SimpleSyntaxType::String
            )))
        );
        assert_eq!(downcast_ctx(parser.context).subctxs.len(), 1);
        assert_eq!(downcast_ctx(parser.context).max_child_ctx_index, 0);
    }

    #[test]
    fn multi_user_func_call_test() {
        let mut parser = SyntaxParser::new();
        let cond_exp = Condition::new_no_input(
            Exp::VarName(rvarname("a1")),
            Exp::VarName(rvarname_index("a1", VarIndex::new(0, 0))),
            Exp::VarName(rvarname_index("a2", VarIndex::new(1, 0))),
        );
        let mut func_def = FunctionDef::new(
            varname("fun"),
            vec![varname("a1"), varname("a2")],
            Block::new(
                vec![],
                Some(Exp::Condition(Box::new(cond_exp.clone()))),
                StrRange::new_empty(),
            ),
            StrRange::new_empty(),
        );
        assert!(parser.parse_func_def(&mut func_def).is_ok());

        let run_funcall1 = |parser: &mut SyntaxParser, len| {
            let mut func_call = FunctionCall::new(
                Exp::VarName(rvarname("fun")),
                vec![int_exp(1), float_exp(2f64)],
                vec![],
                1,
                StrRange::new_empty(),
            );
            assert_eq!(
                parser.parse_func_call(&mut func_call),
                Ok(ParseValue::new_with_type(SyntaxType::Series(
                    SimpleSyntaxType::Float
                )))
            );
            assert_eq!(func_call.spec_index, 0);
            assert_eq!(
                downcast_ctx(parser.context).get_user_func("fun@1"),
                Some(&(
                    SyntaxType::UserFunction(Rc::new((
                        vec!["a1", "a2"],
                        SyntaxType::Series(SimpleSyntaxType::Float)
                    ))),
                    0
                ))
            );
            assert_eq!(func_def.spec_defs.as_ref().unwrap().len(), len);
            let mut cond_exp1 = cond_exp.clone();
            cond_exp1.result_type = SyntaxType::Series(SimpleSyntaxType::Float);
            let def_cond_exp = func_def
                .spec_defs
                .as_ref()
                .and_then(|v| v.get(0))
                .and_then(|f| f.body.ret_stmt.as_ref())
                .unwrap();
            assert_eq!(def_cond_exp, &Exp::Condition(Box::new(cond_exp1)));
        };

        let run_funcall2 = |parser: &mut SyntaxParser| {
            let mut func_call = FunctionCall::new(
                Exp::VarName(rvarname("fun")),
                vec![int_exp(1), int_exp(2)],
                vec![],
                1,
                StrRange::new_empty(),
            );
            assert_eq!(
                parser.parse_func_call(&mut func_call),
                Ok(ParseValue::new_with_type(SyntaxType::Series(
                    SimpleSyntaxType::Int
                )))
            );
            assert_eq!(func_call.spec_index, 1);
            assert_eq!(
                downcast_ctx(parser.context).get_user_func("fun@2"),
                Some(&(
                    SyntaxType::UserFunction(Rc::new((
                        vec!["a1", "a2"],
                        SyntaxType::Series(SimpleSyntaxType::Int)
                    ))),
                    1
                ))
            );
            assert_eq!(func_def.spec_defs.as_ref().unwrap().len(), 2);
            let mut cond_exp2 = cond_exp.clone();
            cond_exp2.result_type = SyntaxType::Series(SimpleSyntaxType::Int);
            let def_cond_exp = func_def
                .spec_defs
                .as_ref()
                .and_then(|v| v.get(1))
                .and_then(|f| f.body.ret_stmt.as_ref())
                .unwrap();
            assert_eq!(def_cond_exp, &Exp::Condition(Box::new(cond_exp2)));
        };

        run_funcall1(&mut parser, 1);
        run_funcall2(&mut parser);
        run_funcall1(&mut parser, 2);
        let mut keys = downcast_ctx(parser.context)
            .user_func_types
            .keys()
            .collect::<Vec<_>>();
        keys.sort();
        assert_eq!(keys, vec![&String::from("fun@1"), &String::from("fun@2")]);
    }

    #[test]
    fn simple_test() {
        let mut parser = SyntaxParser::new();
        assert_eq!(
            parser.parse_exp(&mut Exp::Na(NaNode::new(StrRange::new_empty()))),
            Ok(ParseValue::new_with_type(SyntaxType::Simple(
                SimpleSyntaxType::Na
            )))
        );

        assert_eq!(
            parser.parse_exp(&mut Exp::Bool(BoolNode::new_no_range(true))),
            Ok(ParseValue::new_with_type(SyntaxType::Simple(
                SimpleSyntaxType::Bool
            )))
        );

        assert_eq!(
            parser.parse_exp(&mut Exp::Num(Numeral::from_f64(1f64))),
            Ok(ParseValue::new_with_type(SyntaxType::Simple(
                SimpleSyntaxType::Float
            )))
        );

        assert_eq!(
            parser.parse_exp(&mut Exp::Num(Numeral::from_i64(1))),
            Ok(ParseValue::new_with_type(SyntaxType::Simple(
                SimpleSyntaxType::Int
            )))
        );

        assert_eq!(
            parser.parse_exp(&mut Exp::Color(ColorNode::from_str("#AAAAAA"))),
            Ok(ParseValue::new_with_type(SyntaxType::Simple(
                SimpleSyntaxType::Color
            )))
        );

        assert_eq!(
            parser.parse_exp(&mut Exp::Str(StringNode::new(
                String::from("hello"),
                StrRange::new_empty()
            ))),
            Ok(ParseValue::new_with_type(SyntaxType::Simple(
                SimpleSyntaxType::String
            )))
        );
    }

    #[test]
    fn varname_test() {
        let mut parser = SyntaxParser::new();
        downcast_ctx(parser.context).declare_var_with_index("hello", INT_TYPE);
        let mut varname = rvarname("hello");
        assert_eq!(
            parser.parse_varname(&mut varname),
            Ok(ParseValue::new(INT_TYPE, "hello"))
        );
        assert_eq!(varname.var_index, VarIndex::new(0, 0));

        let mut varname = rvarname("notExist");
        assert_eq!(
            parser.parse_varname(&mut varname),
            Ok(ParseValue::new(
                SyntaxType::Simple(SimpleSyntaxType::Na),
                "notExist"
            ))
        );
        assert_eq!(
            parser.errors,
            vec![PineInputError::new(
                PineErrorKind::VarNotDeclare,
                StrRange::new_empty()
            )]
        )
    }

    #[test]
    fn input_varname_test() {
        use crate::syntax::SimpleInputSrcDetector;

        let input_detector = SimpleInputSrcDetector::new(vec!["close", "open", "high", "low"]);
        let mut parser = SyntaxParser::new();
        downcast_ctx(parser.context).declare_var_with_index("close", INT_TYPE);
        downcast_ctx(parser.context).declare_var_with_index("open", INT_TYPE);
        downcast_ctx(parser.context).set_input_detector(&input_detector);
        // downcast_ctx(parser.context).set_input_options(vec!["close", "open", "high", "low"]);
        let mut varname = rvarname("close");
        assert_eq!(
            parser.parse_varname(&mut varname),
            Ok(ParseValue::new(INT_TYPE, "close"))
        );
        assert_eq!(downcast_ctx(parser.context).get_inputnames(), vec!["close"]);

        assert_eq!(
            parser.parse_varname(&mut varname),
            Ok(ParseValue::new(INT_TYPE, "close"))
        );
        assert_eq!(downcast_ctx(parser.context).get_inputnames(), vec!["close"]);

        let mut subctx = SyntaxContext::new(
            Some(NonNull::new(parser.context).unwrap()),
            ContextType::Normal,
        );
        parser.context = &mut subctx;
        let mut varname = rvarname("open");
        assert_eq!(
            parser.parse_varname(&mut varname),
            Ok(ParseValue::new(INT_TYPE, "open"))
        );
        assert_eq!(
            downcast_ctx(parser.context).get_inputnames(),
            Vec::<&'static str>::new()
        );
        let parent = downcast_ctx(parser.context).parent.unwrap();
        assert_eq!(
            downcast_ctx(parent.as_ptr()).get_inputnames(),
            vec!["close", "open"]
        );
    }

    #[test]
    fn tuple_test() {
        let mut parser = SyntaxParser::new();
        downcast_ctx(parser.context).declare_var_with_index("arg1", INT_TYPE);
        downcast_ctx(parser.context).declare_var_with_index("arg2", FLOAT_TYPE);
        assert_eq!(
            parser.parse_exp(&mut Exp::Tuple(Box::new(TupleNode::new(
                vec![
                    Exp::VarName(rvarname("arg1")),
                    Exp::VarName(rvarname("arg2"))
                ],
                StrRange::new_empty()
            )))),
            Ok(ParseValue::new_with_type(SyntaxType::Tuple(Rc::new(vec![
                INT_TYPE, FLOAT_TYPE
            ]))))
        );
    }

    #[test]
    fn tuple_type_cast() {
        let mut parser = SyntaxParser::new();
        downcast_ctx(parser.context)
            .declare_var_with_index("arg1", SyntaxType::Series(SimpleSyntaxType::Float));

        assert_eq!(
            parser.parse_exp(&mut Exp::TypeCast(Box::new(TypeCast::new_no_input(
                DataType::Int,
                Exp::VarName(rvarname("arg1"))
            )))),
            Ok(ParseValue::new_with_type(SyntaxType::Series(
                SimpleSyntaxType::Int
            )))
        );
    }

    #[test]
    fn ref_call_cast() {
        let mut parser = SyntaxParser::new();
        let context = downcast_ctx(parser.context);
        context.declare_var_with_index("series", SyntaxType::Series(SimpleSyntaxType::Float));
        context.declare_var_with_index("simple", SyntaxType::Simple(SimpleSyntaxType::Float));
        context.declare_var_with_index("arg", SyntaxType::Series(SimpleSyntaxType::Float));

        assert_eq!(
            parser.parse_exp(&mut Exp::RefCall(Box::new(RefCall::new_no_input(
                Exp::VarName(rvarname("series")),
                Exp::VarName(rvarname("arg"))
            )))),
            Ok(ParseValue::new_with_type(SyntaxType::Series(
                SimpleSyntaxType::Float
            )))
        );
        assert_eq!(parser.errors.len(), 1);

        assert_eq!(
            parser.parse_exp(&mut Exp::RefCall(Box::new(RefCall::new_no_input(
                Exp::VarName(rvarname("simple")),
                Exp::VarName(rvarname("arg"))
            )))),
            Ok(ParseValue::new_with_type(SyntaxType::Series(
                SimpleSyntaxType::Float
            )))
        );
        assert_eq!(
            context.get_var_scope("simple"),
            Some(&SyntaxType::Series(SimpleSyntaxType::Float))
        );

        assert!(parser
            .parse_exp(&mut Exp::RefCall(Box::new(RefCall::new_no_input(
                Exp::VarName(rvarname("mm")),
                Exp::VarName(rvarname("arg"))
            ))))
            .is_err());
    }

    #[test]
    fn condition_test() {
        let mut parser = SyntaxParser::new();
        let context = downcast_ctx(parser.context);
        context.declare_var_with_index("cond", SyntaxType::Series(SimpleSyntaxType::Float));
        context.declare_var_with_index("arg1", SyntaxType::Series(SimpleSyntaxType::Float));
        context.declare_var_with_index("arg2", SyntaxType::Series(SimpleSyntaxType::Int));

        assert_eq!(
            parser.parse_exp(&mut Exp::Condition(Box::new(Condition::new_no_input(
                Exp::VarName(rvarname("cond")),
                Exp::VarName(rvarname("arg1")),
                Exp::VarName(rvarname("arg2"))
            )))),
            Ok(ParseValue::new_with_type(SyntaxType::Series(
                SimpleSyntaxType::Float
            )))
        );

        assert_eq!(
            parser.parse_exp(&mut Exp::Condition(Box::new(Condition::new_no_input(
                Exp::Num(Numeral::from_i64(1)),
                Exp::Num(Numeral::from_i64(2)),
                Exp::Num(Numeral::from_i64(3))
            )))),
            Ok(ParseValue::new_with_type(SyntaxType::Series(
                SimpleSyntaxType::Int
            )))
        );
    }

    #[test]
    fn if_then_else_exp_test() {
        use crate::ast::stat_expr::if_then_else_exp;
        use crate::ast::state::AstState;

        let mut parser = SyntaxParser::new();
        let context = downcast_ctx(parser.context);
        context.declare_var_with_index("mvar", SyntaxType::Simple(SimpleSyntaxType::Int));

        let input = Input::new_with_str("if mvar\n    1\nelse\n    1.0");
        let mut exp = if_then_else_exp(input, &AstState::new()).unwrap().1;
        assert_eq!(
            parser.parse_ifthenelse_exp(&mut exp),
            Ok(ParseValue::new_with_type(SyntaxType::Series(
                SimpleSyntaxType::Float
            )))
        );
        assert_eq!(downcast_ctx(parser.context).subctxs.len(), 2);
        assert_eq!(downcast_ctx(parser.context).max_child_ctx_index, 1);
        assert_eq!(exp.result_type, SyntaxType::Series(SimpleSyntaxType::Float));
        assert_eq!(exp.then_ctxid, 0);
        assert_eq!(exp.else_ctxid, 1);

        let input = Input::new_with_str("if 1\n    1\n");
        let mut exp = if_then_else_exp(input, &AstState::new()).unwrap().1;
        assert_eq!(
            parser.parse_ifthenelse_exp(&mut exp),
            Ok(ParseValue::new_with_type(SyntaxType::Series(
                SimpleSyntaxType::Int
            )))
        );
        assert_eq!(exp.result_type, SyntaxType::Series(SimpleSyntaxType::Int));
        assert_eq!(exp.then_ctxid, 2);
    }

    #[test]
    fn if_else_exp2_test() {
        use crate::ast::stat_expr::if_then_else_exp;
        use crate::ast::state::AstState;

        let mut parser = SyntaxParser::new();
        let context = downcast_ctx(parser.context);
        context.declare_var_with_index("mvar", SyntaxType::Simple(SimpleSyntaxType::Int));

        let input = Input::new_with_str("if mvar\n    1\n");
        let mut exp = if_then_else_exp(input, &AstState::new()).unwrap().1;
        assert_eq!(
            parser.parse_ifthenelse_exp(&mut exp),
            Ok(ParseValue::new_with_type(SyntaxType::Series(
                SimpleSyntaxType::Int
            )))
        );
    }

    #[test]
    fn if_else_exp3_test() {
        use crate::ast::stat_expr::if_then_else_exp;
        use crate::ast::state::AstState;

        let mut parser = SyntaxParser::new();
        let context = downcast_ctx(parser.context);
        context.declare_var_with_index("mvar", SyntaxType::Simple(SimpleSyntaxType::Int));
        let input =
            Input::new_with_str("if mvar\n    mvar = 1\n    mvar\nelse\n    mvar=1.0\n    mvar");
        assert_eq!(
            parser.parse_ifthenelse_exp(&mut if_then_else_exp(input, &AstState::new()).unwrap().1),
            Ok(ParseValue::new_with_type(SyntaxType::Series(
                SimpleSyntaxType::Float
            )))
        );
    }

    #[test]
    fn for_range_exp_test() {
        use crate::ast::stat_expr::for_range_exp;
        use crate::ast::state::AstState;

        let mut parser = SyntaxParser::new();
        let context = downcast_ctx(parser.context);
        context.declare_var_with_index("var", SyntaxType::Simple(SimpleSyntaxType::Int));

        let input = Input::new_with_str("for i = 1 to 2\n    i");
        let mut for_range = for_range_exp(input, &AstState::new()).unwrap().1;
        assert_eq!(
            parser.parse_forrange_exp(&mut for_range),
            Ok(ParseValue::new_with_type(SyntaxType::Series(
                SimpleSyntaxType::Int
            )))
        );
        assert_eq!(for_range.varid, 0);
        assert_eq!(downcast_ctx(parser.context).subctxs.len(), 1);
        assert_eq!(downcast_ctx(parser.context).max_child_ctx_index, 0);
    }

    #[test]
    fn unary_exp_test() {
        use crate::ast::stat_expr_types::UnaryExp;

        let mut parser = SyntaxParser::new();
        let context = downcast_ctx(parser.context);

        let mut plus_exp = UnaryExp::new(UnaryOp::Plus, int_exp(1), StrRange::new_empty());
        assert_eq!(
            parser.parse_unary(&mut plus_exp),
            Ok(ParseValue::new_with_type(SyntaxType::Simple(
                SimpleSyntaxType::Int
            )))
        );

        context.declare_var_with_index("var", SyntaxType::Series(SimpleSyntaxType::Int));
        let mut series_plus_exp = UnaryExp::new(
            UnaryOp::Plus,
            Exp::VarName(rvarname("var")),
            StrRange::new_empty(),
        );
        assert_eq!(
            parser.parse_unary(&mut series_plus_exp),
            Ok(ParseValue::new_with_type(SyntaxType::Series(
                SimpleSyntaxType::Int
            )))
        );

        let mut na_plus_exp = UnaryExp::new(
            UnaryOp::Plus,
            Exp::Na(NaNode::new(StrRange::new_empty())),
            StrRange::new_empty(),
        );
        assert_eq!(
            parser.parse_unary(&mut na_plus_exp),
            Err(PineInputError::new(
                PineErrorKind::UnaryTypeNotNum,
                StrRange::new_empty()
            ))
        );

        let mut bool_exp = UnaryExp::new(UnaryOp::BoolNot, int_exp(1), StrRange::new_empty());
        assert_eq!(
            parser.parse_unary(&mut bool_exp),
            Ok(ParseValue::new_with_type(SyntaxType::Simple(
                SimpleSyntaxType::Bool
            )))
        );

        // context.declare_var_with_index("var", SyntaxType::Series(SimpleSyntaxType::Int));
        let mut series_plus_exp = UnaryExp::new(
            UnaryOp::BoolNot,
            Exp::VarName(rvarname("var")),
            StrRange::new_empty(),
        );
        assert_eq!(
            parser.parse_unary(&mut series_plus_exp),
            Ok(ParseValue::new_with_type(SyntaxType::Series(
                SimpleSyntaxType::Bool
            )))
        );
    }

    #[test]
    fn add_binary_exp_test() {
        use crate::ast::string::StringNode;

        let mut parser = SyntaxParser::new();
        let context = downcast_ctx(parser.context);

        let mut str_add_exp = BinaryExp::new(
            BinaryOp::Plus,
            Exp::Str(StringNode::new(String::from("he"), StrRange::new_empty())),
            Exp::Str(StringNode::new(String::from("llo"), StrRange::new_empty())),
            StrRange::new_empty(),
        );
        assert_eq!(
            parser.parse_binary(&mut str_add_exp),
            Ok(ParseValue::new_with_type(SyntaxType::Simple(
                SimpleSyntaxType::String
            )))
        );
        assert_eq!(
            str_add_exp.result_type,
            SyntaxType::Simple(SimpleSyntaxType::String)
        );

        context.declare_var_with_index("str1", SyntaxType::Series(SimpleSyntaxType::String));
        context.declare_var_with_index("str2", SyntaxType::Series(SimpleSyntaxType::String));
        let mut sstr_add_exp = BinaryExp::new(
            BinaryOp::Plus,
            Exp::VarName(rvarname("str1")),
            Exp::VarName(rvarname("str2")),
            StrRange::new_empty(),
        );
        assert_eq!(
            parser.parse_binary(&mut sstr_add_exp),
            Ok(ParseValue::new_with_type(SyntaxType::Series(
                SimpleSyntaxType::String
            )))
        );
        assert_eq!(
            sstr_add_exp.result_type,
            SyntaxType::Series(SimpleSyntaxType::String)
        );

        let mut int_add_exp = BinaryExp::new(
            BinaryOp::Plus,
            int_exp(1),
            int_exp(2),
            StrRange::new_empty(),
        );
        assert_eq!(
            parser.parse_binary(&mut int_add_exp),
            Ok(ParseValue::new_with_type(SyntaxType::Simple(
                SimpleSyntaxType::Int
            )))
        );
        assert_eq!(
            int_add_exp.result_type,
            SyntaxType::Simple(SimpleSyntaxType::Int)
        );

        context.declare_var_with_index("sint", SyntaxType::Series(SimpleSyntaxType::Int));
        let mut int_add_exp = BinaryExp::new(
            BinaryOp::Plus,
            int_exp(1),
            Exp::VarName(rvarname("sint")),
            StrRange::new_empty(),
        );
        assert_eq!(
            parser.parse_binary(&mut int_add_exp),
            Ok(ParseValue::new_with_type(SyntaxType::Series(
                SimpleSyntaxType::Int
            )))
        );
        assert_eq!(
            int_add_exp.result_type,
            SyntaxType::Series(SimpleSyntaxType::Int)
        );

        context.declare_var_with_index("sfloat", SyntaxType::Series(SimpleSyntaxType::Int));
        let mut f_i_add_exp = BinaryExp::new(
            BinaryOp::Plus,
            float_exp(1f64),
            Exp::VarName(rvarname("sfloat")),
            StrRange::new_empty(),
        );
        assert_eq!(
            parser.parse_binary(&mut f_i_add_exp),
            Ok(ParseValue::new_with_type(SyntaxType::Series(
                SimpleSyntaxType::Float
            )))
        );
        assert_eq!(
            f_i_add_exp.result_type,
            SyntaxType::Series(SimpleSyntaxType::Float)
        );

        let mut na_add_exp = BinaryExp::new(
            BinaryOp::Plus,
            int_exp(1),
            Exp::Na(NaNode::new(StrRange::new_empty())),
            StrRange::new_empty(),
        );
        assert_eq!(
            parser.parse_binary(&mut na_add_exp),
            Err(PineInputError::new(
                PineErrorKind::BinaryTypeNotNum,
                StrRange::new_empty()
            ))
        );
        assert_eq!(na_add_exp.result_type, SyntaxType::Any);
    }

    #[test]
    fn eq_exp_test() {
        let mut parser = SyntaxParser::new();
        let context = downcast_ctx(parser.context);
        context.declare_var_with_index("sint", SyntaxType::Series(SimpleSyntaxType::Int));

        let mut eq_exp = BinaryExp::new(
            BinaryOp::Eq,
            int_exp(1),
            Exp::VarName(rvarname("sint")),
            StrRange::new_empty(),
        );
        assert_eq!(
            parser.parse_binary(&mut eq_exp),
            Ok(ParseValue::new_with_type(SyntaxType::Series(
                SimpleSyntaxType::Bool
            )))
        );
        assert_eq!(eq_exp.ref_type, SyntaxType::Series(SimpleSyntaxType::Int));

        let mut eq_exp = BinaryExp::new(
            BinaryOp::Eq,
            float_exp(1f64),
            Exp::VarName(rvarname("sint")),
            StrRange::new_empty(),
        );
        assert_eq!(
            parser.parse_binary(&mut eq_exp),
            Ok(ParseValue::new_with_type(SyntaxType::Series(
                SimpleSyntaxType::Bool
            )))
        );
        assert_eq!(eq_exp.ref_type, SyntaxType::Series(SimpleSyntaxType::Float));

        let mut eq_exp = BinaryExp::new(
            BinaryOp::Eq,
            int_exp(1),
            Exp::Na(NaNode::new(StrRange::new_empty())),
            StrRange::new_empty(),
        );
        assert_eq!(
            parser.parse_binary(&mut eq_exp),
            Ok(ParseValue::new_with_type(SyntaxType::Simple(
                SimpleSyntaxType::Bool
            )))
        );
        // assert!(!parser.errors.is_empty());
        assert_eq!(
            eq_exp.result_type,
            SyntaxType::Simple(SimpleSyntaxType::Bool)
        );

        let mut eq_dif_type_exp = BinaryExp::new(
            BinaryOp::Eq,
            int_exp(1),
            Exp::Str(StringNode::new(String::from("h"), StrRange::new_empty())),
            StrRange::new_empty(),
        );
        assert_eq!(
            parser.parse_binary(&mut eq_dif_type_exp),
            Ok(ParseValue::new_with_type(SyntaxType::Simple(
                SimpleSyntaxType::Bool
            )))
        );
        assert_eq!(
            *parser.errors.last().unwrap(),
            PineInputError::new(PineErrorKind::TypeMismatch, StrRange::new_empty())
        );
    }

    #[test]
    fn binary_exp_test() {
        let mut parser = SyntaxParser::new();
        let context = downcast_ctx(parser.context);

        // Geq
        context.declare_var_with_index("sint", SyntaxType::Series(SimpleSyntaxType::Int));
        let mut geq_exp =
            BinaryExp::new(BinaryOp::Geq, int_exp(1), int_exp(2), StrRange::new_empty());
        assert_eq!(
            parser.parse_binary(&mut geq_exp),
            Ok(ParseValue::new_with_type(SyntaxType::Simple(
                SimpleSyntaxType::Bool
            )))
        );
        assert_eq!(
            geq_exp.result_type,
            SyntaxType::Simple(SimpleSyntaxType::Bool)
        );

        let mut series_geq_exp = BinaryExp::new(
            BinaryOp::Geq,
            int_exp(1),
            Exp::VarName(rvarname("sint")),
            StrRange::new_empty(),
        );
        assert_eq!(
            parser.parse_binary(&mut series_geq_exp),
            Ok(ParseValue::new_with_type(SyntaxType::Series(
                SimpleSyntaxType::Bool
            )))
        );
        assert_eq!(
            series_geq_exp.result_type,
            SyntaxType::Series(SimpleSyntaxType::Bool)
        );

        let mut na_geq_exp = BinaryExp::new(
            BinaryOp::Geq,
            int_exp(1),
            Exp::Na(NaNode::new(StrRange::new_empty())),
            StrRange::new_empty(),
        );
        assert_eq!(
            parser.parse_binary(&mut na_geq_exp),
            Ok(ParseValue::new_with_type(SyntaxType::Simple(
                SimpleSyntaxType::Bool
            )))
        );
        assert_eq!(
            *parser.errors.last().unwrap(),
            PineInputError::new(PineErrorKind::BinaryTypeNotNum, StrRange::new_empty())
        );

        // BoolAnd
        context.declare_var_with_index("var", SyntaxType::Series(SimpleSyntaxType::Int));
        let mut bool_and_exp = BinaryExp::new(
            BinaryOp::BoolAnd,
            int_exp(1),
            Exp::VarName(rvarname("var")),
            StrRange::new_empty(),
        );
        assert_eq!(
            parser.parse_binary(&mut bool_and_exp),
            Ok(ParseValue::new_with_type(SyntaxType::Series(
                SimpleSyntaxType::Bool
            )))
        );
        let mut bool_and_exp = BinaryExp::new(
            BinaryOp::BoolAnd,
            int_exp(1),
            Exp::Str(StringNode::new(String::from("h"), StrRange::new_empty())),
            StrRange::new_empty(),
        );
        assert_eq!(
            parser.parse_binary(&mut bool_and_exp),
            Ok(ParseValue::new_with_type(SyntaxType::Simple(
                SimpleSyntaxType::Bool
            )))
        );
        assert_eq!(
            *parser.errors.last().unwrap(),
            PineInputError::new(PineErrorKind::BoolExpTypeNotBool, StrRange::new_empty())
        );
    }

    #[test]
    fn prefix_exp_test() {
        use std::collections::BTreeMap;

        let mut parser = SyntaxParser::new();
        let context = downcast_ctx(parser.context);

        let map2: BTreeMap<_, _> = [("key2", SyntaxType::Simple(SimpleSyntaxType::Int))]
            .iter()
            .cloned()
            .collect();
        let map1: BTreeMap<_, _> = [("key1", SyntaxType::Object(Rc::new(map2)))]
            .iter()
            .cloned()
            .collect();
        context.declare_var_with_index("var", SyntaxType::Object(Rc::new(map1)));

        let mut prefix_exp = PrefixExp::new_no_input(
            Exp::PrefixExp(Box::new(PrefixExp::new_no_input(
                Exp::VarName(rvarname("var")),
                varname("key1"),
            ))),
            varname("key2"),
        );
        assert_eq!(
            parser.parse_prefix(&mut prefix_exp),
            Ok(ParseValue::new_with_type(SyntaxType::Simple(
                SimpleSyntaxType::Int
            )))
        );
        // assert_eq!(prefix_exp.var_index, VarIndex::new(0, 0));

        let mut prefix_exp =
            PrefixExp::new_no_input(Exp::VarName(rvarname("var")), varname("nokey"));
        assert_eq!(
            parser.parse_prefix(&mut prefix_exp),
            Err(PineInputError::new(
                PineErrorKind::RefKeyNotExist,
                StrRange::new_empty()
            ))
        );

        let mut prefix_exp =
            PrefixExp::new_no_input(Exp::VarName(rvarname("var2")), varname("nokey"));
        assert_eq!(
            parser.parse_prefix(&mut prefix_exp),
            Err(PineInputError::new(
                PineErrorKind::RefObjTypeNotObj,
                StrRange::new_empty()
            ))
        );
    }

    #[test]
    fn if_then_else_stmt_test() {
        use crate::ast::stat_expr::if_then_else_with_indent;
        use crate::ast::state::AstState;

        let mut parser = SyntaxParser::new();
        let context = downcast_ctx(parser.context);
        context.declare_var_with_index("mvar", SyntaxType::Simple(SimpleSyntaxType::Int));

        let input = Input::new_with_str("if mvar\n    1\nelse\n    na");
        assert_eq!(
            parser.parse_ifthenelse_stmt(
                &mut if_then_else_with_indent(input, &AstState::new()).unwrap().1
            ),
            Ok(ParseValue::new_with_type(SyntaxType::Void))
        );
    }

    #[test]
    fn assign_test() {
        use crate::ast::stat_expr::assign_stmt;
        use crate::ast::state::AstState;

        let mut parser = SyntaxParser::new();
        let context = downcast_ctx(parser.context);
        context.declare_var_with_index("var", SyntaxType::Simple(SimpleSyntaxType::Int));

        let input = Input::new_with_str("a = 1");
        let val_type = SyntaxType::Simple(SimpleSyntaxType::Int);
        let mut assign = assign_stmt(input, &AstState::new()).unwrap().1;
        assert_eq!(
            parser.parse_assign(&mut assign),
            Ok(ParseValue::new_with_type(val_type.clone()))
        );
        assert_eq!(context.get_var_scope("a"), Some(&val_type));
        assert_eq!(assign.varids, Some(vec![1]));

        // context.var_indexs = HashMap::new();
        let mut assign = assign_stmt(input, &AstState::new()).unwrap().1;
        assert_eq!(
            parser.parse_assign(&mut assign),
            Ok(ParseValue::new_with_type(val_type.clone()))
        );
        assert_eq!(assign.varids, Some(vec![1]));
        assert_eq!(
            parser.errors.last(),
            Some(&PineInputError::new(
                PineErrorKind::VarHasDeclare,
                StrRange::from_start("a", Position::new(0, 0))
            ))
        );

        context.var_indexs = HashMap::new();
        let input = Input::new_with_str("[a1, a2] = [1, 2]");
        let mut assign = assign_stmt(input, &AstState::new()).unwrap().1;
        assert_eq!(
            parser.parse_assign(&mut assign),
            Ok(ParseValue::new_with_type(SyntaxType::Tuple(Rc::new(vec![
                val_type.clone(),
                val_type.clone()
            ]))))
        );
        assert_eq!(assign.varids, Some(vec![2, 3]));

        context.var_indexs = HashMap::new();
        let input = Input::new_with_str("int [a1, a2] = [1.0, 2.0]");
        assert_eq!(
            parser.parse_assign(&mut assign_stmt(input, &AstState::new()).unwrap().1),
            Ok(ParseValue::new_with_type(SyntaxType::Tuple(Rc::new(vec![
                val_type.clone(),
                val_type.clone()
            ]))))
        );
        assert_matches!(
            parser.errors.last().unwrap(),
            &PineInputError {
                code: PineErrorKind::InvalidTypeCast { origin: _, cast: _ },
                range: _,
            }
        );
    }

    #[test]
    fn var_assign_test() {
        use crate::ast::stat_expr::var_assign_stmt;
        use crate::ast::state::AstState;

        let mut parser = SyntaxParser::new();
        let context = downcast_ctx(parser.context);
        context.declare_var_with_index("a", SyntaxType::Simple(SimpleSyntaxType::Float));

        let input = Input::new_with_str("a := 1");
        let val_type = SyntaxType::Series(SimpleSyntaxType::Float);
        let mut assign = var_assign_stmt(input, &AstState::new()).unwrap().1;
        assert_eq!(
            parser.parse_var_assign(&mut assign),
            Ok(ParseValue::new_with_type(val_type.clone()))
        );

        context.vars.remove("a");
        assert_eq!(
            parser.parse_var_assign(&mut assign),
            Err(PineInputError::new(
                PineErrorKind::VarNotDeclare,
                StrRange::from_start("a := 1", Position::new(0, 0))
            ))
        );
        assert_eq!(assign.var_index, VarIndex::new(0, 0));
    }

    const BLOCK: &str = "a = if 1\n    2\nelse\n    4\nb = for i = 1 to 2\n    i\nmyfun(x, y) => x + y\nmyfun(1, 2)\nmyfun(1, 2)";

    #[test]
    fn block_test() {
        use crate::ast::stat_expr::block;
        use crate::ast::state::AstState;

        let mut parser = SyntaxParser::new();

        let input = Input::new_with_str(BLOCK);
        let myblk = block(input, &AstState::new());
        let mut blk = myblk.unwrap().1;
        assert_eq!(
            parser.parse_blk(&mut blk),
            Ok(ParseValue::new_with_type(SyntaxType::Void))
        );
        assert_eq!(blk.var_count, 3);
        assert_eq!(blk.subctx_count, 5);
        assert_eq!(blk.libfun_count, 0);
    }

    #[test]
    fn dynamic_expr_test() {
        use crate::ast::stat_expr::block;
        use crate::ast::state::AstState;

        let mut parser = SyntaxParser::new();
        let context = downcast_ctx(parser.context);
        context.declare_var_with_index(
            "security",
            SyntaxType::Function(Rc::new(FunctionTypes(vec![FunctionType::new((
                vec![(
                    "m",
                    SyntaxType::DynamicExpr(Box::new(SyntaxType::float_series())),
                )],
                SyntaxType::float_series(),
            ))]))),
        );
        context.declare_var_with_index("close", SyntaxType::float_series());

        let blk_str: &str = "a = close + 1\nsecurity(close + a)";

        let input = Input::new_with_str(blk_str);
        let myblk = block(input, &AstState::new());
        let mut blk = myblk.unwrap().1;
        assert_eq!(
            parser.parse_blk(&mut blk),
            Ok(ParseValue::new_with_type(SyntaxType::Void))
        );
        assert_eq!(blk.stmts.len(), 3);
        assert_eq!(blk.var_count, 4);
        assert_eq!(blk.subctx_count, 0);
        assert_eq!(blk.libfun_count, 1);

        fn get_funcall<'a, 'b>(s: &'b Statement<'a>) -> &'b FunctionCall<'a> {
            match s {
                Statement::FuncCall(m) => &*m,
                Statement::Exp(Exp::FuncCall(m)) => &*m,
                _ => unreachable!(),
            }
        }

        fn get_fundef<'a, 'b>(s: &'b Statement<'a>) -> &'b FunctionDef<'a> {
            match s {
                Statement::FuncDef(m) => &*m,
                _ => unreachable!(),
            }
        }

        fn get_varname<'a, 'b>(s: &'b Exp<'a>) -> &'b RVVarName<'a> {
            match s {
                Exp::VarName(m) => &*m,
                _ => unreachable!(),
            }
        }
        assert_eq!(
            get_varname(&get_funcall(&blk.stmts[2]).pos_args[0]).var_index,
            VarIndex::new(get_fundef(&blk.stmts[0]).name_varid, 0)
        );
        assert_eq!(get_fundef(&blk.stmts[0]).body.stmts.len(), 1);

        match get_fundef(&blk.stmts[0]).body.ret_stmt.as_ref().unwrap() {
            Exp::BinaryExp(_) => assert!(true),
            _ => assert!(false),
        };
    }
}
