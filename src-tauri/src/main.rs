// #![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::fmt::format;
use std::fs;
use std::path::Path;
use std::str::FromStr;

use pyo3::prelude::*;
use pyo3::types::*;
use tauri::Manager;

mod pyhelpers;

// TODO use dotenv
const PY_VENV_PATH: &str = r#"D:\RustRoot\RustPolars\00_venv\portopt\Lib\site-packages"#;

fn main() {
    let app = tauri::Builder::default().setup(|app| {
        {
            let window = app.get_window("main").unwrap();
            window.open_devtools();
            // window.close_devtools();
        }
        Ok(())
    });

    app.invoke_handler(tauri::generate_handler![
        serve_python_string,
        server_check_interacton,
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

#[tauri::command(rename_all = "snake_case")]
fn serve_python_string() -> String {
    let func_args = (&String::from("None"),);

    let pysrc = pyhelpers::pysrc_main();
    let py_res = pyhelpers::pyfunc_runtime(PY_VENV_PATH, pysrc, "main", func_args);

    format!("Hello, Rust and {}", py_res)
}

#[tauri::command(rename_all = "snake_case")]
fn server_check_interacton(input_string: String) -> String {
    dbg!(user_input)
}

mod test {
    use crate::serve_python_string;

    #[test]
    fn test_py_str() {
        assert_eq!(
            serve_python_string(),
            "Hello, Rust and Hello, Python".to_string()
        )
    }
}
