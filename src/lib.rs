use pyo3::prelude::*;
use pyo3::types::{PyBytes, PyDict};
use std::collections::HashMap;
use std::io::Cursor;

#[pyfunction]
fn getexif(bts: &[u8]) -> PyResult<HashMap<String, String>> {
    let res = read_bytes(bts);
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

fn read_metadata(bytes: &PyBytes) -> () {
    // TODO read metadata to return a dictionary
    todo!();
}

fn write_metadata(dict: PyDict, bytes: &PyBytes) -> () {
    // TODO write metadata and bytes to return bytes
    todo!();
}

fn read_bytes(b: &[u8]) -> Result<exif::Exif, exif::Error> {
    let mut x = Cursor::new(b);
    let exif = exif::Reader::new().read_from_container(&mut x)?;

    Ok(exif)
}

#[pymodule]
fn visio_exif(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(getexif, m)?)?;

    Ok(())
}
