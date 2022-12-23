use super::VarResult;

use crate::ast::syntax_type::{FunctionType, FunctionTypes, SimpleSyntaxType, SyntaxType};
use crate::helper::err_msgs::*;
use crate::helper::str_replace;
use crate::helper::{
    move_element, pine_ref_to_bool, pine_ref_to_f64, pine_ref_to_i64, pine_ref_to_string,
};
use crate::runtime::context::{downcast_ctx, Ctx};
use crate::runtime::output::{
    BoolInputInfo, FloatInputInfo, InputInfo, InputVal, IntInputInfo, SourceInputInfo,
    StringInputInfo,
};
use crate::types::{
    downcast_pf, Callable, PineClass, PineRef, RuntimeErr, SeriesCall, SimpleCallableObject, Tuple,
};
use std::cell::RefCell;
use std::collections::btree_map::BTreeMap;
use std::rc::Rc;


const BOOL_TYPE_STR: &'static str = "bool";
const INT_TYPE_STR: &'static str = "int";
const FLOAT_TYPE_STR: &'static str = "float";
const STRING_TYPE_STR: &'static str = "string";
const SOURCE_TYPE_STR: &'static str = "source";

#[derive(Debug, PartialEq, Clone)]
struct InputCall<'a> {
    val: RefCell<Option<PineRef<'a>>>,
}

impl<'a> InputCall<'a> {
    pub fn new() -> InputCall<'a> {
        InputCall {
            val: RefCell::new(None),
        }
    }
}

impl<'a> SeriesCall<'a> for InputCall<'a> {
    fn step(
        &mut self,
        context: &mut dyn Ctx<'a>,
        val: Vec<Option<PineRef<'a>>>,
        func_type: FunctionType<'a>,
    ) -> Result<PineRef<'a>, RuntimeErr> {
        if let Some(val) = &*self.val.borrow() {
            return Ok(val.copy_inner());
        }
        match pine_input(context, val, func_type) {
            Err(e) => Err(e),
            Ok(res) => {
                self.val.replace(Some(res.clone()));
                Ok(res.copy_inner())
            }
        }
    }

    fn run(&mut self, _context: &mut dyn Ctx<'a>) -> Result<(), RuntimeErr> {
        self.val.replace(None);
        Ok(())
    }

    fn copy(&self) -> Box<dyn SeriesCall<'a> + 'a> {
        Box::new(self.clone())
    }
}



fn input_for_bool<'a>(
    context: &mut dyn Ctx<'a>,
    mut param: Vec<Option<PineRef<'a>>>,
) -> Result<PineRef<'a>, RuntimeErr> {

    let ctx_ins = downcast_ctx(context);
    if !ctx_ins.check_is_input_info_ready() {

        ctx_ins.push_input_info(InputInfo::Bool(BoolInputInfo {
            defval: pine_ref_to_bool(param[0].clone()),
            title: pine_ref_to_string(move_element(&mut param, 1)),
            tooltip: pine_ref_to_string(move_element(&mut param, 2)),
            inline: pine_ref_to_string(move_element(&mut param, 3)),
            group: pine_ref_to_string(move_element(&mut param, 4)),
            confirm: pine_ref_to_bool(move_element(&mut param, 5)),

        }));
    }
    println!("==={:?}",ctx_ins.get_inputs());

    let input_val = ctx_ins.copy_next_input();

    println!("====----{:?}",input_val);
    match input_val {
        Some(InputVal::Bool(val)) => Ok(PineRef::new_box(val)),
        _ => match move_element(&mut param, 0) {

            Some(val) => Ok(val),
            _ => Err(RuntimeErr::NotValidParam),
        },
    }

}


