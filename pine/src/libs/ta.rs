use super::max::declare_max_var;
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
use crate::libs::{accdist, alma, atr, bb, bbw, cci, change, cmo, cog, correlation, cum, dmi, ema, falling, highest, highestbars, hma, kc, kcw, lowest, lowestbars, macd, max, mfi, rising, rsi, sma, stoch, swma, tr, tsi, vwma};


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
            "alma" => Ok(alma::declare_var().value),
            "atr" => Ok(atr::declare_var().value),

            "bb" => Ok(bb::declare_var().value),
            "bbw" => Ok(bbw::declare_var().value),

            "cci" => Ok(cci::declare_var().value),
            "change" => Ok(change::declare_change_var().value),
            "cmo" => Ok(cmo::declare_var().value),
            "cog" => Ok(cog::declare_var().value),
            "correlation" => Ok(correlation::declare_var().value),
            // todo cross crossover crossunder
            "cum" => Ok(cum::declare_var().value),

            "dev" => Ok(sma::declare_dev_var().value),
            "dmi" => Ok(dmi::declare_var().value),

            "ema" => Ok(ema::declare_ema_var().value),
            "falling" => Ok(falling::declare_var().value),


            "highest" => Ok(highest::declare_var().value),
            "highestbars" => Ok(highestbars::declare_var().value),
            "hma" => Ok(hma::declare_var().value),

            "kc" => Ok(kc::declare_var().value),
            "kcw" => Ok(kcw::declare_var().value),

            //todo linereg

            "lowest" => Ok(lowest::declare_var().value),
            "lowestbars" => Ok(lowestbars::declare_var().value),


            "macd" => Ok(macd::declare_var().value),
            "max" => Ok(max::declare_max_var().value),
            //todo ta.median

            "mfi" => Ok(mfi::declare_var().value),
            "min" => Ok(max::declare_min_var().value),

            //todo ta.mode
            "mom" => Ok(change::declare_mom_var().value),

            //todo ta.percentile_linear_interpolation
            // ta.percentile_nearest_rank
            // ta.percentrank
            // 使用最近的两个排名之间的线性插值方法计算百分比。

            /**
            ta.pivothigh
            ta.pivotlow
            此函数返回枢轴高点的价格。 如果没有枢轴高点，则返回“NaN”。
             **/

            /**
            ta.range

            返回序列中最小值和最大值之间的差。
            **/

            "rising" => Ok(rising::declare_var().value),
            "rma" => Ok(ema::declare_rma_var().value),
            "rsi" => Ok(rsi::declare_var().value),

            /**
            ta.roc

            计算 `source` 的当前值与其值 `length` K线之前的涨跌百分比（涨跌幅）。
            由以下公式计算：100 * change(src, length) / src[length]。
            **/

            "sma" => Ok(sma::declare_sma_var().value),
            "stdev" => Ok(sma::declare_stdev_var().value),
            "stoch" => Ok(stoch::declare_var().value),
            /*
            ta.supertrend

            超级趋势指标。超级趋势指标是一个跟随趋势的指标。
            */

            "swma" => Ok(swma::declare_var().value),

            "tr" => Ok(tr::declare_var().value),
            "tsi" => Ok(tsi::declare_var().value),
            "variance" => Ok( sma::declare_variance_var().value),





            "wma" => Ok(sma::declare_wma_var().value),
            // todo ta.vwap

            "accdist" => Ok(accdist::declare_var().value),

            // todo ta.obv
            //
            // 能量潮指标。

            "vwma" => Ok(vwma::declare_var().value),





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
        "alma",
        alma::declare_var().syntax_type,
    );
    obj_type.insert(
        "atr",
        atr::declare_var().syntax_type,
    );

    obj_type.insert(
        "bb",
        bb::declare_var().syntax_type,
    );
    obj_type.insert(
        "bbw",
        bbw::declare_var().syntax_type,
    );

    obj_type.insert(
        "cci",
        cci::declare_var().syntax_type,
    );

    obj_type.insert(
        "change",
        change::declare_change_var().syntax_type,
    );

    obj_type.insert(
        "cmo",
        cmo::declare_var().syntax_type,
    );

    obj_type.insert(
        "cog",
        cog::declare_var().syntax_type,
    );
    obj_type.insert(
        "correlation",
        correlation::declare_var().syntax_type,
    );

    obj_type.insert(
        "cum",
        cum::declare_var().syntax_type,
    );

    obj_type.insert(
        "dev",
        sma::declare_dev_var().syntax_type,
    );

    obj_type.insert(
        "dmi",
        dmi::declare_var().syntax_type,
    );

    obj_type.insert(
        "ema",
        ema::declare_ema_var().syntax_type,
    );
    obj_type.insert(
        "falling",
        falling::declare_var().syntax_type,
    );

    obj_type.insert(
        "highest",
        highest::declare_var().syntax_type,
    );

    obj_type.insert(
        "highestbars",
        highestbars::declare_var().syntax_type,
    );


    obj_type.insert(
        "hma",
        hma::declare_var().syntax_type,
    );

    obj_type.insert(
        "kc",
        kc::declare_var().syntax_type,
    );
  
    obj_type.insert(
        "kcw",
        kcw::declare_var().syntax_type,
    );

    obj_type.insert(
        "lowest",
        lowest::declare_var().syntax_type,
    );
  
    obj_type.insert(
        "lowestbars",
        lowestbars::declare_var().syntax_type,
    );

    obj_type.insert(
        "macd",
        macd::declare_var().syntax_type,
    );
    obj_type.insert(
        "max",
        max::declare_max_var().syntax_type,
    );

    obj_type.insert(
        "mfi",
        mfi::declare_var().syntax_type,
    );

    obj_type.insert(
        "min",
        max::declare_min_var().syntax_type,
    );
    obj_type.insert(
        "mom",
        change::declare_mom_var().syntax_type,
    );

    obj_type.insert(
        "rising",
        rising::declare_var().syntax_type,
    ); 
  

    obj_type.insert(
        "rma",
        ema::declare_rma_var().syntax_type,
    );

    obj_type.insert(
        "rsi",
        rsi::declare_var().syntax_type,
    );



    obj_type.insert(
        "sma",
        sma::declare_sma_var().syntax_type,
    );

    obj_type.insert(
        "stdev",
        sma::declare_stdev_var().syntax_type,
    );

    obj_type.insert(
        "stoch",
        stoch::declare_var().syntax_type,
    );
    obj_type.insert(
        "swma",
        swma::declare_var().syntax_type,
    );

    obj_type.insert(
        "tr",
        tr::declare_var().syntax_type,
    );

    obj_type.insert(
        "tsi",
        tsi::declare_var().syntax_type,
    );

    obj_type.insert(
        "variance",
        sma::declare_variance_var().syntax_type,
    );
    


    obj_type.insert(
        "wma",
        sma::declare_wma_var().syntax_type,
    );

    obj_type.insert(
        "accdist",
        accdist::declare_var().syntax_type,
    );


    obj_type.insert(
        "vwma",
        vwma::declare_var().syntax_type,
    );

    // obj_type.insert(
    //     "dev",
    //     sma::declare_dev_var().syntax_type,
    // );
    

    
    // obj_type.insert(
    //     "alma",
    //     alma::declare_var().syntax_type,
    // );

    
  
    


    let syntax_type = SyntaxType::ObjectFunction(Rc::new(obj_type), Rc::new(func_type));
    VarResult::new(value, syntax_type, VAR_NAME)
}
