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
use crate::libs::{alma, atr, bb, change, ema, hma, rsi, sma, tr};


struct PlotProps;

impl<'a> PineClass<'a> for PlotProps {
    fn custom_type(&self) -> &str {
        "ta"
    }

    fn get(&self, _ctx: &mut dyn Ctx<'a>, name: &str) -> Result<PineRef<'a>, RuntimeErr> {
        match name {
            // "new" => Ok(PineRef::new(CallableFactory::new(|| {
            //     Callable::new(None, Some(Box::new(LineFromNaVal::new())))
            // }))),
            "hma" => Ok(hma::declare_var().value),
            "sma" => Ok(sma::declare_sma_var().value),
            "wma" => Ok(sma::declare_wma_var().value),
            "dev" => Ok(sma::declare_dev_var().value),
            "variance" => Ok(sma::declare_variance_var().value),
            "stdev" => Ok(sma::declare_stdev_var().value),
            "tr" => Ok(tr::declare_var().value),
            "alma" => Ok(alma::declare_var().value),
            "rsi" => Ok(rsi::declare_var().value),
            "ema" => Ok(ema::declare_ema_var().value),
            "rma" => Ok(ema::declare_rma_var().value),
            "bb" => Ok(bb::declare_var().value),
            "atr" => Ok(atr::declare_var().value),
            "change" => Ok(change::declare_change_var().value),

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

pub const VAR_NAME: &'static str = "ta";

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
        "hma",
        hma::declare_var().syntax_type,
    );
    obj_type.insert(
        "ema",
        ema::declare_ema_var().syntax_type,
    );

    obj_type.insert(
        "rma",
        ema::declare_rma_var().syntax_type,
    );
    obj_type.insert(
        "sma",
        sma::declare_sma_var().syntax_type,
    );

    obj_type.insert(
        "wma",
        sma::declare_wma_var().syntax_type,
    );
    obj_type.insert(
        "dev",
        sma::declare_dev_var().syntax_type,
    );
    obj_type.insert(
        "variance",
        sma::declare_variance_var().syntax_type,
    );
    obj_type.insert(
        "stdev",
        sma::declare_stdev_var().syntax_type,
    );


    obj_type.insert(
        "tr",
        tr::declare_var().syntax_type,
    );
    obj_type.insert(
        "alma",
        alma::declare_var().syntax_type,
    );

    obj_type.insert(
        "rsi",
        rsi::declare_var().syntax_type,
    );
    obj_type.insert(
        "bb",
        bb::declare_var().syntax_type,
    );
obj_type.insert(
        "atr",
        atr::declare_var().syntax_type,
    );obj_type.insert(
        "change",
        change::declare_change_var().syntax_type,
    );


    let syntax_type = SyntaxType::ObjectFunction(Rc::new(obj_type), Rc::new(func_type));
    VarResult::new(value, syntax_type, VAR_NAME)
}