fn input_for_string<'a>(
    context: &mut dyn Ctx<'a>,
    mut param: Vec<Option<PineRef<'a>>>,
) -> Result<PineRef<'a>, RuntimeErr> {

    let ctx_ins = downcast_ctx(context);
    if !ctx_ins.check_is_input_info_ready() {
        let res=InputInfo::String(StringInputInfo {
            defval: pine_ref_to_string(param[0].clone()),
            title: pine_ref_to_string(move_element(&mut param, 1)),
            tooltip: pine_ref_to_string(move_element(&mut param, 2)),
            inline: pine_ref_to_string(move_element(&mut param, 3)),
            group: pine_ref_to_string(move_element(&mut param, 4)),
            confirm: pine_ref_to_bool(move_element(&mut param, 5)),
            options: pine_ref_to_str_list(move_element(&mut param, 6)),

        });
        println!("{:?}",res);
        ctx_ins.push_input_info(res);
    }
    println!("{:?}",ctx_ins.get_inputs());

    let input_val = ctx_ins.copy_next_input();

    println!("{:?}",ctx_ins.get_inputs());
    match input_val {
        Some(InputVal::String(val)) => Ok(PineRef::new_rc(val)),
        _ => match move_element(&mut param, 0) {

            Some(val) => Ok(val),
            _ => Err(RuntimeErr::NotValidParam),
        },
    }

}
//
// const SOURCES: &[&'static str] = &["close", "open", "high", "low"];
// fn get_name_from_source<'a>(
//     context: &mut dyn Ctx<'a>,
//     var: &Option<PineRef<'a>>,
// ) -> Result<Option<String>, RuntimeErr> {
//     if var.is_none() {
//         println!("2343243");
//         return Ok(None);
//     }
//
//     //todo 不知道是啥问题// val.as_ptr() == var.as_ref().unwrap().as_ptr()
//     let src = SOURCES
//         .iter()
//         .find(|name| match context.get_top_varname_index(name) {
//
//             Some(index) => match context.get_var(index) {
//                 Some(val) => { val==var.as_ref().unwrap() },
//                 _ => false,
//             },
//             _ => false,
//         })
//         .map(|&s| String::from(s));
//
//     if src.is_none() {
//         return Err(RuntimeErr::InvalidParameters(str_replace(
//             INPUT_SRCS,
//             vec![SOURCES.join(", ")],
//         )));
//     }
//
//     Ok(src)
// }
//
// fn get_source_from_name<'a>(context: &mut dyn Ctx<'a>, name: String) -> Option<PineRef<'a>> {
//     match context.get_top_varname_index(&name) {
//         Some(index) => match context.get_var(index) {
//             Some(val) => Some(PineRef::clone(val)),
//             _ => None,
//         },
//         _ => None,
//     }
// }

const SOURCES: &[&'static str] = &["close", "open", "high", "low"];
fn get_name_from_source<'a>(
    context: &mut dyn Ctx<'a>,
    var: &Option<PineRef<'a>>,
) -> Result<Option<String>, RuntimeErr> {
    if var.is_none() {
        return Ok(None);
    }
    let src = SOURCES
        .iter()
        .find(|name| match context.get_top_varname_index(name) {
            Some(index) => match context.get_var(index) {
                Some(val) => {
                    println!("{:?}----{:?}",val.as_ptr(),var.as_ref().unwrap().as_ptr());
                    val.as_ptr() == var.as_ref().unwrap().as_ptr()
                },
                _ => false,
            },
            _ => false,
        })
        .map(|&s| String::from(s));
    if src.is_none() {
        return Err(RuntimeErr::InvalidParameters(str_replace(
            INPUT_SRCS,
            vec![SOURCES.join(", ")],
        )));
    }
    Ok(src)
}

fn get_source_from_name<'a>(context: &mut dyn Ctx<'a>, name: String) -> Option<PineRef<'a>> {
    match context.get_top_varname_index(&name) {
        Some(index) => match context.get_var(index) {
            Some(val) => Some(PineRef::clone(val)),
            _ => None,
        },
        _ => None,
    }
}

