// #![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::fmt::format;
use std::fs;
use std::path::Path;

use pyo3::prelude::*;
use pyo3::types::*;
use tauri::Manager;

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

    app.invoke_handler(tauri::generate_handler![tauri_main,])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command(rename_all = "snake_case")]
fn tauri_main() -> String {

  let pysrc = pysrc_main();
  let py_res = pyfunc_runtime(PY_VENV_PATH, pysrc, "main", ("None", ));

  format!("Hello, Rust and {}", py_res)
}
