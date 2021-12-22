use pyo3::prelude::*;
use base64::encode;
use img_parts::{Bytes, DynImage, ImageEXIF, ImageICC};

#[pyfunction]
#[pyo3(text_signature = "(buf, icc_profile, exif_data, /)")]
pub fn import_metadata(buf:Vec<u8>, icc_profile: Vec<u8>, exif_data: Vec<u8>) -> String {
    match DynImage::from_bytes(buf.clone().into()).unwrap() {
        Some(mut img) => {
            img.set_icc_profile(Bytes::from(icc_profile).into());
            img.set_exif(Bytes::from(exif_data).into());
            encode(img.encoder().bytes().to_vec()) 
        },
        None => encode(buf)
    }
}

#[pyfunction]
#[pyo3(text_signature = "(buf, /)")]
pub fn export_metadata(buf: Vec<u8>) -> (Vec<u8>, Vec<u8>) {

    match DynImage::from_bytes(buf.into()).unwrap() {
        Some(img) => { 
            let icc_profile = img.icc_profile().unwrap_or(Bytes::new()).to_vec();
            let exif_data = img.exif().unwrap_or(Bytes::new()).to_vec();
            (icc_profile, exif_data)
        },
        None => (vec![], vec![])
    }
}

#[pymodule]
#[pyo3(name = "visio_img_meta")]
fn visio_exif(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(import_metadata, m)?)?;
    m.add_function(wrap_pyfunction!(export_metadata, m)?)?;
    Ok(())
}