fn input_for_source<'a>(
    context: &mut dyn Ctx<'a>,
    mut param: Vec<Option<PineRef<'a>>>,
) -> Result<PineRef<'a>, RuntimeErr> {
    if !downcast_ctx(context).check_is_input_info_ready() {

        let name = get_name_from_source(context.get_top_ctx(), &param[0])?;
        downcast_ctx(context).push_input_info(InputInfo::Source(SourceInputInfo {
            defval: name,
            title: pine_ref_to_string(move_element(&mut param, 1)),
            tooltip:pine_ref_to_string(move_element(&mut param, 2)),
            inline: pine_ref_to_string(move_element(&mut param, 3)),
            group:pine_ref_to_string(move_element(&mut param, 4)),
        }));
    }

    let input_val = downcast_ctx(context).copy_next_input();
    println!("Get input val {:?}", input_val);

    match input_val {
        Some(InputVal::String(val)) | Some(InputVal::Source(val)) => {
            match get_source_from_name(context, val) {
                Some(pine_val) => Ok(pine_val),
                None => Err(RuntimeErr::NotValidParam),
            }
        }
        _ => match move_element(&mut param, 0) {
            Some(val) => Ok(val),
            _ => Err(RuntimeErr::NotValidParam),
        },
    }
}




pub fn pine_ref_to_list<'a, T, F>(val: Option<PineRef<'a>>, f: F) -> Option<Vec<T>>
where
    F: Fn(Option<PineRef<'a>>) -> Option<T>,
{
    if val.is_none() {
        return None;
    }
    match downcast_pf::<Tuple>(val.unwrap()) {
        Ok(res) => {
            let tuple_val = res.into_inner();
            Some(tuple_val.0.into_iter().filter_map(|s| f(Some(s))).collect())
        }
        Err(_) => None,
    }
}

pub fn pine_ref_to_i64_list<'a>(val: Option<PineRef<'a>>) -> Option<Vec<i64>> {
    pine_ref_to_list(val, pine_ref_to_i64)
}

pub fn pine_ref_to_f64_list<'a>(val: Option<PineRef<'a>>) -> Option<Vec<f64>> {
    pine_ref_to_list(val, pine_ref_to_f64)
}

pub fn pine_ref_to_str_list<'a>(val: Option<PineRef<'a>>) -> Option<Vec<String>> {
    pine_ref_to_list(val, pine_ref_to_string)
}

fn input_for_int<'a>(
    context: &mut dyn Ctx<'a>,
    mut param: Vec<Option<PineRef<'a>>>,
) -> Result<PineRef<'a>, RuntimeErr> {
    // if pine_ref_to_string(param[2].clone()) == Some(String::from("float")) {
    //     return input_for_float(context, param);
    // }

    move_tuplet!(
        (defval, title, minval, maxval, step, tooltip,  inline, group, confirm,options) = param
    );

    //let type_str = pine_ref_to_i64(move_element(&mut param, 0));

    let ctx_ins = downcast_ctx(context);
    if !ctx_ins.check_is_input_info_ready() {

        let a = InputInfo::Int(IntInputInfo {
            defval: pine_ref_to_i64(defval.clone()),
            title: pine_ref_to_string(title),
            minval: pine_ref_to_i64(minval),
            maxval: pine_ref_to_i64(maxval),
           // confirm: pine_ref_to_bool(confirm),
            step: pine_ref_to_i64(step),
            options: pine_ref_to_i64_list(options),
            tooltip: pine_ref_to_string(tooltip),
            inline: pine_ref_to_string(inline),
            group: pine_ref_to_string(group),
            confirm: pine_ref_to_bool(confirm),

        });


        ctx_ins.push_input_info(a);
    }

    let input_val = ctx_ins.copy_next_input();


    match input_val {
        Some(InputVal::Int(val)) => Ok(PineRef::new_box(Some(val))),
        _ => match defval {
            Some(val) => Ok(val),
            _ => Err(RuntimeErr::NotValidParam),
        },
    }
}

