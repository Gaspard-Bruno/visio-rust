use pyo3::prelude::*;
use pyo3::types::{PyBytes, PyDict};
use std::collections::HashMap;
use std::io::Cursor;

/// Get exif dictionary from image file
#[pyfunction]
#[pyo3(text_signature = "(bytes, /)")]
fn getexif(bytes: &[u8]) -> PyResult<HashMap<String, String>> {
    let res = read_bytes_from_container(bytes);
    let mut exif = HashMap::new();
    match res {
        Ok(r) => {
            for f in r.fields() {
                exif.insert(f.tag.to_string(), f.display_value().to_string());
            }
        }
        Err(e) => {
            println!("ERROR {}", e);
            ()
        }
    }
    Ok(exif)
}

/// Read read_metadata from exif bytes
#[pyfunction]
#[pyo3(text_signature = "(bytes, /)")]
fn read_metadata(_bytes: &PyBytes) -> () {
    todo!("read metadata to return a dictionary");
}

/// Write exif bytes from dictionary
#[pyfunction]
#[pyo3(text_signature = "(fields, bytes, /)")]
fn write_metadata(_fields: &PyDict, _bytes: &PyBytes) -> () {
    todo!("write metadata and bytes to return bytes");
}

/// Read bytes from container
fn read_bytes_from_container(b: &[u8]) -> Result<exif::Exif, exif::Error> {
    let mut x = Cursor::new(b);
    let exif = exif::Reader::new().read_from_container(&mut x)?;

    Ok(exif)
}

#[pymodule]
fn visio_exif(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(getexif, m)?)?;
    m.add_function(wrap_pyfunction!(read_metadata, m)?)?;
    m.add_function(wrap_pyfunction!(write_metadata, m)?)?;
    Ok(())
}
