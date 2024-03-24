# Example of using Tauri and PyO3 with Vanilla JS
The example follows the PyO3 and Tauri guides, link below.
To run in dev mode, simply use `cargo tauri dev`.

To build, before using `cargo tauri build`, the following is needed to setup PyO3 for its linker.
- Provide environement param `PYO3_PYTHON`, which point to the Python interpreter,
- Add to PATH the directory for the directory contain Python interpreter.

After build, the Python (virtual) environement may not be bundled (e.g. in my case of pyenv and venv on windows).
However, the linker does make the exe points to its directory. So in which,
- Include the Python DLLs, `python3.dll` and `python3XX.dll`.
- The Python standard library, in a `Lib` directory (i.e. the `stdlib dir`).
- If no Tkinter feature is needed, it could be easier to get those from an embedded Python distribution.
- Bundling 3rd party packages is similar, the `site-packages` directory is also needed.

## Links
https://pyo3.rs/v0.10.1/python_from_rust
https://tauri.app/v1/guides/getting-started/setup/html-css-js