fn input_for_float<'a>(
    context: &mut dyn Ctx<'a>,
    mut param: Vec<Option<PineRef<'a>>>,
) -> Result<PineRef<'a>, RuntimeErr> {

    move_tuplet!(
        (defval, title, minval, maxval, step, tooltip,  inline, group, confirm,options) = param
    );

    //let type_str = pine_ref_to_i64(move_element(&mut param, 0));

    let ctx_ins = downcast_ctx(context);
    if !ctx_ins.check_is_input_info_ready() {

        let a = InputInfo::Float(FloatInputInfo {
            defval: pine_ref_to_f64(defval.clone()),
            title: pine_ref_to_string(title),
            minval: pine_ref_to_f64(minval),
            maxval: pine_ref_to_f64(maxval),
            // confirm: pine_ref_to_bool(confirm),
            step: pine_ref_to_f64(step),
            options: pine_ref_to_f64_list(options),
            tooltip: pine_ref_to_string(tooltip),
            inline: pine_ref_to_string(inline),
            group: pine_ref_to_string(group),
            confirm: pine_ref_to_bool(confirm),

        });


        ctx_ins.push_input_info(a);
    }



    let input_val = ctx_ins.copy_next_input();
    match input_val {
        Some(InputVal::Float(val)) => Ok(PineRef::new_box(Some(val))),
        _ => match defval {
            Some(val) => Ok(val),
            _ => Err(RuntimeErr::NotValidParam),
        },
    }
}

fn gen_bool_type<'a>() -> FunctionType<'a> {
    FunctionType::new((
        vec![
            ("defval", SyntaxType::bool()),
            ("title", SyntaxType::string()),
            ("tooltip", SyntaxType::string()),
            ("inline", SyntaxType::string()),
            ("group", SyntaxType::string()),
            ("confirm", SyntaxType::bool()),
        ],
        SyntaxType::bool(),
    ))
}
fn gen_int_type<'a>() -> FunctionType<'a> {
    FunctionType::new((
        vec![
            ("defval", SyntaxType::int()),
            ("title", SyntaxType::string()),
            ("minval", SyntaxType::int()),
            ("maxval", SyntaxType::int()),
            ("step", SyntaxType::int()),
            ("tooltip", SyntaxType::string()),
            ("inline", SyntaxType::string()),
            ("group", SyntaxType::string()),
            ("confirm", SyntaxType::bool()),
            ("options", SyntaxType::List(SimpleSyntaxType::Int)),
        ],
        SyntaxType::int(),
    ))
}

fn gen_float_type<'a>() -> FunctionType<'a> {
    FunctionType::new((
        vec![
            ("defval", SyntaxType::float()),
            ("title", SyntaxType::string()),
            ("minval", SyntaxType::float()),
            ("maxval", SyntaxType::float()),
            ("step", SyntaxType::float()),
            ("tooltip", SyntaxType::string()),
            ("inline", SyntaxType::string()),
            ("group", SyntaxType::string()),
            ("confirm", SyntaxType::bool()),
            ("options", SyntaxType::List(SimpleSyntaxType::Float)),
        ],
        SyntaxType::float(),
    ))
}

fn gen_string_type<'a>() -> FunctionType<'a> {
    FunctionType::new((
        vec![
            ("defval", SyntaxType::string()),
            ("title", SyntaxType::string()),

            ("tooltip", SyntaxType::string()),
            ("inline", SyntaxType::string()),
            ("group", SyntaxType::string()),
            ("confirm", SyntaxType::bool()),
            ("options", SyntaxType::List(SimpleSyntaxType::String)),
        ],
        SyntaxType::string(),
    ))
}


