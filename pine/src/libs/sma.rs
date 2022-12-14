use super::VarResult;

use crate::ast::syntax_type::{FunctionType, FunctionTypes, SyntaxType};


use crate::helper::{
    ge1_param_i64, move_element, pine_ref_to_f64_series,
    pine_ref_to_i64, require_param,
};
use crate::runtime::context::{Ctx};

use crate::types::{
    Callable, CallableCreator, CallableFactory, Float, ParamCollectCall, PineRef, RefData, RuntimeErr, Series, SeriesCall,
};
use std::f64;
use std::mem;
use std::rc::Rc;

pub fn series_wma<'a>(series: &Series<Float>, length: i64) -> Result<Float, RuntimeErr> {
    let mut norm = 0f64;
    let mut sum = 0f64;
    for i in 0..length {
        let weight = (length - i) as f64;
        norm += weight;
        match series.index_value(i as usize)? {
            Some(val) => {
                sum += val * weight as f64;
            }
            None => {
                return Ok(Float::from(None));
            }
        }
    }
    Ok(Some(sum / norm))
}

pub fn series_sma<'a>(series: &Series<Float>, length: i64) -> Result<Float, RuntimeErr> {
    let mut sum = 0f64;
    for i in 0..length as usize {
        match series.index_value(i)? {
            Some(val) => {
                sum += val / length as f64;
            }
            None => {
                return Ok(Float::from(None));
            }
        }
    }
    Ok(Some(sum))
}

pub fn series_dev<'a>(series: &Series<Float>, length: i64) -> Result<Float, RuntimeErr> {
    generic_dev_func(series, length, deviation)
}

pub fn series_stdev<'a>(series: &Series<Float>, length: i64) -> Result<Float, RuntimeErr> {
    generic_dev_func(series, length, stdev)
}

pub fn sma_func<'a>(source: RefData<Series<Float>>, length: i64) -> Result<Float, RuntimeErr> {
    series_sma(&*source, length)
}

pub fn wma_func<'a>(source: RefData<Series<Float>>, length: i64) -> Result<Float, RuntimeErr> {
    series_wma(&*source, length)
}

fn deviation(values: Vec<f64>, avg: f64) -> f64 {
    values.iter().fold(0f64, |mul, x| mul + (x - avg).abs()) / values.len() as f64
}

fn variance(values: Vec<f64>, avg: f64) -> f64 {
    values.iter().fold(0f64, |mul, x| mul + (x - avg).powi(2)) / values.len() as f64
}

fn stdev(values: Vec<f64>, avg: f64) -> f64 {
    variance(values, avg).sqrt()
}

// For dev function, if the input source length < length, then we need return na.
pub fn generic_dev_func<'a>(
    source: &Series<Float>,
    length: i64,
    func: fn(Vec<f64>, f64) -> f64,
) -> Result<Float, RuntimeErr> {
    let mut values = vec![];
    for i in 0..length as usize {
        if let Some(val) = source.index_value(i)? {
            values.push(val);
        } else {
            return Ok(None);
        }
    }
    if values.is_empty() {
        return Ok(Float::from(None));
    }
    let avg: f64 = values.iter().sum::<f64>() / values.len() as f64;
    let val = func(values, avg);
    let result = if val.is_nan() { None } else { Some(val) };
    Ok(result)
}

fn dev_func<'a>(source: RefData<Series<Float>>, length: i64) -> Result<Float, RuntimeErr> {
    generic_dev_func(&*source, length, deviation)
}

fn variance_func<'a>(source: RefData<Series<Float>>, length: i64) -> Result<Float, RuntimeErr> {
    generic_dev_func(&*source, length, variance)
}

pub fn stdev_func<'a>(source: RefData<Series<Float>>, length: i64) -> Result<Float, RuntimeErr> {
    generic_dev_func(&*source, length, stdev)
}

type HandleFunc = fn(RefData<Series<Float>>, i64) -> Result<Float, RuntimeErr>;

#[derive(Debug, Clone, PartialEq)]
struct SmaVal {
    ma_func: *mut (),
}

impl SmaVal {
    pub fn new(ma_func: *mut ()) -> SmaVal {
        SmaVal { ma_func }
    }
}

impl<'a> SeriesCall<'a> for SmaVal {
    fn step(
        &mut self,
        _ctx: &mut dyn Ctx<'a>,
        mut param: Vec<Option<PineRef<'a>>>,
        _func_type: FunctionType<'a>,
    ) -> Result<PineRef<'a>, RuntimeErr> {
        move_tuplet!((source, length) = param);

