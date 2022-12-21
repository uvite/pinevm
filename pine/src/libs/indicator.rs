use super::VarResult;
use crate::ast::syntax_type::{FunctionType, FunctionTypes, SyntaxType};
use crate::helper::err_msgs::*;
use crate::helper::str_replace;
use crate::helper::{move_element, pine_ref_to_bool, pine_ref_to_i64, pine_ref_to_string};
use crate::runtime::context::{downcast_ctx, Ctx};
use crate::runtime::{ScriptPurpose, IndicatorScript};
use crate::types::{Callable, CallableFactory, PineRef, RuntimeErr, NA};
use std::rc::Rc;
/**
title (const string) 脚本标题。当没有使用`shorttitle`参数时，它会显示在图表上，并在发布脚本时成为出版物的默认标题。
shorttitle (const string) 脚本在图表上的显示名称。如果指定，它将替换大多数图表相关窗口中的`title`参数。可选。默认值是用于`title`的参数。
overlay (const bool) 如果true，指标将显示在图表上方。如果false，它将被添加到单独的窗格中。可选。默认值为false。
format (const string) 指定脚本显示值的格式。可能的值：format.inherit、format.price、format.volume。可选。默认值为format.inherit。
precision (const int) 指定脚本显示值的浮点数之后的位数。必须是不大于16的非负整数。如果`format`设置为format.inherit并指定了`precision`，则格式将改为设置为format.price。可选。默认值继承自图表商品的精度。



indicator(title, shorttitle, overlay, format, precision,
scale, max_bars_back, timeframe, timeframe_gaps,
explicit_plot_zorder, max_lines_count, max_labels_count, max_boxes_count)
**/


fn indicator<'a>(
    context: &mut dyn Ctx<'a>,
    mut param: Vec<Option<PineRef<'a>>>,
    _func_type: FunctionType<'a>,
) -> Result<PineRef<'a>, RuntimeErr> {
    move_tuplet!((title, shorttitle, overlay, format, precision,timeframe,timeframe_gaps) = param);
    if !downcast_ctx(context).check_is_input_info_ready() {
        if let Some(title) = pine_ref_to_string(title) {
            let indicator = IndicatorScript {
                title,
                shorttitle: pine_ref_to_string(shorttitle),
                overlay: pine_ref_to_bool(overlay),
                format: pine_ref_to_string(format),
                precision: pine_ref_to_i64(precision),
                timeframe: pine_ref_to_string(timeframe),
                timeframe_gaps: pine_ref_to_bool(timeframe_gaps),
            };
            downcast_ctx(context).set_script_type(ScriptPurpose::Indicator(indicator));
        } else {
            return Err(RuntimeErr::MissingParameters(str_replace(
                REQUIRED_PARAMETERS,
                vec![String::from("title")],
            )));
        }
    }
    Ok(PineRef::new(NA))
}

pub const VAR_NAME: &'static str = "indicator";

pub fn declare_var<'a>() -> VarResult<'a> {
    let value = PineRef::new(CallableFactory::new(|| Callable::new(Some(indicator), None)));

    let func_type = FunctionTypes(vec![FunctionType::new((
        vec![
            ("title", SyntaxType::string()),
            ("shorttitle", SyntaxType::string()),
            ("overlay", SyntaxType::bool()),
            ("format", SyntaxType::string()),
            ("precision", SyntaxType::int()),
            ("timeframe", SyntaxType::string()),
            ("timeframe_gaps", SyntaxType::bool()),

        ],
        SyntaxType::Void,
    ))]);
    let syntax_type = SyntaxType::Function(Rc::new(func_type));
    VarResult::new(value, syntax_type, VAR_NAME)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::syntax_type::SimpleSyntaxType;
    use crate::runtime::{AnySeries, NoneCallback};
    use crate::{LibInfo, PineParser, PineRunner};

    #[test]
    fn plotbar_info_test() {
        let lib_info = LibInfo::new(
            vec![declare_var()],
            vec![("close", SyntaxType::Series(SimpleSyntaxType::Float))],
        );
        let src = r"indicator('hello', 'dd', true, 'price', 2)";
        let blk = PineParser::new(src, &lib_info).parse_blk().unwrap();
        let mut runner = PineRunner::new(&lib_info, &blk, &NoneCallback());

        runner
            .run(
                &vec![(
                    "close",
                    AnySeries::from_float_vec(vec![Some(1f64), Some(2f64)]),
                )],
                None,
            )
            .unwrap();

        assert_eq!(
            runner.get_io_info().get_script_type(),
            &Some(ScriptPurpose::Indicator(IndicatorScript {
                title: String::from("hello"),
                shorttitle: Some(String::from("dd")),
                overlay: Some(true),
                format: Some(String::from("price")),
                precision: Some(2)
            }))
        );
    }
}