fn gen_source_type<'a>() -> FunctionType<'a> {
    FunctionType::new((
        vec![
            ("defval", SyntaxType::Series(SimpleSyntaxType::Float)),
            ("title", SyntaxType::string()),
            ("tooltip", SyntaxType::string()),
            ("inline", SyntaxType::string()),
            ("group", SyntaxType::string()),
        ],
        SyntaxType::Series(SimpleSyntaxType::Float),
    ))
}

fn pine_input<'a>(
    context: &mut dyn Ctx<'a>,
    param: Vec<Option<PineRef<'a>>>,
    func_type: FunctionType<'a>,
) -> Result<PineRef<'a>, RuntimeErr> {
   // println!("{}", func_type.arg_names().len());
    if func_type.arg_names().len() == 6 {
        //println!("{:?}", param);

        input_for_bool(context, param)
    } else if func_type.arg_names().len() == 10 {

        if func_type == gen_int_type() {
            input_for_int(context, param)
        } else if func_type == gen_float_type() {
            input_for_float(context, param)
        } else {
            unreachable!();
        }
    } else if func_type.arg_names().len() == 7 {
        input_for_string(context, param)
    } else if func_type.arg_names().len() == 5 {
        input_for_source(context, param)
    } else {
        unreachable!();
    }
}

struct InputProps;

impl<'a> PineClass<'a> for InputProps {
    fn custom_type(&self) -> &str {
        "input"
    }

