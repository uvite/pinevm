use pine::{libs,PineParser,LibInfo,};
use pine::runtime::data_src::{Callback, DataSrc,};
use pine::runtime::any_series::{AnySeriesType, AnySeries,};
use pine::types::Float;
use pine::ast::syntax_type::SyntaxType;
use pyo3::exceptions;
use pyo3::prelude::*;
use pyo3::types::PyList;
use pyo3::wrap_pyfunction;
use std::collections::HashMap;

struct PyCallbackObj<'p> {
    pyobj: PyObject,
    py: Python<'p>,
}

impl<'p> Callback for PyCallbackObj<'p> {
    fn print(&self, _str: String) {
        let print_method = self.pyobj.getattr(self.py, "print");
        let result = print_method.unwrap().call(self.py, (_str,), None).unwrap();
    }

    fn plot(&self, floats: Vec<f64>) {
        let plot_method = self.pyobj.getattr(self.py, "plot");
        let result = plot_method.unwrap().call(self.py, (floats,), None).unwrap();
    }
}

#[pyfunction]
/// Formats the sum of two numbers as string
fn load_script(py: Python, script: String, close: Vec<f64>, callbacks: PyObject) -> PyResult<()> {
    //let mut script_block = pine::parse_all(&script).unwrap();
    let lib_info = LibInfo::new(
        libs::declare_vars(),
        vec![("close", SyntaxType::float_series())],
    );
    let blk = PineParser::new(&script, &lib_info).parse_blk().unwrap();


    let inner_vars = libs::declare_vars();

    let callback = PyCallbackObj {
        pyobj: callbacks,
        py,
    };
   // lib_vars=lib_info.lib_vars.clone();
    let mut datasrc = DataSrc::new(
        &blk,
        vec![],
        vec![("close", AnySeriesType::Float)],
        &callback,
    );

    let close_val: Vec<Float> = close.into_iter().map(|s| Some(s)).collect();
    let data = vec![(
        "close",
        AnySeries::from_float_vec(close_val),
    )];

    match datasrc.run(&data,None) {
        Ok(_) => Ok(()),
        Err(err) => Err(PyErr::new::<exceptions::TypeError, _>(format!(
            "Err {:?}",
            err
        ))),
    }
   // assert_eq!(datasrc.run(&data, None), Ok(()));
    //
    // let inner_vars = libs::declare_vars();
    //
    // let callback = PyCallbackObj {
    //     pyobj: callbacks,
    //     py,
    // };
    // let mut datasrc = DataSrc::new(&mut script_block, inner_vars, &callback);
    //
    // let mut data = HashMap::new();
    //
    // let close_val: Vec<Float> = close.into_iter().map(|s| Some(s)).collect();
    // data.insert("close", close_val);
    //
    // match datasrc.run(data) {
    //     Ok(_) => Ok(()),
    //     Err(err) => Err(PyErr::new::<exceptions::TypeError, _>(format!(
    //         "Err {:?}",
    //         err
    //     ))),
    // }
}

/// This module is a python module implemented in Rust.
#[pymodule]
fn pine_py(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(load_script))?;

    Ok(())
}
