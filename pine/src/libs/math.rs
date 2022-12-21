use super::xloc::*;
use super::VarResult;
use crate::ast::syntax_type::{FunctionType, FunctionTypes, SimpleSyntaxType, SyntaxType,};
use crate::helper::err_msgs::*;
use crate::helper::str_replace;
use crate::helper::{
    move_element, pine_ref_to_color, pine_ref_to_f64, pine_ref_to_i64, pine_ref_to_string,
};
use crate::runtime::context::Ctx;

use crate::types::{
    downcast_pf, Callable, CallableFactory, CallableObject, DataType, Float, Int, Object,
    PineClass, PineFrom, PineRef, PineStaticType, PineType, RefData, RuntimeErr, SecondType,
    Series, SeriesCall, SimpleType, NA,
};

use crate::libs::{abs, avg, ceil, cos, max, pow};

use std::cell::RefCell;
use std::collections::BTreeMap;
use std::fmt;
use std::mem;
use std::ops::Deref;
use std::rc::Rc;

// math.e
//
// 是欧拉数)的命名常数。它等于2.7182818284590452。
const mathematical: f64 = 2.7182818284590452  ;

// const mathematical: &'static f64 = &2.7182818284590452;
// math.phi
//
// 是黄金分割的命名常数。等于1.6180339887498948。
const Goldenratio:  f64 = 1.6180339887498948;
// math.pi
//
// 是阿基米德常数的命名常数。它等于3.1415926535897932。
const PI: f64 = 3.1415926535897932;
/*
math.rphi

是黄金分割率的命名常数。它等于0.6180339887498948。
 */
const Golden_ratio_conjugate:  f64 = 0.6180339887498948;

struct PlotProps;

impl<'a> PineClass<'a> for PlotProps {
    fn custom_type(&self) -> &str {
        "math"
    }

    fn get(&self, _ctx: &mut dyn Ctx<'a>, name: &str) -> Result<PineRef<'a>, RuntimeErr> {
        match name {
            // "new" => Ok(PineRef::new(CallableFactory::new(|| {
            //     Callable::new(None, Some(Box::new(LineFromNaVal::new())))
            // }))),
            "pow" => Ok(pow::declare_var().value),
            "abs" => Ok(abs::declare_var().value),
            "max" => Ok(max::declare_max_var().value),
            "min" => Ok(max::declare_min_var().value),

            "avg" => Ok(avg::declare_var().value),

            "ceil" => Ok(ceil::declare_ceil_var().value),

            "sin" => Ok(cos::declare_sin_var().value),
            "asin" => Ok(cos::declare_asin_var().value),

            "tan" => Ok(cos::declare_tan_var().value),
            "atan" => Ok(cos::declare_atan_var().value),

            "cos" => Ok(cos::declare_cos_var().value),
            "acos" => Ok(cos::declare_acos_var().value),

            "sqrt" => Ok(cos::declare_sqrt_var().value),
            "exp" => Ok(cos::declare_exp_var().value),
            "log" => Ok(cos::declare_log_var().value),
            "log10" => Ok(cos::declare_log10_var().value),
            "sign" => Ok(cos::declare_sign_var().value),

            "floor" => Ok(ceil::declare_floor_var().value),
            "round" => Ok(ceil::declare_round_var().value),


             "e" => Ok(PineRef::new_box(Some(mathematical))),
             "pi" => Ok(PineRef::new_box(Some(PI))),
             "phi" => Ok(PineRef::new_box(Some(Goldenratio))),
             "rphi" => Ok(PineRef::new_box(Some(Golden_ratio_conjugate))),


            _ => Err(RuntimeErr::NotImplement(str_replace(
                NO_FIELD_IN_OBJECT,
                vec![String::from(name), String::from("line")],
            ))),
        }
    }

    fn copy(&self) -> Box<dyn PineClass<'a> + 'a> {
        Box::new(PlotProps)
    }
}

pub const VAR_NAME: &'static str = "math";

pub fn declare_var<'a>() -> VarResult<'a> {
    // let value = PineRef::new(CallableObject::new(Box::new(PlotProps), || {
    //     Callable::new(None, Some(Box::new(LineFromNaVal::new())))
    // }));
    let value = PineRef::new(Object::new(Box::new(PlotProps)));

    let func_type = FunctionTypes(vec![FunctionType::new((
        vec![("x", SyntaxType::Simple(SimpleSyntaxType::Na))],
        SyntaxType::ObjectClass("ta"),
    ))]);
    let mut obj_type = BTreeMap::new();

    obj_type.insert("pow", pow::declare_var().syntax_type);
    obj_type.insert("abs", abs::declare_var().syntax_type);
    obj_type.insert("max", max::declare_max_var().syntax_type);
    obj_type.insert("min", max::declare_min_var().syntax_type);

    obj_type.insert("avg", avg::declare_var().syntax_type);

    obj_type.insert("ceil", ceil::declare_ceil_var().syntax_type);

    obj_type.insert("sin", cos::declare_sin_var().syntax_type);
    obj_type.insert("asin", cos::declare_asin_var().syntax_type);

    obj_type.insert("tan", cos::declare_tan_var().syntax_type);
    obj_type.insert("atan", cos::declare_atan_var().syntax_type);

    obj_type.insert("cos", cos::declare_cos_var().syntax_type);
    obj_type.insert("acos", cos::declare_acos_var().syntax_type);

    obj_type.insert("sqrt", cos::declare_sqrt_var().syntax_type);
    obj_type.insert("exp", cos::declare_exp_var().syntax_type);

    obj_type.insert("log", cos::declare_log_var().syntax_type);
    obj_type.insert("log10", cos::declare_log10_var().syntax_type);

    obj_type.insert("sign", cos::declare_sign_var().syntax_type);
    obj_type.insert("floor", ceil::declare_floor_var().syntax_type);
    obj_type.insert("round", ceil::declare_round_var().syntax_type);


    obj_type.insert("e", SyntaxType::float());
    obj_type.insert("pi", SyntaxType::float());
    obj_type.insert("phi", SyntaxType::float());
    obj_type.insert("rphi", SyntaxType::float());


    let syntax_type = SyntaxType::ObjectFunction(Rc::new(obj_type), Rc::new(func_type));
    VarResult::new(value, syntax_type, VAR_NAME)
}