    fn get(&self, _ctx: &mut dyn Ctx<'a>, name: &str) -> Result<PineRef<'a>, RuntimeErr> {
        match name {
            "bool" => Ok(PineRef::new(Callable::new(Some(pine_input), None))),
            "float" => Ok(PineRef::new(Callable::new(Some(pine_input), None))),
            "int" => Ok(PineRef::new(Callable::new(Some(pine_input), None))),
            "string" => Ok(PineRef::new(Callable::new(Some(pine_input), None))),
            "source" => Ok(PineRef::new(Callable::new(Some(pine_input), None))),

            // "bool" => Ok(PineRef::new_rc(String::from(BOOL_TYPE_STR))),
            // "float" => Ok(PineRef::new_rc(String::from(FLOAT_TYPE_STR))),
            "integer" => Ok(PineRef::new_rc(String::from(INT_TYPE_STR))),
            "resolution" => Ok(PineRef::new_rc(String::from(STRING_TYPE_STR))),
            "session" => Ok(PineRef::new_rc(String::from(STRING_TYPE_STR))),
            //"source" => Ok(PineRef::new_rc(String::from(SOURCE_TYPE_STR))),
            //"string" => Ok(PineRef::new_rc(String::from(STRING_TYPE_STR))),
            "symbol" => Ok(PineRef::new_rc(String::from(STRING_TYPE_STR))),

            _ => Err(RuntimeErr::NotImplement(str_replace(
                NO_FIELD_IN_OBJECT,
                vec![String::from(name), String::from("input")],
            ))),
        }
    }

    fn copy(&self) -> Box<dyn PineClass<'a> + 'a> {
        Box::new(InputProps)
    }
}

pub const VAR_NAME: &'static str = "input";

pub fn declare_var<'a>() -> VarResult<'a> {
    let value = PineRef::new(SimpleCallableObject::new(Box::new(InputProps), || {
        Callable::new(None, Some(Box::new(InputCall::new())))
    }));

    let mut obj_type = BTreeMap::new();

    obj_type.insert(
        "int",
        SyntaxType::Function(Rc::new(FunctionTypes(vec![gen_int_type()]))),
    );
    obj_type.insert(
        "float",
        SyntaxType::Function(Rc::new(FunctionTypes(vec![gen_float_type()]))),
    );
    obj_type.insert(
        "bool",
        SyntaxType::Function(Rc::new(FunctionTypes(vec![gen_bool_type()]))),
    );

    obj_type.insert(
        "string",
        SyntaxType::Function(Rc::new(FunctionTypes(vec![gen_string_type()]))),
    );

    obj_type.insert(
        "source",
        SyntaxType::Function(Rc::new(FunctionTypes(vec![gen_source_type()]))),
    );


    obj_type.insert("resolution", SyntaxType::string());
    obj_type.insert("session", SyntaxType::string());
    obj_type.insert("symbol", SyntaxType::string());
    let syntax_type = SyntaxType::ObjectFunction(
        Rc::new(obj_type),
        Rc::new(FunctionTypes(vec![
            gen_int_type(),
            gen_float_type(),
            gen_bool_type(),
            gen_string_type(),
            gen_source_type(),
        ])),
    );
    VarResult::new(value, syntax_type, VAR_NAME)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::stat_expr_types::VarIndex;
    use crate::runtime::{AnySeries, NoneCallback, VarOperate};
    use crate::types::{PineRef, Series};
    use crate::{LibInfo, PineParser, PineRunner};

    #[test]
    fn input_type_test() {
        let lib_info = LibInfo::new(vec![declare_var()], vec![]);
        PineParser::new("input(true, 'title', 'bool', false)", &lib_info);
        PineParser::new(
            "input(defval = true, title = 'title', type = 'bool', confirm = false)",
            &lib_info,
        );
        PineParser::new(
            "input(true, 'hello', defval = true, title = 'title', type = 'bool', confirm = false)",
            &lib_info,
        );

        PineParser::new(
            "input(1, 'hello', type = 'int', confirm = false)",
            &lib_info,
        );

        PineParser::new(
            "input(1, 'hello', 'int', 1, 10, true, 1, [1, 2, 3])",
            &lib_info,
        );

        PineParser::new(
            "input(1, 'hello', 'float', 1, 10, true, 1, [1, 2, 3])",
            &lib_info,
        );

        PineParser::new("input(1, 'hello', 'int')", &lib_info);
    }

    #[test]
    fn bool_input_test<'a>() {
        let lib_info = LibInfo::new(
            vec![declare_var()],
            vec![("close", SyntaxType::Series(SimpleSyntaxType::Float))],
        );
        let src = "m = input.bool(true, title='title', tooltip='tooltip',inline='inline',group='group',confirm=false)";

        let blk = PineParser::new(src, &lib_info).parse_blk().unwrap();
        let mut runner = PineRunner::new(&lib_info, &blk, &NoneCallback());

        runner
            .run(
                &vec![("close", AnySeries::from_float_vec(vec![Some(1f64)]))],
                None,
            )
            .unwrap();
        assert_eq!(
            runner.get_context().move_var(VarIndex::new(0, 0)),
            Some(PineRef::new_box(true))
        );

        runner.change_inputs(vec![Some(InputVal::Bool(false))]);
        runner
            .run(
                &vec![("close", AnySeries::from_float_vec(vec![Some(1f64)]))],
                None,
            )
            .unwrap();
        assert_eq!(
            runner.get_context().move_var(VarIndex::new(0, 0)),
            Some(PineRef::new_box(false))
        );
        assert_eq!(
            runner.get_io_info().get_inputs(),
            &vec![InputInfo::Bool(BoolInputInfo {
                defval: Some(true),
                title: Some(String::from("title")),
                tooltip: Some(String::from("tooltip")),
                inline: Some(String::from("inline")),
                group: Some(String::from("group")),
                confirm: Some(false),
            })]
        )
    }

    #[test]
    fn int_input_test<'a>() {
        let lib_info = LibInfo::new(
            vec![declare_var()],
            vec![("close", SyntaxType::Series(SimpleSyntaxType::Float))],
        );
        let src = "m = input.int(1,title='ok',confirm=true)";

        let blk = PineParser::new(src, &lib_info).parse_blk().unwrap();
        let mut runner = PineRunner::new(&lib_info, &blk, &NoneCallback());

