use pyo3::prelude::*;
use pyo3::types::{PyBytes, PyDict};
use std::io::Cursor;

#[pyfunction]
fn getexif(bts: &[u8]) -> PyResult<()> {
    let res = read_bytes(bts);
    match res {
        Ok(r) => {
            for f in r.fields() {
                println!("{}: {}", f.tag, f.display_value().with_unit(&r));
            }
        }
        Err(e) => {
            println!("ERROR {}", e);
            ()
        }
    }
    Ok(())
}


fn read_metadata(bytes: &PyBytes) -> () {
    // TODO read metadata to return a dictionary
    
}

fn write_metadata(dict: PyDict, bytes: &PyBytes) -> () {
    // TODO write metadata and bytes to return bytes
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