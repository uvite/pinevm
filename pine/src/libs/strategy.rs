use std::rc::Rc;

use crate::ast::syntax_type::{FunctionType, FunctionTypes, SyntaxType};
use crate::helper::{move_element, pine_ref_to_bool, pine_ref_to_i64, pine_ref_to_string};
use crate::helper::err_msgs::*;
use crate::helper::str_replace;
use crate::runtime::{ScriptPurpose, StrategyScript};
use crate::runtime::context::{Ctx, downcast_ctx};
use crate::types::{Callable, CallableFactory, NA, PineRef, RuntimeErr};

use super::VarResult;

/**
title (const string) 脚本标题。当没有使用`shorttitle`参数时，它会显示在图表上，并在发布脚本时成为出版物的默认标题。
shorttitle (const string) 脚本在图表上的显示名称。如果指定，它将替换大多数图表相关窗口中的`title`参数。可选。默认值是用于`title`的参数。
overlay (const bool) 如果true，指标将显示在图表上方。如果false，它将被添加到单独的窗格中。可选。默认值为false。
format (const string) 指定脚本显示值的格式。可能的值：format.inherit、format.price、format.volume。可选。默认值为format.inherit。
precision (const int) 指定脚本显示值的浮点数之后的位数。必须是不大于16的非负整数。如果`format`设置为format.inherit并指定了`precision`，则格式将改为设置为format.price。可选。默认值继承自图表商品的精度。



strategy(title, shorttitle, overlay, format, precision, scale,
pyramiding, calc_on_order_fills,
calc_on_every_tick, max_bars_back,
backtest_fill_limits_assumption,
default_qty_type, default_qty_value,
initial_capital,
currency, slippage, commission_type, commission_value,
process_orders_on_close, close_entries_rule, margin_long,
margin_short, explicit_plot_zorder, max_lines_count,
max_labels_count, max_boxes_count, risk_free_rate, use_bar_magnifier)
 **/


fn strategy<'a>(
    context: &mut dyn Ctx<'a>,
    mut param: Vec<Option<PineRef<'a>>>,
    _func_type: FunctionType<'a>,
) -> Result<PineRef<'a>, RuntimeErr> {
    move_tuplet!((title, shorttitle, overlay, format, precision, scale, pyramiding, calc_on_order_fills, calc_on_every_tick, max_bars_back, backtest_fill_limits_assumption, default_qty_type, default_qty_value, initial_capital, currency, slippage, commission_type, commission_value, process_orders_on_close, close_entries_rule, margin_long, margin_short, explicit_plot_zorder, max_lines_count, max_labels_count, max_boxes_count, risk_free_rate, use_bar_magnifier) = param);
    println!("{:?}", param);
    if !downcast_ctx(context).check_is_input_info_ready() {
        if let Some(title) = pine_ref_to_string(title) {
            let strategy = StrategyScript {
                title,
                shorttitle: pine_ref_to_string(shorttitle),
                overlay: pine_ref_to_bool(overlay),
                format: pine_ref_to_string(format),
                precision: pine_ref_to_i64(precision),
                scale: pine_ref_to_string(scale),
                pyramiding: pine_ref_to_i64(pyramiding),
                calc_on_order_fills: pine_ref_to_bool(calc_on_order_fills),
                calc_on_every_tick: pine_ref_to_bool(calc_on_every_tick),
                max_bars_back: pine_ref_to_i64(max_bars_back),
                backtest_fill_limits_assumption: pine_ref_to_i64(backtest_fill_limits_assumption),
                default_qty_type: pine_ref_to_string(default_qty_type),
                default_qty_value: pine_ref_to_i64(default_qty_value),
                initial_capital: pine_ref_to_i64(initial_capital),
                currency: pine_ref_to_string(currency),
                slippage: pine_ref_to_i64(slippage),
                commission_type: pine_ref_to_string(commission_type),
                commission_value: pine_ref_to_i64(commission_value),
                process_orders_on_close: pine_ref_to_bool(process_orders_on_close),
                close_entries_rule: pine_ref_to_string(close_entries_rule),
                max_lines_count: pine_ref_to_i64(max_lines_count),
                max_labels_count: pine_ref_to_i64(max_labels_count),
                max_boxes_count: pine_ref_to_i64(max_boxes_count),
                margin_long: pine_ref_to_i64(margin_long),
                margin_short: pine_ref_to_i64(margin_short),
                explicit_plot_zorder: pine_ref_to_bool(explicit_plot_zorder),
                risk_free_rate: pine_ref_to_i64(risk_free_rate),
                use_bar_magnifier: pine_ref_to_bool(use_bar_magnifier),
            };
            downcast_ctx(context).set_script_type(ScriptPurpose::Strategy(strategy));
        } else {
            return Err(RuntimeErr::MissingParameters(str_replace(
                REQUIRED_PARAMETERS,
                vec![String::from("title")],
            )));
        }
    }
    Ok(PineRef::new(NA))
}

pub const VAR_NAME: &'static str = "strategy";

pub fn declare_var<'a>() -> VarResult<'a> {
    let value = PineRef::new(CallableFactory::new(|| Callable::new(Some(strategy), None)));


    let func_type = FunctionTypes(vec![FunctionType::new((
        vec![
            ("title", SyntaxType::string()),
            ("shorttitle", SyntaxType::string()),
            ("overlay", SyntaxType::bool()),
            ("format", SyntaxType::string()),
            ("precision", SyntaxType::int()),
            ("scale", SyntaxType::string()),
            ("pyramiding", SyntaxType::int()),
            ("calc_on_order_fills", SyntaxType::bool()),
            ("calc_on_every_tick", SyntaxType::bool()),
            ("max_bars_back", SyntaxType::int()),
            ("backtest_fill_limits_assumption", SyntaxType::int()),
            ("default_qty_type", SyntaxType::string()),
            ("default_qty_value", SyntaxType::int()),
            ("initial_capital", SyntaxType::int()),
            ("currency", SyntaxType::string()),
            ("slippage", SyntaxType::int()),
            ("commission_type", SyntaxType::string()),
            ("commission_value", SyntaxType::int()),
            ("process_orders_on_close", SyntaxType::bool()),
            ("close_entries_rule", SyntaxType::string()),
            ("margin_long", SyntaxType::int()),
            ("margin_short", SyntaxType::int()),
            ("explicit_plot_zorder", SyntaxType::bool()),
            ("max_lines_count", SyntaxType::int()),
            ("max_labels_count", SyntaxType::int()),
            ("max_boxes_count", SyntaxType::int()),
            ("risk_free_rate", SyntaxType::int()),
            ("use_bar_magnifier", SyntaxType::bool()),
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
        let src = r"strategy(title='title')";
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
            &Some(ScriptPurpose::Strategy(StrategyScript {
                title: String::from("title"),

                shorttitle: None,
                overlay: None,
                format: None,
                precision: None,
                scale: None,
                pyramiding: None,
                calc_on_order_fills: None,
                calc_on_every_tick: None,
                max_bars_back: None,
                backtest_fill_limits_assumption: None,
                default_qty_type: None,
                initial_capital: None,
                currency: None,
                slippage: None,
                commission_type: None,
                commission_value: None,
                process_orders_on_close: None,
                close_entries_rule: None,
                max_lines_count: None,
                max_labels_count: None,
                max_boxes_count: None,
                margin_long: None,
                margin_short: None,
                explicit_plot_zorder: None,
                risk_free_rate: None,
                use_bar_magnifier: None,
            }))
        );
    }
}