        runner
            .run(
                &vec![("close", AnySeries::from_float_vec(vec![Some(1f64)]))],
                None,
            )
            .unwrap();
        assert_eq!(
            runner.get_context().move_var(VarIndex::new(0, 0)),
            Some(PineRef::new_box(Some(1)))
        );
        //
         runner.change_inputs(vec![Some(InputVal::Int(4))]);
        runner
            .run(
                &vec![("close", AnySeries::from_float_vec(vec![Some(1f64)]))],
                None,
            )
            .unwrap();
        // assert_eq!(
        //     runner.get_context().move_var(VarIndex::new(0, 0)),
        //     Some(PineRef::new_box(Some(4)))
        // );
        println!("{:?}",runner.get_io_info().get_inputs());
        assert_eq!(
            runner.get_io_info().get_inputs(),
            &vec![InputInfo::Int(IntInputInfo {
                defval: Some(1),
                title: Some(String::from("ok")),

                confirm: Some(true),
                minval: None,
                maxval: None,
                step: None,
                options: None,
                tooltip: None,
                inline: None,
                group: None
            })]
        )
    }

    #[test]
    fn some_int_input_test<'a>() {
        let lib_info = LibInfo::new(
            vec![declare_var()],
            vec![("close", SyntaxType::Series(SimpleSyntaxType::Float))],
        );
        let src = "m = input.int(1, 'hello')";

        let blk = PineParser::new(src, &lib_info).parse_blk().unwrap();
        let mut runner = PineRunner::new(&lib_info, &blk, &NoneCallback());
        runner.change_inputs(vec![Some(InputVal::Int(4))]);
        runner
            .run(
                &vec![("close", AnySeries::from_float_vec(vec![Some(1f64)]))],
                None,
            )
            .unwrap();
        assert_eq!(
            runner.get_context().move_var(VarIndex::new(0, 0)),
            Some(PineRef::new_box(Some(4)))
        );
    }

    #[test]
    fn float_input_test<'a>() {
        let lib_info = LibInfo::new(
            vec![declare_var()],
            vec![("close", SyntaxType::Series(SimpleSyntaxType::Float))],
        );
        let src = "m = input.float(1.5, 'hello',confirm=false)";

        let blk = PineParser::new(src, &lib_info).parse_blk().unwrap();
        let mut runner = PineRunner::new(&lib_info, &blk, &NoneCallback());

        runner
            .run(
                &vec![("close", AnySeries::from_float_vec(vec![Some(1f64)]))],
                None,
            )
            .unwrap();
        assert_eq!(
            runner.get_context().move_var(VarIndex::new(0, 0)),
            Some(PineRef::new_box(Some(1.5f64)))
        );
    }

    #[test]
    fn string_input_test<'a>() {
        let lib_info = LibInfo::new(
            vec![declare_var()],
            vec![("close", SyntaxType::Series(SimpleSyntaxType::Float))],
        );
        let src = "m = input.string('true3', title='title')";

        let blk = PineParser::new(src, &lib_info).parse_blk().unwrap();
        let mut runner = PineRunner::new(&lib_info, &blk, &NoneCallback());

        runner
            .run(
                &vec![("close", AnySeries::from_float_vec(vec![Some(1f64)]))],
                None,
            )
            .unwrap();
        // assert_eq!(
        //     runner.get_context().move_var(VarIndex::new(0, 0)),
        //     Some(PineRef::new_box(true))
        // );

       // runner.change_inputs(vec![Some(InputVal::Bool(false))]);


        runner
            .run(
                &vec![("close", AnySeries::from_float_vec(vec![Some(1f64)]))],
                None,
            )
            .unwrap();
        assert_eq!(
            runner.get_context().move_var(VarIndex::new(0, 0)),
            Some(PineRef::new_rc(String::from("true3")))
        );

        assert_eq!(
            runner.get_io_info().get_inputs(),
            &vec![InputInfo::String(StringInputInfo {
                defval: Some(String::from("true3")),

                title: Some(String::from("title")),
                tooltip: None,
                inline: None,
                group: None,
                confirm: None,
                options: None
            })]
        )
    }


    #[test]
    fn source_input_test<'a>() {
        let lib_info = LibInfo::new(
            vec![declare_var()],
            vec![
                ("close", SyntaxType::Series(SimpleSyntaxType::Float)),
                ("open", SyntaxType::Series(SimpleSyntaxType::Float)),
            ],
        );
        let src = "m = input(close, 'hello', 'source')";

        let blk = PineParser::new(src, &lib_info).parse_blk().unwrap();
        let mut runner = PineRunner::new(&lib_info, &blk, &NoneCallback());

        runner
            .run(
                &vec![
                    ("close", AnySeries::from_float_vec(vec![Some(1f64)])),
                    ("open", AnySeries::from_float_vec(vec![Some(2f64)])),
                ],
                None,
            )
            .unwrap();
        assert_eq!(
            runner.get_context().move_var(VarIndex::new(0, 0)),
            Some(PineRef::new_rc(Series::from_vec(vec![Some(1f64)])))
        );

        runner.change_inputs(vec![Some(InputVal::String(String::from("open")))]);
        runner
            .run(
                &vec![("open", AnySeries::from_float_vec(vec![Some(10f64)]))],
                None,
            )
            .unwrap();
        assert_eq!(
            runner.get_context().move_var(VarIndex::new(0, 0)),
            Some(PineRef::new_rc(Series::from_vec(vec![Some(10f64)])))
        );
    }

    #[test]
    // fn input_fields_test() {
    //     use crate::types::Tuple;
    //
    //     let lib_info = LibInfo::new(
    //         vec![declare_var()],
    //         vec![("close", SyntaxType::float_series())],
    //     );
    //     let src = r"m = [
    //         input.bool, input.float, input.integer, input.resolution,
    //         input.session, input.source, input.string, input.symbol
    //     ]";
    //
    //     let blk = PineParser::new(src, &lib_info).parse_blk().unwrap();
    //     let mut runner = PineRunner::new(&lib_info, &blk, &NoneCallback());
    //
    //     runner
    //         .run(
    //             &vec![("close", AnySeries::from_float_vec(vec![Some(1f64)]))],
    //             None,
    //         )
    //         .unwrap();
    //     let tuple_res =
    //         downcast_pf::<Tuple>(runner.get_context().move_var(VarIndex::new(0, 0)).unwrap());
    //     assert_eq!(
    //         tuple_res.unwrap().into_inner(),
    //         Tuple(vec![
    //             PineRef::new_rc(String::from(BOOL_TYPE_STR)),
    //             PineRef::new_rc(String::from(FLOAT_TYPE_STR)),
    //             PineRef::new_rc(String::from(INT_TYPE_STR)),
    //             PineRef::new_rc(String::from(STRING_TYPE_STR)),
    //             PineRef::new_rc(String::from(STRING_TYPE_STR)),
    //             PineRef::new_rc(String::from(SOURCE_TYPE_STR)),
    //             PineRef::new_rc(String::from(STRING_TYPE_STR)),
    //             PineRef::new_rc(String::from(STRING_TYPE_STR)),
    //         ])
    //     );
    // }

    #[test]
    fn input_float_test() {
        let lib_info = LibInfo::new(
            vec![declare_var()],
            vec![("close", SyntaxType::float_series())],
        );
        let src = r#"band = input(1.0, title="Multiplier")"#;
        let blk = PineParser::new(src, &lib_info).parse_blk().unwrap();
        let mut runner = PineRunner::new(&lib_info, &blk, &NoneCallback());

        runner.change_inputs(vec![Some(InputVal::Float(2f64))]);
        runner
            .run(
                &vec![("close", AnySeries::from_float_vec(vec![Some(1f64)]))],
                None,
            )
            .unwrap();
        assert_eq!(
            runner.get_context().move_var(VarIndex::new(0, 0)),
            Some(PineRef::new_box(Some(2f64)))
        );
    }
}
