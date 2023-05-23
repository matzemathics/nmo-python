use std::fs::read_to_string;

use nemo::{
    io::{OutputFileManager, RecordWriter},
    logical::execution::ExecutionEngine,
    physical::dictionary::value_serializer::TrieSerializer,
};
use pyo3::{create_exception, prelude::*};

create_exception!(module, NemoError, pyo3::exceptions::PyException);

trait PythonResult {
    type Value;

    fn py_res(self) -> PyResult<Self::Value>;
}

impl<T> PythonResult for Result<T, nemo::error::Error> {
    type Value = T;

    fn py_res(self) -> PyResult<Self::Value> {
        self.map_err(|err| NemoError::new_err(format!("{}", err)))
    }
}

#[pyclass]
#[derive(Clone)]
struct NemoProgram(nemo::logical::model::Program);

#[pyfunction]
fn load_file(file: String) -> PyResult<NemoProgram> {
    let contents = read_to_string(file)?;
    let program = nemo::io::parser::parse_program(contents).py_res()?;
    Ok(NemoProgram(program))
}

#[pyfunction]
fn load_string(rules: String) -> PyResult<NemoProgram> {
    let program = nemo::io::parser::parse_program(rules).py_res()?;
    Ok(NemoProgram(program))
}

#[pyclass]
struct NemoOutputManager(nemo::io::OutputFileManager);

#[pymethods]
impl NemoOutputManager {
    #[new]
    #[pyo3(signature =(path,overwrite=false, gzip=false))]
    fn py_new(path: String, overwrite: bool, gzip: bool) -> PyResult<Self> {
        let output_manager = OutputFileManager::try_new(path.into(), overwrite, gzip).py_res()?;

        Ok(NemoOutputManager(output_manager))
    }
}

#[pyclass]
#[derive(Clone)]
struct NemoIdb(nemo::logical::execution::execution_engine::IdbPredicate);

#[pymethods]
impl NemoIdb {
    fn name(&self) -> String {
        format!("{}", self.0)
    }
}

#[pyclass(unsendable)]
struct NemoEngine(nemo::logical::execution::DefaultExecutionEngine);

#[pymethods]
impl NemoEngine {
    #[new]
    fn py_new(program: NemoProgram) -> PyResult<Self> {
        let engine = ExecutionEngine::initialize(program.0).py_res()?;
        Ok(NemoEngine(engine))
    }

    fn reason(&mut self) -> PyResult<Vec<NemoIdb>> {
        self.0.execute().py_res()?;
        let results = self.0.combine_results().py_res()?;
        Ok(results.into_iter().map(NemoIdb).collect())
    }

    fn write_result(
        &self,
        output_manager: &PyCell<NemoOutputManager>,
        predicate: NemoIdb,
    ) -> PyResult<()> {
        let mut writer = output_manager
            .borrow()
            .0
            .create_file_writer(predicate.0.identifier())
            .py_res()?;

        if let Some(table) = self.0.table_serializer(predicate.0) {
            writer.write_trie(table).py_res()?;
        }

        Ok(())
    }

    fn result(&self, predicate: NemoIdb) -> PyResult<Vec<Vec<String>>> {
        let Some(mut table) = self.0.table_serializer(predicate.0) else { return Ok(Vec::new()); };

        let mut res = Vec::new();

        while let Some(record) = table.next_record() {
            res.push(
                record
                    .into_iter()
                    .map(|v| String::from_utf8(v.as_ref().to_vec()).unwrap())
                    .collect(),
            );
        }

        Ok(res)
    }
}

/// Python bindings for the nemo reasoner
#[pymodule]
fn nmo_python(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<NemoProgram>()?;
    m.add_class::<NemoEngine>()?;
    m.add_class::<NemoOutputManager>()?;
    m.add_function(wrap_pyfunction!(load_file, m)?)?;
    m.add_function(wrap_pyfunction!(load_string, m)?)?;
    Ok(())
}
