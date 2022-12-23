use std::rc::Rc;

use crate::ast::syntax_type::{FunctionType, FunctionTypes, SyntaxType};
use crate::helper::{move_element, pine_ref_to_bool, pine_ref_to_i64, pine_ref_to_string};
use crate::helper::err_msgs::*;
use crate::helper::str_replace;
use crate::runtime::{IndicatorScript, ScriptPurpose};
use crate::runtime::context::{Ctx, downcast_ctx};
use crate::types::{Callable, CallableFactory, NA, PineRef, RuntimeErr};

use super::VarResult;
 
fn indicator<'a>(
    context: &mut dyn Ctx<'a>,
    mut param: Vec<Option<PineRef<'a>>>,
    _func_type: FunctionType<'a>,
) -> Result<PineRef<'a>, RuntimeErr> {
    move_tuplet!((title, shorttitle, overlay, format, precision, scale, max_bars_back, timeframe, timeframe_gaps, explicit_plot_zorder, max_lines_count, max_labels_count, max_boxes_count) = param);
    if !downcast_ctx(context).check_is_input_info_ready() {
        if let Some(title) = pine_ref_to_string(title) {
            let indicator = IndicatorScript {
                title,
                shorttitle: pine_ref_to_string(shorttitle),
                overlay: pine_ref_to_bool(overlay),
                format: pine_ref_to_string(format),
                precision: pine_ref_to_i64(precision),
                scale: pine_ref_to_string(scale),
                max_bars_back:pine_ref_to_i64(max_bars_back),
                timeframe: pine_ref_to_string(timeframe),
                timeframe_gaps: pine_ref_to_bool(timeframe_gaps),
                explicit_plot_zorder: pine_ref_to_bool(explicit_plot_zorder),
                max_lines_count: pine_ref_to_i64(max_lines_count),
                max_labels_count: pine_ref_to_i64(max_labels_count),
                max_boxes_count: pine_ref_to_i64(max_boxes_count),
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
            ("scale", SyntaxType::string()),
            ("max_bars_back", SyntaxType::int()),
            ("timeframe", SyntaxType::string()),
            ("timeframe_gaps", SyntaxType::bool()),
            ("explicit_plot_zorder", SyntaxType::bool()),
            ("max_lines_count", SyntaxType::int()),
            ("max_labels_count", SyntaxType::int()),
            ("max_boxes_count", SyntaxType::int()),
        ],
        SyntaxType::Void,
    ))]);

    let syntax_type = SyntaxType::Function(Rc::new(func_type));
    VarResult::new(value, syntax_type, VAR_NAME)
}

#[cfg(test)]
mod tests {
    use crate::{LibInfo, PineParser, PineRunner, VarIndex};
    use crate::ast::syntax_type::SimpleSyntaxType;
    use crate::runtime::{AnySeries, NoneCallback};

    use super::*;

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
                precision: Some(2),
                scale: None,
              
                explicit_plot_zorder: None,
                max_lines_count: None,
                max_labels_count: None,
                max_bars_back: None,
                timeframe: None,
                max_boxes_count: None,
                timeframe_gaps: None
            }))
        );
    }
}
