
use pyo3::prelude::*;
use pyo3::types::*;
use tauri::Manager;

fn main() {
    let app = tauri::Builder::default().setup(|app| {
        {
            let window = app.get_window("main").unwrap();
            window.open_devtools();
        }
        Ok(())
    });

    app.invoke_handler(tauri::generate_handler![
        add_numbers,
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

#[tauri::command(rename_all = "snake_case")]
fn add_numbers(num1: i32, num2: i32) -> String {

    let pysrc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "./src/add_numbers.py"));
    let py_res = Python::with_gil(|py| -> PyResult<Py<PyAny>> {

        let app: Py<PyAny> = PyModule::from_code(py, pysrc, "", "")?
            .getattr("add_numbers")?
            .into();

        app.call1(py, (num1, num2))
    });

    dbg!(&py_res);
    match py_res {
        Ok(py) => py.to_string(),
        Err(e) => {
            let err_string = e.to_string();
            err_string
        }
    }
}

#[cfg(test)]
mod test {
    use crate::serve_python_string;

    #[test]
    fn test_py_str() {
        assert_eq!(
            serve_python_string(1, 2),
            "3".to_string()
        )
    }
}
