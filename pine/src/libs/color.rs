use std::collections::BTreeMap;
use std::rc::Rc;

use crate::ast::syntax_type::{FunctionType, FunctionTypes, SyntaxType};
use crate::helper::err_msgs::*;
use crate::helper::{pine_ref_to_color, pine_ref_to_color2, str_replace};

use crate::helper::{
    move_element, pine_ref_to_bool, pine_ref_to_f64, pine_ref_to_i64, pine_ref_to_string,
};
use crate::runtime::context::Ctx;
use crate::{downcast_ctx, InputVal, SimpleSyntaxType};
use crate::SyntaxType::Series;
use crate::types::{Callable, Color, Object, PineClass, PineRef, RefData, RuntimeErr};

use super::VarResult;

struct ColorProps;


fn input_for_new<'a>(
    context: &mut dyn Ctx<'a>,
    mut param: Vec<Option<PineRef<'a>>>,
) -> Result<PineRef<'a>, RuntimeErr> {
    move_tuplet!(
        (color, transp) = param
    );
    Ok(PineRef::new_box(Color("#00E676")))
}

fn input_for_r_g_b<'a>(
    context: &mut dyn Ctx<'a>,
    mut param: Vec<Option<PineRef<'a>>>,
) -> Result<PineRef<'a>, RuntimeErr> {
    println!("---------{:?}",param);
    Ok(PineRef::new_box(Color("#00E676")))
}

fn input_for_rgb<'a>(
    context: &mut dyn Ctx<'a>,
    mut param: Vec<Option<PineRef<'a>>>,
) -> Result<PineRef<'a>, RuntimeErr> {

    let ctx_ins = downcast_ctx(context);
    // if !ctx_ins.check_is_input_info_ready() {
    //     let res=InputInfo::String(StringInputInfo {
    //         defval: pine_ref_to_string(param[0].clone()),
    //         title: pine_ref_to_string(move_element(&mut param, 1)),
    //         tooltip: pine_ref_to_string(move_element(&mut param, 2)),
    //         inline: pine_ref_to_string(move_element(&mut param, 3)),
    //         group: pine_ref_to_string(move_element(&mut param, 4)),
    //         confirm: pine_ref_to_bool(move_element(&mut param, 5)),
    //         options: pine_ref_to_str_list(move_element(&mut param, 6)),
    //
    //     });
    //     println!("{:?}",res);
    //     ctx_ins.push_input_info(res);
    // }
    // println!("{:?}",ctx_ins.get_inputs());

    Ok(PineRef::new(Color("#123456")))



}

fn input_from_gradient<'a>(
    context: &mut dyn Ctx<'a>,
    mut param: Vec<Option<PineRef<'a>>>,
) -> Result<PineRef<'a>, RuntimeErr> {
    move_tuplet!(
        (value, bottom_value, top_value, bottom_color, top_color) = param
    );
    Ok(PineRef::new_box(Color("#00E676")))
}

fn color_input<'a>(
    context: &mut dyn Ctx<'a>,
    param: Vec<Option<PineRef<'a>>>,
    func_type: FunctionType<'a>,
) -> Result<PineRef<'a>, RuntimeErr> {
    println!("{}", func_type.arg_names().len());
    if func_type.arg_names().len() == 1 {
        input_for_r_g_b(context, param)
    } else if func_type.arg_names().len() == 2 {
        input_for_new(context, param)
    } else if func_type.arg_names().len() == 4 {
        input_for_rgb(context, param)
    } else if func_type.arg_names().len() == 5 {
        input_from_gradient(context, param)
    } else {
        unreachable!();
    }
}

fn gen_new_type<'a>() -> FunctionType<'a> {
    FunctionType::new((
        vec![
            ("color", SyntaxType::Simple(SimpleSyntaxType::Color)),
            ("transp", SyntaxType::int()),
        ],
        SyntaxType::string(),
    ))
}

