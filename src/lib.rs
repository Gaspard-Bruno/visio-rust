use exif::Reader;
use pyo3::prelude::*;
use std::fs::File;
use std::io::{BufReader, Cursor, Read, Write};

#[pyfunction]
fn calculate(nterms: u64) -> PyResult<f64> {
    let numerator = 4.0;
    let mut denominator = 1.0;
    let mut operation = 1.0;
    let mut pi = 0.0;

    for _ in 0..nterms {
        pi += operation * (numerator / denominator);
        denominator += 2.0;
        operation *= -1.0;
    }
    Ok(pi)
}

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

#[pymodule]
fn calculate_pi(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(calculate, m)?)?;
    m.add_function(wrap_pyfunction!(getexif, m)?)?;
    Ok(())
}

fn read_bytes(b: &[u8]) -> Result<exif::Exif, exif::Error> {
    let mut x = Cursor::new(b);
    let exif = exif::Reader::new().read_from_container(&mut x)?;

    Ok(exif)
}
