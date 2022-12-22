//
// use super::VarResult;
// use crate::ast::stat_expr_types::VarIndex;
// use crate::ast::syntax_type::{FunctionType, FunctionTypes, SyntaxType};
// use crate::helper::{
//     ensure_srcs, ge1_param_i64, pine_ref_to_f64_series, pine_ref_to_i64, series_index,
// };
// use crate::runtime::context::{Ctx};
//
// use crate::types::{
//     Callable, CallableCreator, CallableFactory, Float,
//     ParamCollectCall, PineRef, RefData, RuntimeErr, Series, SeriesCall,
// };
// use std::mem;
// use std::rc::Rc;
//
// type GenIndexFunc<'a> = fn(&mut dyn Ctx<'a>) -> VarIndex;
// type GetValFunc = fn(&Option<RefData<Series<Float>>>, i64) -> Float;
//
// fn gen_high_index<'a>(ctx: &mut dyn Ctx<'a>) -> VarIndex {
//     VarIndex::new(*ctx.get_varname_index("high").unwrap(), 0)
// }
//
// pub fn get_max_val<'a>(source: &Option<RefData<Series<Float>>>, length: i64) -> Float {
//     let mut max_val = Some(0f64);
//     for i in 0..length as usize {
//         let cur_val = series_index(source, i);
//         if cur_val.is_some() && cur_val > max_val {
//             max_val = cur_val;
//         }
//     }
//     max_val
// }
//
// #[derive(Debug, Clone, PartialEq)]
// struct AtrVal {
//     src_name: &'static str,
//     run_func: *mut (),
//     dest_index: VarIndex,
// }
//
// impl AtrVal {
//     pub fn new(src_name: &'static str, run_func: *mut ()) -> AtrVal {
//         AtrVal {
//             src_name,
//             run_func,
//             dest_index: VarIndex::new(0, 0),
//         }
//     }
// }
//
// impl<'a> SeriesCall<'a> for AtrVal {
//     fn step(
//         &mut self,
//         ctx: &mut dyn Ctx<'a>,
//         mut param: Vec<Option<PineRef<'a>>>,
//         _func_type: FunctionType<'a>,
//     ) -> Result<PineRef<'a>, RuntimeErr> {
//         let source;
//         let length;
//
//         let runner = unsafe { mem::transmute::<_, GetValFunc>(self.run_func) };
//
//         if _func_type.signature.0.len() == 1 {
//             ensure_srcs(ctx, vec![self.src_name], |indexs| {
//                 self.dest_index = indexs[0];
//             });
//
//             source = pine_ref_to_f64_series(ctx.get_var(self.dest_index).clone());
//             length = ge1_param_i64("length", pine_ref_to_i64(mem::replace(&mut param[0], None)))?;
//         } else {
//             source = pine_ref_to_f64_series(mem::replace(&mut param[0], None));
//             length = ge1_param_i64("length", pine_ref_to_i64(mem::replace(&mut param[1], None)))?;
//         }
//         let max_val = runner(&source, length);
//         Ok(PineRef::new_rc(Series::from(max_val)))
//     }
//
//     fn copy(&self) -> Box<dyn SeriesCall<'a> + 'a> {
//         Box::new(self.clone())
//     }
// }
//
// #[derive(Debug, Clone, PartialEq)]
// struct CrossVal {
//     rev: &'static bool,
//     handle: *mut (),
// }
//
// impl CrossVal {
//     pub fn new(rev:  bool, handle: *mut ()) -> CrossVal {
//         CrossVal { rev, handle }
//     }
//     // fn process_cross(
//     //     &mut self,
//     //     _ctx: &mut dyn Ctx<'a>,
//     //     mut param: Vec<Option<PineRef<'a>>>,
//     //     _func_type: FunctionType<'a>,
//     // ) -> Result<bool, RuntimeErr> {
//     //     move_tuplet!((x, y) = param);
//     //
//     //     match _func_type.get_type(1) {
//     //         _ => {
//     //             let s1 = pine_ref_to_f64_series(x.clone());
//     //             let s2 = pine_ref_to_f64_series(y.clone());
//     //             let up = series_minus(&s1, &s2, 0);
//     //             let down = series_minus(&s1, &s2, 1);
//     //
//     //             let result = up.gt(Some(0f64)) && down.lt(Some(0f64));
//     //
//     //             Ok(result)
//     //
//     //             //Ok(PineRef::new_box(pine_ref_to_bool(result)));
//     //
//     //
//     //         }
//     //     }
//     // }
// }
//
// //
// // impl<'a> SeriesCall<'a> for CrossVal<'a> {
// //     fn step(
// //         &mut self,
// //         _ctx: &mut dyn Ctx<'a>,
// //         param: Vec<Option<PineRef<'a>>>,
// //         _func_type: FunctionType<'a>,
// //     ) -> Result<PineRef<'a>, RuntimeErr> {
// //         let res = self.process_cross(_ctx, param, _func_type)?;
// //         // let lookahead = pine_ref_to_bool(val).unwrap_or(false);
// //
// //         Ok(PineRef::new_box(res))
// //
// //
// //     }
// //
// //     fn copy(&self) -> Box<dyn SeriesCall<'a> + 'a> {
// //         Box::new(self.clone())
// //     }
// // }
//
// // impl<'a> CallableCreator<'a> for CrossVal {
// //     fn create(&self) -> Callable<'a> {
// //         Callable::new(
// //             None,
// //             Some(Box::new(ParamCollectCall::new_with_caller(Box::new(
// //                 AtrVal::new(self.src_name, self.handle),
// //             )))),
// //         )
// //     }
// //
// //     fn copy(&self) -> Box<dyn CallableCreator<'a>> {
// //         Box::new(self.clone())
// //     }
// // }
//
// pub fn declare_s_var<'a>(
//     name: &'static str,
//     rev: &'static bool,
//     run_func: GetValFunc,
// ) -> VarResult<'a> {
//     let value = PineRef::new(CallableFactory::new_with_creator(Box::new(
//         CrossVal::new(rev, run_func as *mut ()),
//     )));
//
//     let func_type = FunctionTypes(vec![
//         FunctionType::new((
//             vec![
//                 ("x", SyntaxType::float_series()),
//                 ("y", SyntaxType::float_series()),
//             ],
//             SyntaxType::bool()
//         )),
//     ]);
//     let syntax_type = SyntaxType::Function(Rc::new(func_type));
//     VarResult::new(value, syntax_type, name)
// }
//
// pub fn declare_crossover_var<'a>() -> VarResult<'a> {
//     declare_s_var("crossover", true, get_max_val)
// }
//
// pub fn declare_crossunder_var<'a>() -> VarResult<'a> {
//     declare_s_var("crossunder", false, get_max_val)
// }
//
//
// #[cfg(test)]
// mod tests {
//     use crate::{LibInfo, PineParser, PineRunner, VarIndex};
//     use crate::ast::syntax_type::SyntaxType;
//     use crate::runtime::{AnySeries, NoneCallback};
//     use crate::runtime::VarOperate;
//     use crate::types::Series;
//
//     use super::*;
//
// // use crate::libs::{floor, exp, };
//
//     #[test]
//     fn rsi_int_test() {
//         let lib_info = LibInfo::new(
//             vec![declare_crossover_var(),declare_crossunder_var()],
//             vec![("close", SyntaxType::float_series()),("high", SyntaxType::float_series())],
//         );
//         let src = "m = crossover(close, high)\n";
//         let blk = PineParser::new(src, &lib_info).parse_blk().unwrap();
//         let mut runner = PineRunner::new(&lib_info, &blk, &NoneCallback());
//
//         runner
//             .run(
//                 &vec![(
//                           "close",
//                           AnySeries::from_float_vec(vec![Some(10f64), Some(30f64)]),
//                       ),(
//                           "high",
//                           AnySeries::from_float_vec(vec![Some(15f64), Some(25f64)]),
//                       )],
//                 None,
//             )
//             .unwrap();
//
//         //println!("{:?}",runner.get_context().move_var(VarIndex::new(0, 0)));
//
//         assert_eq!(
//             runner.get_context().move_var(VarIndex::new(0, 0)),
//             Some(PineRef::new_box(true))
//         );
//     }
//
// }
