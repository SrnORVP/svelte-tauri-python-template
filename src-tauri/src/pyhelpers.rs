use pyo3::prelude::*;
use pyo3::types::*;
use std::path::Path;

#[allow(dead_code)]
pub fn pylib_import(py: Python<'_>, pyvenv_path: &str) -> PyResult<()> {
    let pypath = Path::new(pyvenv_path);
    let syspath: &PyList = py.import("sys")?.getattr("path")?.downcast()?;
    syspath.insert(0, pypath)
}

#[allow(dead_code)]
pub fn pysrc_main() -> &'static str {
    let pysrc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/py-app/main.py"));
    pysrc
}

#[allow(dead_code)]
pub fn pyutil_check_import(py: Python<'_>) -> PyResult<()> {
    let syspath: &PyList = py.import("sys")?.getattr("path")?.downcast()?;
    dbg!(syspath);
    Ok(())
}

#[allow(dead_code)]
#[allow(unused_variables)]
pub fn pyfunc_runtime(pyvenv_path: &str, pysrc_main: &str, pysrc_func: &str, func_args: (&String, )) -> String {
    // A runtime with apt modules and params for the a function
    // Input: python script
    // name of the python function
    // corresponding function params

    // println!("{:?}{:?}", py_command, py_args);
    let py_res = Python::with_gil(|py| -> PyResult<Py<PyAny>> {
        // Append lib path to python, import search_import
        let _ = pylib_import(py, pyvenv_path);
        // Append lib path to python, import search_import

        let app: Py<PyAny> = PyModule::from_code(py, pysrc_main, "", "")?
            .getattr(pysrc_func)?
            .into();

        // see how to separate the app with the runtime
        // println!("{:?}", app);
        app.call1(py, func_args)
    });

    match py_res {
        Ok(py) => py.to_string(),
        Err(e) => {
            let err_string = e.to_string();
            println!("{}", err_string);
            err_string
        }
    }
}
