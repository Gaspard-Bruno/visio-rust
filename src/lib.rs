use img_parts::{Bytes, DynImage, ImageEXIF, ImageICC};
use pyo3::prelude::*;

#[pyfunction]
#[pyo3(text_signature = "(buf, icc_profile, exif_data, /)")]
pub fn set_metadata(buf: Vec<u8>, icc_profile: Vec<u8>, _exif_data: Vec<u8>) -> Vec<u8> {
    match DynImage::from_bytes(buf.clone().into()).unwrap() {
        Some(mut img) => {
            //img.set_exif(Bytes::from(exif_data).into());
            img.set_icc_profile(Bytes::from(icc_profile).into());
            img.encoder().bytes().to_vec()
        }
        None => buf,
    }
}

#[pyfunction]
#[pyo3(text_signature = "(buf, /)")]
pub fn get_metadata(buf: Vec<u8>) -> (Vec<u8>, Vec<u8>) {
    match DynImage::from_bytes(buf.into()).unwrap() {
        Some(img) => {
            
            let icc_profile = img.icc_profile().unwrap_or_default().to_vec();
            let exif_data = img.exif().unwrap_or_default().to_vec();
            (icc_profile, exif_data)
        }
        None => (vec![], vec![]),
    }
}

#[pymodule]
#[pyo3(name = "visio_rust")]
fn visio_rust(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(set_metadata, m)?)?;
    m.add_function(wrap_pyfunction!(get_metadata, m)?)?;
    Ok(())
}
