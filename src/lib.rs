use pyo3::prelude::*;
use std::collections::HashMap;

use img_parts::{Bytes, DynImage, ImageEXIF, ImageICC};

#[pyfunction]
#[pyo3(text_signature = "(buf, icc_profile, exif_data, /)")]
pub fn import_metadata(buf: Vec<u8>, icc_profile: Vec<u8>, exif_data: Vec<u8>) -> Vec<u8> {
    let original_image = buf.clone();
    let iccp = Bytes::from(icc_profile);
    let exif = Bytes::from(exif_data);

    match DynImage::from_bytes(buf.into()).expect("image loaded")  {
        Some(img) => {
            let mut out = img;
            out.set_icc_profile(iccp.into());
            out.set_exif(exif.into());
            out.encoder().bytes().to_vec()
          
        },
        _ => original_image
        
    }

}

#[pyfunction]
#[pyo3(text_signature = "(buf, /)")]
pub fn export_metadata(buf: Vec<u8>) -> HashMap<&'static str, Vec<u8>> {
    // extract ICC and EXIF metadata
    let (iccp, exif) = DynImage::from_bytes(buf.into())
        .expect("image loaded")
        .map_or((None, None), |dimg| (dimg.icc_profile(), dimg.exif()));

    let icc_profile = iccp.unwrap_or(Bytes::new()).to_vec();
    let exif_data = exif.unwrap_or(Bytes::new()).to_vec();
    let metadata = HashMap::from([("ICC_PROFILE", icc_profile), ("EXIF_DATA", exif_data)]);
    metadata
}

#[pymodule]
#[pyo3(name = "visio_img_meta")]
fn visio_exif(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(import_metadata, m)?)?;
    m.add_function(wrap_pyfunction!(export_metadata, m)?)?;
    Ok(())
}
