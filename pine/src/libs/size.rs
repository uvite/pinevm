use super::VarResult;
use crate::ast::syntax_type::SyntaxType;
use crate::helper::err_msgs::*;
use crate::helper::str_replace;
use crate::runtime::context::{Ctx};

use crate::types::{
    Object,
    PineClass, PineRef, RuntimeErr,
};
use std::collections::BTreeMap;
use std::rc::Rc;

struct PlotProps;

impl<'a> PineClass<'a> for PlotProps {
    fn custom_type(&self) -> &str {
        "size"
    }

    fn get(&self, _ctx: &mut dyn Ctx<'a>, name: &str) -> Result<PineRef<'a>, RuntimeErr> {
        match name {
            "auto" => Ok(PineRef::new(String::from("auto"))),
            "huge" => Ok(PineRef::new(String::from("huge"))),
            "large" => Ok(PineRef::new(String::from("large"))),
            "normal" => Ok(PineRef::new(String::from("normal"))),
            "small" => Ok(PineRef::new(String::from("small"))),
            "tiny" => Ok(PineRef::new(String::from("tiny"))),
            _ => Err(RuntimeErr::NotImplement(str_replace(
                NO_FIELD_IN_OBJECT,
                vec![String::from(name), String::from("plot")],
            ))),
        }
    }

    fn copy(&self) -> Box<dyn PineClass<'a> + 'a> {
        Box::new(PlotProps)
    }
}

pub const VAR_NAME: &'static str = "size";

pub fn declare_var<'a>() -> VarResult<'a> {
    let value = PineRef::new(Object::new(Box::new(PlotProps)));

    let mut obj_type = BTreeMap::new();
    obj_type.insert("auto", SyntaxType::string());
    obj_type.insert("huge", SyntaxType::string());
    obj_type.insert("large", SyntaxType::string());
    obj_type.insert("normal", SyntaxType::string());
    obj_type.insert("small", SyntaxType::string());
    obj_type.insert("tiny", SyntaxType::string());
    let syntax_type = SyntaxType::Object(Rc::new(obj_type));
    VarResult::new(value, syntax_type, VAR_NAME)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::runtime::{AnySeries, NoneCallback};
    use crate::{LibInfo, PineParser, PineRunner,VarIndex};
    #[test]
    fn display_fields_test() {
        use crate::ast::stat_expr_types::VarIndex;
        use crate::types::{downcast_pf, Tuple};

        let lib_info = LibInfo::new(
            vec![declare_var()],
            vec![("close", SyntaxType::float_series())],
        );
        let src = r"m = [
            size.auto, size.huge, size.large, size.normal, size.small, size.tiny
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
                PineRef::new(String::from("auto")),
                PineRef::new(String::from("huge")),
                PineRef::new(String::from("large")),
                PineRef::new(String::from("normal")),
                PineRef::new(String::from("small")),
                PineRef::new(String::from("tiny")),
            ]
        );
    }
}
