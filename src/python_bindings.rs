#[cfg(feature = "pyo3")]
use pyo3::prelude::*;

#[cfg(feature = "pyo3")]
#[pyclass]
#[derive(Debug)]
pub struct PyExecutableInfo {
    #[pyo3(get)]
    pub path: String,
    #[pyo3(get)]
    pub version: String,
}

#[cfg(feature = "pyo3")]
#[pymethods]
impl PyExecutableInfo {
    #[new]
    fn new(path: String, version: String) -> Self {
        Self { path, version }
    }
}

#[cfg(feature = "pyo3")]
#[pyfunction]
fn find_executables_py(command: &str) -> PyResult<Vec<String>> {
    find_executables(command)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))
}

#[cfg(feature = "pyo3")]
#[pyfunction]
fn get_version_py(executable_path: &str) -> PyResult<PyExecutableInfo> {
    get_version(executable_path)
        .map(|info| PyExecutableInfo {
            path: info.path,
            version: info.version,
        })
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))
}

#[cfg(feature = "pyo3")]
#[pyfunction]
fn find_latest_command_py(command: &str) -> PyResult<PyExecutableInfo> {
    find_latest_command(command)
        .map(|info| PyExecutableInfo {
            path: info.path,
            version: info.version,
        })
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))
}

#[cfg(feature = "pyo3")]
#[pymodule]
fn _latest_version(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyExecutableInfo>()?;
    m.add_function(wrap_pyfunction!(find_executables_py, m)?)?;
    m.add_function(wrap_pyfunction!(get_version_py, m)?)?;
    m.add_function(wrap_pyfunction!(find_latest_command_py, m)?)?;

    Ok(())
}
