use super::plot::plot_color;
use super::VarResult;
use crate::ast::syntax_type::{FunctionType, FunctionTypes, SimpleSyntaxType, SyntaxType};


use crate::helper::{
    move_element, pine_ref_to_bool, pine_ref_to_color, pine_ref_to_i64,
    pine_ref_to_string, require_param,
};
use crate::runtime::context::{downcast_ctx, Ctx};
use crate::runtime::output::{FillInfo, OutputData, OutputInfo};
use crate::types::{
    Callable, CallableFactory, ParamCollectCall, PineRef, PineType, RuntimeErr, SeriesCall, NA,
};

use std::rc::Rc;

fn pine_plot<'a>(
    context: &mut dyn Ctx<'a>,
    mut param: Vec<Option<PineRef<'a>>>,
    _func_type: FunctionType<'a>,
) -> Result<(), RuntimeErr> {
    move_tuplet!((_plot1, _plot2, color) = param);
    match (_func_type.get_type(2), color) {
        (Some(SyntaxType::Series(_)), Some(color)) => {
            let color = plot_color(color, context)?;
            downcast_ctx(context)
                .push_output_data(Some(OutputData::new_with_sc(vec![], vec![color])));
            Ok(())
        }
        _ => {
            downcast_ctx(context).push_output_data(None);
            Ok(())
        }
    }
}

#[derive(Debug, Clone)]
struct PlotVal {
    start_id: i64,
    end_id: i64,
    output_id: i32,
}

impl PlotVal {
    fn new() -> PlotVal {
        PlotVal {
            start_id: -1i64,
            end_id: -1i64,
            output_id: -1,
        }
    }
}

impl<'a> SeriesCall<'a> for PlotVal {
    fn step(
        &mut self,
        context: &mut dyn Ctx<'a>,
        mut p: Vec<Option<PineRef<'a>>>,
        _func_type: FunctionType<'a>,
    ) -> Result<PineRef<'a>, RuntimeErr> {
        if self.output_id < 0 && !downcast_ctx(context).check_is_output_info_ready() {
            move_tuplet!((plot1, plot2, color, opacity, title, editable, show_last) = p);
            let names = match _func_type.get_type(0) {
                Some(&SyntaxType::ObjectClass("plot")) => ("plot", "plot1", "plot2"),
                Some(&SyntaxType::ObjectClass("hline")) => ("hline", "hline1", "hline2"),
                _ => unreachable!(),
            };
            let plot_info = FillInfo {
                fill_type: String::from(names.0),
                start: require_param(names.1, pine_ref_to_i64(plot1))?,
                end: require_param(names.2, pine_ref_to_i64(plot2))?,
                title: pine_ref_to_string(title),
                color: pine_ref_to_color(color),
                opacity: pine_ref_to_i64(opacity),
                editable: pine_ref_to_bool(editable),
                show_last: pine_ref_to_i64(show_last),
            };
            self.output_id =
                downcast_ctx(context).push_output_info_retindex(OutputInfo::Fill(plot_info));
        }
        // Ok(PineRef::Box(Box::new(Some(self.output_id as i64))))
        Ok(PineRef::new_box(NA))
    }

    fn run_with_cd(
        &mut self,
        _context: &mut dyn Ctx<'a>,
        params: Vec<Option<PineRef<'a>>>,
        func_type: FunctionType<'a>,
    ) -> Result<(), RuntimeErr> {
        pine_plot(_context, params, func_type)
    }

    fn copy(&self) -> Box<dyn SeriesCall<'a> + 'a> {
        Box::new(self.clone())
    }
}

pub const VAR_NAME: &'static str = "fill";