        let func = unsafe { mem::transmute::<_, HandleFunc>(self.ma_func) };
        let source = require_param("source", pine_ref_to_f64_series(source))?;
        let length = ge1_param_i64("length", pine_ref_to_i64(length))?;
        Ok(PineRef::new(Series::from(func(source, length)?)))
    }

    fn copy(&self) -> Box<dyn SeriesCall<'a> + 'a> {
        Box::new(self.clone())
    }
}

#[derive(Debug, Clone, PartialEq)]
struct SmaCreator {
    handle: *mut (),
}

impl SmaCreator {
    pub fn new(handle: *mut ()) -> SmaCreator {
        SmaCreator { handle }
    }
}

impl<'a> CallableCreator<'a> for SmaCreator {
    fn create(&self) -> Callable<'a> {
        Callable::new(
            None,
            Some(Box::new(ParamCollectCall::new_with_caller(Box::new(
                SmaVal::new(self.handle as *mut ()),
            )))),
        )
    }

    fn copy(&self) -> Box<dyn CallableCreator<'a>> {
        Box::new(self.clone())
    }
}

pub fn declare_ma_var<'a>(name: &'static str, handle: HandleFunc) -> VarResult<'a> {
    let value = PineRef::new(CallableFactory::new_with_creator(Box::new(
        SmaCreator::new(handle as *mut ()),
    )));

    let func_type = FunctionTypes(vec![FunctionType::new((
        vec![
            ("source", SyntaxType::float_series()),
            ("length", SyntaxType::int()),
        ],
        SyntaxType::float_series(),
    ))]);
    let syntax_type = SyntaxType::Function(Rc::new(func_type));
    VarResult::new(value, syntax_type, name)
}

pub fn declare_sma_var<'a>() -> VarResult<'a> {
    declare_ma_var("sma", sma_func)
}

pub fn declare_wma_var<'a>() -> VarResult<'a> {
    declare_ma_var("wma", wma_func)
}

pub fn declare_dev_var<'a>() -> VarResult<'a> {
    declare_ma_var("dev", dev_func)
}

pub fn declare_variance_var<'a>() -> VarResult<'a> {
    declare_ma_var("variance", variance_func)
}

pub fn declare_stdev_var<'a>() -> VarResult<'a> {
    declare_ma_var("stdev", stdev_func)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::syntax_type::SyntaxType;
    use crate::runtime::VarOperate;
    use crate::runtime::{AnySeries, NoneCallback};
    use crate::types::Series;
    use crate::{LibInfo, PineParser, PineRunner};
    // use crate::libs::{floor, exp, };

    #[test]
    fn sma_test() {
        let lib_info = LibInfo::new(
            vec![
                declare_sma_var(),
                declare_wma_var(),
                declare_dev_var(),
                declare_variance_var(),
                declare_stdev_var(),
            ],
            vec![("close", SyntaxType::float_series())],
        );
        let src = "m = sma(close, 2)\nm2 = wma(close, 2)\n
        m3 = dev(close, 2)\nm4 = variance(close, 2)\n
        m5 = stdev(close, 2)";
        let blk = PineParser::new(src, &lib_info).parse_blk().unwrap();
        let mut runner = PineRunner::new(&lib_info, &blk, &NoneCallback());

        runner
            .run(
                &vec![(
                    "close",
                    AnySeries::from_float_vec(vec![Some(6f64), Some(12f64)]),
                )],
                None,
            )
            .unwrap();
        let starti = 0;
        assert_eq!(
            runner.get_context().move_var(VarIndex::new(starti, 0)),
            Some(PineRef::new(Series::from_vec(vec![None, Some(9f64)])))
        );
        assert_eq!(
            runner.get_context().move_var(VarIndex::new(starti + 1, 0)),
            Some(PineRef::new(Series::from_vec(vec![None, Some(10f64)])))
        );
        assert_eq!(
            runner.get_context().move_var(VarIndex::new(starti + 2, 0)),
            Some(PineRef::new(Series::from_vec(vec![None, Some(3f64)])))
        );
        assert_eq!(
            runner.get_context().move_var(VarIndex::new(starti + 3, 0)),
            Some(PineRef::new(Series::from_vec(vec![None, Some(9f64)])))
        );
        assert_eq!(
            runner.get_context().move_var(VarIndex::new(starti + 4, 0)),
            Some(PineRef::new(Series::from_vec(vec![None, Some(3f64)])))
        );
    }
}
// 4 * 12 + 2 * 6  4 + 2  5 * 12 / 6 =
