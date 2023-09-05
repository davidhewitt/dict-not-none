use pyo3::{
    ffi::{self, PyTuple_GET_ITEM, PyTuple_GET_SIZE},
    prelude::*,
    types::{PyCFunction, PyDict},
};

// PRESENT FORM

#[pyfunction(signature = (**kwargs))]
fn dict_not_none_pyo3<'py>(py: Python<'py>, kwargs: Option<&'py PyDict>) -> &'py PyDict {
    let out = PyDict::new(py);
    if let Some(kwargs) = kwargs {
        for (key, value) in kwargs.iter() {
            if !value.is_none() {
                out.set_item(key, value).unwrap();
            }
        }
    }
    out
}

// FUTURE FORM

// #[pyfunction(signature = (**kwargs))]
// fn dict_not_none_pyo3<'py>(py: Python<'py>, kwargs: Option<Py2<'py, PyDict>>) -> Py2<'py, PyDict> {
//     let out = PyDict::new(py);
//     if let Some(kwargs) = kwargs {
//         for (key, value) in kwargs.iter() {
//             if !value.is_none() {
//                 out.set_item(key, value).unwrap();
//             }
//         }
//     }
//     out
// }

unsafe extern "C" fn dict_not_none_baremetal(
    _self: *mut ffi::PyObject,
    args: *const *mut ffi::PyObject,
    nargs: ffi::Py_ssize_t,
    kwnames: *mut ffi::PyObject,
) -> *mut ffi::PyObject {
    let kwargs = ffi::PyDict_New();
    let kwargs_count = PyTuple_GET_SIZE(kwnames);
    if nargs != 0 {
        ffi::PyErr_SetString(
            ffi::PyExc_ValueError,
            "expected positional arguments only\0".as_ptr().cast(),
        );
        return std::ptr::null_mut();
    }
    // args is expected to be an array of pointers to PyObject,
    // where the first `nargs` are positional and the trailing are keyword arguments
    // which correspond to `kwnames` in turn.
    let mut current = args;
    for i in 0..kwargs_count {
        if ffi::Py_IsNone(*current) == 0 {
            ffi::PyDict_SetItem(kwargs, PyTuple_GET_ITEM(kwnames, i), *current);
        }
        current = current.add(1);
    }
    kwargs
}

static DICT_NOT_NONE: pyo3::PyMethodDef = pyo3::PyMethodDef::fastcall_cfunction_with_keywords(
    "dict_not_none_baremetal",
    pyo3::methods::PyCFunctionFastWithKeywords(dict_not_none_baremetal),
    "",
);

#[pymodule]
fn _dict_not_none(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(dict_not_none_pyo3, m)?)?;

    m.add_function(PyCFunction::internal_new(&DICT_NOT_NONE, m.into())?)?;

    Ok(())
}