fn gen_rgb_type<'a>() -> FunctionType<'a> {
    FunctionType::new((
        vec![
            ("red", SyntaxType::Simple(SimpleSyntaxType::Int)),
            ("green", SyntaxType::Simple(SimpleSyntaxType::Int)),
            ("blue", SyntaxType::Simple(SimpleSyntaxType::Int)),
            ("transp", SyntaxType::int()),
        ],
        SyntaxType::string(),
    ))
}

fn gen_from_gradient_type<'a>() -> FunctionType<'a> {
    FunctionType::new((
        vec![
            ("value", SyntaxType::Simple(SimpleSyntaxType::Int)),
            ("bottom_value", SyntaxType::Simple(SimpleSyntaxType::Int)),
            ("top_value", SyntaxType::Simple(SimpleSyntaxType::Int)),
            ("bottom_color", SyntaxType::Simple(SimpleSyntaxType::Int)),
            ("top_color", SyntaxType::Simple(SimpleSyntaxType::Int)),
        ],
        SyntaxType::string(),
    ))
}

fn gen_r_g_b_type<'a>() -> FunctionType<'a> {
    FunctionType::new((
        vec![
            ("color", SyntaxType::Simple(SimpleSyntaxType::Color)),
        ],
        SyntaxType::string(),
    ))
}

impl<'a> PineClass<'a> for ColorProps {
    fn custom_type(&self) -> &str {
        "color"
    }

    fn get(&self, _ctx: &mut dyn Ctx<'a>, name: &str) -> Result<PineRef<'a>, RuntimeErr> {
        match name {
            "new" => Ok(PineRef::new(Callable::new(Some(color_input), None))),
            "from_gradient" => Ok(PineRef::new(Callable::new(Some(color_input), None))),
            "rgb" => Ok(PineRef::new(Callable::new(Some(color_input), None))),
            "r" => Ok(PineRef::new(Callable::new(Some(color_input), None))),
            "g" => Ok(PineRef::new(Callable::new(Some(color_input), None))),
            "b" => Ok(PineRef::new(Callable::new(Some(color_input), None))),

            "aqua" => Ok(PineRef::new_box(Color("#00BCD4"))),
            "black" => Ok(PineRef::new_box(Color("#363A45"))),
            "blue" => Ok(PineRef::new_box(Color("#2196F3"))),
            "fuchsia" => Ok(PineRef::new_box(Color("#E040FB"))),
            "gray" => Ok(PineRef::new_box(Color("#787B86"))),
            "green" => Ok(PineRef::new_box(Color("#4CAF50"))),
            "lime" => Ok(PineRef::new_box(Color("#00E676"))),
            "maroon" => Ok(PineRef::new_box(Color("#880E4F"))),
            "navy" => Ok(PineRef::new_box(Color("#311B92"))),
            "olive" => Ok(PineRef::new_box(Color("#808000"))),
            "orange" => Ok(PineRef::new_box(Color("#FF9800"))),
            "purple" => Ok(PineRef::new_box(Color("#9C27B0"))),
            "red" => Ok(PineRef::new_box(Color("#FF5252"))),
            "silver" => Ok(PineRef::new_box(Color("#B2B5BE"))),
            "teal" => Ok(PineRef::new_box(Color("#00897B"))),
            "white" => Ok(PineRef::new_box(Color("#FFFFFF"))),
            "yellow" => Ok(PineRef::new_box(Color("#FFEB3B"))),
            _ => Err(RuntimeErr::NotImplement(str_replace(
                NO_FIELD_IN_OBJECT,
                vec![String::from(name), String::from("color")],
            ))),
        }
    }

    fn copy(&self) -> Box<dyn PineClass<'a> + 'a> {
        Box::new(ColorProps)
    }
}


pub const VAR_NAME: &'static str = "color";

pub fn declare_var<'a>() -> VarResult<'a> {
    let value = PineRef::new(Object::new(Box::new(ColorProps)));

    let mut obj_type = BTreeMap::new();
    obj_type.insert(
        "new",
        SyntaxType::Function(Rc::new(FunctionTypes(vec![gen_new_type()]))),
    );
    obj_type.insert(
        "from_gradient",
        SyntaxType::Function(Rc::new(FunctionTypes(vec![gen_from_gradient_type()]))),
    );
    obj_type.insert(
        "rgb",
        SyntaxType::Function(Rc::new(FunctionTypes(vec![gen_rgb_type()]))),
    );
    obj_type.insert(
        "r",
        SyntaxType::Function(Rc::new(FunctionTypes(vec![gen_r_g_b_type()]))),
    );
    obj_type.insert(
        "g",
        SyntaxType::Function(Rc::new(FunctionTypes(vec![gen_r_g_b_type()]))),
    );
    obj_type.insert(
        "b",
        SyntaxType::Function(Rc::new(FunctionTypes(vec![gen_r_g_b_type()]))),
    );

    obj_type.insert("aqua", SyntaxType::color());
    obj_type.insert("black", SyntaxType::color());
    obj_type.insert("blue", SyntaxType::color());
    obj_type.insert("fuchsia", SyntaxType::color());
    obj_type.insert("gray", SyntaxType::color());
    obj_type.insert("green", SyntaxType::color());
    obj_type.insert("lime", SyntaxType::color());
    obj_type.insert("maroon", SyntaxType::color());
    obj_type.insert("navy", SyntaxType::color());
    obj_type.insert("olive", SyntaxType::color());
    obj_type.insert("orange", SyntaxType::color());
    obj_type.insert("purple", SyntaxType::color());
    obj_type.insert("red", SyntaxType::color());
    obj_type.insert("silver", SyntaxType::color());
    obj_type.insert("teal", SyntaxType::color());
    obj_type.insert("white", SyntaxType::color());
    obj_type.insert("yellow", SyntaxType::color());
    let syntax_type = SyntaxType::Object(Rc::new(obj_type));

    VarResult::new(value, syntax_type, VAR_NAME)
}

#[cfg(test)]
mod tests {
    use crate::{LibInfo, PineParser, PineRunner, VarIndex};
    use crate::runtime::{AnySeries, NoneCallback};

