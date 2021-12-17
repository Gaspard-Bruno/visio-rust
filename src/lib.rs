use pyo3::prelude::*;
use pyo3::types::PyDict;

use img_parts::{DynImage, ImageEXIF, ImageICC};

/// get exif, icc profile from file bytes
#[pyfunction]
#[pyo3(text_signature = "(fields, bytes, /)")]
fn get_metadata_from_bytes(file_bytes: &[u8]) -> (&PyDict, &PyDict) {
    //visio_exif.get_metadata_from_bytes(bytes) -> (exif, iccprofile)

    todo!("get exif, icc profile from file bytes");
}
/// get exif, icc profile from file path
#[pyfunction]
#[pyo3(text_signature = "(fields, bytes, /)")]
fn get_metadata_from_path(file_path: &str) -> (&PyDict, &PyDict) {
    // visio_exif.get_metadata_from_path(path) -> (exif, iccprofile)

    todo!("get exif, icc profile from file path");
}

/// set exif, icc
#[pyfunction]
#[pyo3(text_signature = "(fields, bytes, /)")]
fn set_metadata_from_bytes(file_bytes: &[u8], meta_data: (&PyDict, &PyDict)) -> Vec<u8> {
    // visio_exif.set_metadata_from_bytes(bytes, (exif, iccprofile)) -> bytes

    todo!("set exif, icc");
}
/// set exif, icc
#[pyfunction]
#[pyo3(text_signature = "(fields, bytes, /)")]
fn set_metadata_from_path(file_path: &[u8], meta_data: (&PyDict, &PyDict)) -> Vec<u8> {
    // visio_exif.ser_metadata_from_path(path, (exif, iccprofile))  -> bytes

    todo!("set exif, icc");
}

#[pymodule]
fn visio_exif(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(get_metadata_from_bytes, m)?)?;
    m.add_function(wrap_pyfunction!(get_metadata_from_path, m)?)?;
    m.add_function(wrap_pyfunction!(set_metadata_from_bytes, m)?)?;
    m.add_function(wrap_pyfunction!(set_metadata_from_path, m)?)?;
    Ok(())
}
