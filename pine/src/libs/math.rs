use super::xloc::*;
use super::VarResult;
use crate::ast::syntax_type::{FunctionType, FunctionTypes, SimpleSyntaxType, SyntaxType};
use crate::helper::err_msgs::*;
use crate::helper::str_replace;
use crate::helper::{
    move_element, pine_ref_to_color, pine_ref_to_f64, pine_ref_to_i64,
    pine_ref_to_string,
};
use crate::runtime::context::{Ctx};

use crate::types::{
    downcast_pf, Callable, CallableFactory, CallableObject,
    DataType, Float, Int,Object, PineClass, PineFrom, PineRef, PineStaticType, PineType,
    RefData, RuntimeErr, SecondType, Series, SeriesCall, SimpleType, NA,
};


use std::cell::{RefCell};
use std::collections::BTreeMap;
use std::fmt;
use std::mem;
use std::ops::Deref;
use std::rc::Rc;
use crate::libs::{abs, pow};


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

    obj_type.insert(
        "pow",
        pow::declare_var().syntax_type,
    );
    obj_type.insert(
        "abs",
        abs::declare_var().syntax_type,
    );


    let syntax_type = SyntaxType::ObjectFunction(Rc::new(obj_type), Rc::new(func_type));
    VarResult::new(value, syntax_type, VAR_NAME)
}
