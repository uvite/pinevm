extern crate pine;
use pine::ast::syntax_type::{SimpleSyntaxType, SyntaxType};
use pine::libs::declare_vars;
use pine::libs::plot;
use pine::libs::print;
use pine::runtime::data_src::{Callback, DataSrc, NoneCallback};
use pine::runtime::AnySeries;


const VI_SCRIPTS: &'static str = r#"
study(title = "Vortex Indicator", shorttitle="VI", format=format.price, precision=4)
period_ = input(14, title="Period", minval=2)

VMP = sum( abs( high - low[1]), period_ )
VMM = sum( abs( low - high[1]), period_ )
STR = sum( atr(1), period_ )
VIP = VMP / STR
VIM = VMM / STR

plot(VIP, title="VI +", color=#3BB3E4)
plot(VIM, title="VI -", color=#FF006E)
"#;

fn main() {
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