    use super::*;

    #[test]
    fn plot_fields_test() {
        use crate::ast::stat_expr_types::VarIndex;
        use crate::runtime::VarOperate;
        use crate::types::{downcast_pf, Tuple};

        let lib_info = LibInfo::new(
            vec![declare_var()],
            vec![("close", SyntaxType::float_series())],
        );
        let src = r"m = [
            color.aqua, color.black, color.blue, color.fuchsia, color.gray, color.green,
            color.lime, color.maroon, color.navy, color.olive, color.orange, color.purple,
            color.red, color.silver, color.teal, color.white, color.yellow
        ]";

        let blk = PineParser::new(src, &lib_info).parse_blk().unwrap();
        let mut runner = PineRunner::new(&lib_info, &blk, &NoneCallback());

        runner
            .run(
                &vec![("close", AnySeries::from_float_vec(vec![Some(1f64)]))],
                None,
            )
            .unwrap();
        let tuple_res =
            downcast_pf::<Tuple>(runner.get_context().move_var(VarIndex::new(0, 0)).unwrap());
        let tuple_vec = tuple_res.unwrap().into_inner().0;
        assert_eq!(
            tuple_vec,
            vec![
                PineRef::new_box(Color("#00BCD4")),
                PineRef::new_box(Color("#363A45")),
                PineRef::new_box(Color("#2196F3")),
                PineRef::new_box(Color("#E040FB")),
                PineRef::new_box(Color("#787B86")),
                PineRef::new_box(Color("#4CAF50")),
                PineRef::new_box(Color("#00E676")),
                PineRef::new_box(Color("#880E4F")),
                PineRef::new_box(Color("#311B92")),
                PineRef::new_box(Color("#808000")),
                PineRef::new_box(Color("#FF9800")),
                PineRef::new_box(Color("#9C27B0")),
                PineRef::new_box(Color("#FF5252")),
                PineRef::new_box(Color("#B2B5BE")),
                PineRef::new_box(Color("#00897B")),
                PineRef::new_box(Color("#FFFFFF")),
                PineRef::new_box(Color("#FFEB3B")),
            ]
        );
    }
}
