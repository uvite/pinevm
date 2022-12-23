// use super::VarResult;
// use crate::ast::syntax_type::{SyntaxType};
// use crate::helper::err_msgs::*;
// use crate::helper::str_replace;
//
// use crate::runtime::context::{Ctx};
//
// use crate::types::{
//     Scale, Object, PineClass,
//     PineRef, RuntimeErr,
// };
// use std::collections::BTreeMap;
// use std::rc::Rc;
//
// struct ScaleProps;
//
// impl<'a> PineClass<'a> for ScaleProps {
//     fn custom_type(&self) -> &str {
//         "scale"
//     }
//
//     fn get(&self, _ctx: &mut dyn Ctx<'a>, name: &str) -> Result<PineRef<'a>, RuntimeErr> {
//         match name {
//             "aqua" => Ok(PineRef::new_box(Scale("#00BCD4"))),
//             "black" => Ok(PineRef::new_box(Scale("#363A45"))),
//             "blue" => Ok(PineRef::new_box(Scale("#2196F3"))),
//             "fuchsia" => Ok(PineRef::new_box(Scale("#E040FB"))),
//             "gray" => Ok(PineRef::new_box(Scale("#787B86"))),
//             "green" => Ok(PineRef::new_box(Scale("#4CAF50"))),
//             "lime" => Ok(PineRef::new_box(Scale("#00E676"))),
//             "maroon" => Ok(PineRef::new_box(Scale("#880E4F"))),
//             "navy" => Ok(PineRef::new_box(Scale("#311B92"))),
//             "olive" => Ok(PineRef::new_box(Scale("#808000"))),
//             "orange" => Ok(PineRef::new_box(Scale("#FF9800"))),
//             "purple" => Ok(PineRef::new_box(Scale("#9C27B0"))),
//             "red" => Ok(PineRef::new_box(Scale("#FF5252"))),
//             "silver" => Ok(PineRef::new_box(Scale("#B2B5BE"))),
//             "teal" => Ok(PineRef::new_box(Scale("#00897B"))),
//             "white" => Ok(PineRef::new_box(Scale("#FFFFFF"))),
//             "yellow" => Ok(PineRef::new_box(Scale("#FFEB3B"))),
//             _ => Err(RuntimeErr::NotImplement(str_replace(
//                 NO_FIELD_IN_OBJECT,
//                 vec![String::from(name), String::from("scale")],
//             ))),
//         }
//     }
//
//     fn copy(&self) -> Box<dyn PineClass<'a> + 'a> {
//         Box::new(ScaleProps)
//     }
// }
//
// pub const VAR_NAME: &'static str = "scale";
//
// pub fn declare_var<'a>() -> VarResult<'a> {
//     let value = PineRef::new(Object::new(Box::new(ScaleProps)));
//
//     let mut obj_type = BTreeMap::new();
//     obj_type.insert("aqua", SyntaxType::scale());
//     obj_type.insert("black", SyntaxType::scale());
//     obj_type.insert("blue", SyntaxType::scale());
//     obj_type.insert("fuchsia", SyntaxType::scale());
//     obj_type.insert("gray", SyntaxType::scale());
//     obj_type.insert("green", SyntaxType::scale());
//     obj_type.insert("lime", SyntaxType::scale());
//     obj_type.insert("maroon", SyntaxType::scale());
//     obj_type.insert("navy", SyntaxType::scale());
//     obj_type.insert("olive", SyntaxType::scale());
//     obj_type.insert("orange", SyntaxType::scale());
//     obj_type.insert("purple", SyntaxType::scale());
//     obj_type.insert("red", SyntaxType::scale());
//     obj_type.insert("silver", SyntaxType::scale());
//     obj_type.insert("teal", SyntaxType::scale());
//     obj_type.insert("white", SyntaxType::scale());
//     obj_type.insert("yellow", SyntaxType::scale());
//     let syntax_type = SyntaxType::Object(Rc::new(obj_type));
//
//     VarResult::new(value, syntax_type, VAR_NAME)
// }
//
// #[cfg(test)]
// mod tests {
//     use super::*;
//     use crate::runtime::{AnySeries, NoneCallback};
//     use crate::{LibInfo, PineParser, PineRunner,VarIndex};
//
//     #[test]
//     fn plot_fields_test() {
//         use crate::ast::stat_expr_types::VarIndex;
//         use crate::runtime::VarOperate;
//         use crate::types::{downcast_pf, Tuple};
//
//         let lib_info = LibInfo::new(
//             vec![declare_var()],
//             vec![("close", SyntaxType::float_series())],
//         );
//         let src = r"m = [
//             scale.aqua, scale.black, scale.blue, scale.fuchsia, scale.gray, scale.green,
//             scale.lime, scale.maroon, scale.navy, scale.olive, scale.orange, scale.purple,
//             scale.red, scale.silver, scale.teal, scale.white, scale.yellow
//         ]";
//
//         let blk = PineParser::new(src, &lib_info).parse_blk().unwrap();
//         let mut runner = PineRunner::new(&lib_info, &blk, &NoneCallback());
//
//         runner
//             .run(
//                 &vec![("close", AnySeries::from_float_vec(vec![Some(1f64)]))],
//                 None,
//             )
//             .unwrap();
//         let tuple_res =
//             downcast_pf::<Tuple>(runner.get_context().move_var(VarIndex::new(0, 0)).unwrap());
//         let tuple_vec = tuple_res.unwrap().into_inner().0;
//         assert_eq!(
//             tuple_vec,
//             vec![
//                 PineRef::new_box(Scale("#00BCD4")),
//                 PineRef::new_box(Scale("#363A45")),
//                 PineRef::new_box(Scale("#2196F3")),
//                 PineRef::new_box(Scale("#E040FB")),
//                 PineRef::new_box(Scale("#787B86")),
//                 PineRef::new_box(Scale("#4CAF50")),
//                 PineRef::new_box(Scale("#00E676")),
//                 PineRef::new_box(Scale("#880E4F")),
//                 PineRef::new_box(Scale("#311B92")),
//                 PineRef::new_box(Scale("#808000")),
//                 PineRef::new_box(Scale("#FF9800")),
//                 PineRef::new_box(Scale("#9C27B0")),
//                 PineRef::new_box(Scale("#FF5252")),
//                 PineRef::new_box(Scale("#B2B5BE")),
//                 PineRef::new_box(Scale("#00897B")),
//                 PineRef::new_box(Scale("#FFFFFF")),
//                 PineRef::new_box(Scale("#FFEB3B")),
//             ]
//         );
//     }
// }