pub fn declare_var<'a>() -> VarResult<'a> {
    let value = PineRef::new(CallableFactory::new(|| {
        Callable::new(
            None,
            Some(Box::new(ParamCollectCall::new_with_caller(Box::new(
                PlotVal::new(),
            )))),
        )
    }));

    let func_type = FunctionTypes(vec![
        FunctionType::new((
            vec![
                ("plot1", SyntaxType::ObjectClass("plot")),
                ("plot2", SyntaxType::ObjectClass("plot")),
                ("color", SyntaxType::Simple(SimpleSyntaxType::Color)),
                ("opacity", SyntaxType::int()),
                ("title", SyntaxType::string()),
                ("editable", SyntaxType::bool()),
                ("show_last", SyntaxType::int()),
            ],
            SyntaxType::Void,
        )),
        FunctionType::new((
            vec![
                ("plot1", SyntaxType::ObjectClass("plot")),
                ("plot2", SyntaxType::ObjectClass("plot")),
                ("color", SyntaxType::Series(SimpleSyntaxType::Color)),
                ("opacity", SyntaxType::int()),
                ("title", SyntaxType::string()),
                ("editable", SyntaxType::bool()),
                ("show_last", SyntaxType::int()),
            ],
            SyntaxType::Void,
        )),
        FunctionType::new((
            vec![
                ("hline1", SyntaxType::ObjectClass("hline")),
                ("hline2", SyntaxType::ObjectClass("hline")),
                ("color", SyntaxType::Simple(SimpleSyntaxType::Color)),
                ("opacity", SyntaxType::int()),
                ("title", SyntaxType::string()),
                ("editable", SyntaxType::bool()),
                ("show_last", SyntaxType::int()),
            ],
            SyntaxType::Void,
        )),
        FunctionType::new((
            vec![
                ("hline1", SyntaxType::ObjectClass("hline")),
                ("hline2", SyntaxType::ObjectClass("hline")),
                ("color", SyntaxType::Series(SimpleSyntaxType::Color)),
                ("opacity", SyntaxType::int()),
                ("title", SyntaxType::string()),
                ("editable", SyntaxType::bool()),
                ("show_last", SyntaxType::int()),
            ],
            SyntaxType::Void,
        )),
    ]);
    let syntax_type = SyntaxType::Function(Rc::new(func_type));
    VarResult::new(value, syntax_type, VAR_NAME)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::runtime::{AnySeries, NoneCallback};
    use crate::{LibInfo, PineParser, PineRunner};

    #[test]
    fn fill_info_test() {
        use super::super::plot;
        use crate::runtime::OutputInfo;

        let lib_info = LibInfo::new(
            vec![declare_var(), plot::declare_var()],
            vec![("close", SyntaxType::Series(SimpleSyntaxType::Float))],
        );
        let src = "p1 = plot(close)\np01 = plot(close)\np2 = plot(close)\n\
        fill(p1, p2, #111111, 1, 'fill', true, 1)\n\
        fill(p1, p2, close > close ? #111111 : #222222)";
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
            &runner.get_io_info().get_outputs()[3],
            &OutputInfo::Fill(FillInfo {
                fill_type: String::from("plot"),
                start: 0i64,
                end: 2i64,
                title: Some(String::from("fill")),
                color: Some(String::from("#111111")),
                opacity: Some(1i64),
                editable: Some(true),
                show_last: Some(1i64),
            })
        );

        let output_data = runner.move_output_data();
        assert_eq!(output_data[3], None);
        assert_eq!(
            output_data[4],
            Some(OutputData::new_with_sc(
                vec![],
                vec![StrOptionsData {
                    options: vec![String::from("#222222")],
                    values: vec![Some(0), Some(0)]
                }]
            ))
        );
    }

    #[test]
    fn fill_hline_test() {
        use super::super::hline;
        use crate::runtime::OutputInfo;

        let lib_info = LibInfo::new(
            vec![declare_var(), hline::declare_var()],
            vec![("close", SyntaxType::Series(SimpleSyntaxType::Float))],
        );
        let src = "p1 = hline(1.0)\np2 = hline(2.0)\n
        fill(p1, p2, #111111, 1, 'fill', true, 1)";
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
            &runner.get_io_info().get_outputs()[2],
            &OutputInfo::Fill(FillInfo {
                fill_type: String::from("hline"),
                start: 0i64,
                end: 1i64,
                title: Some(String::from("fill")),
                color: Some(String::from("#111111")),
                opacity: Some(1i64),
                editable: Some(true),
                show_last: Some(1i64),
            })
        );
    }
}
