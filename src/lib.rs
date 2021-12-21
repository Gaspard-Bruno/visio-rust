// use pyo3::prelude::*;
// use pyo3::types::{PyByteArray, PyDict};
// use std::io::Cursor;

// use img_parts::{Bytes, DynImage, ImageEXIF, ImageICC};

// /// get exif, icc profile from file bytes
// #[pyfunction]
// #[pyo3(text_signature = "(fields, bytes, /)")]
// fn get_metadata_from_bytes(file_bytes: &[u8]) -> () {
//     //visio_exif.get_metadata_from_bytes(bytes) -> (exif, iccprofile)

//     let (iccp, exif) = DynImage::from_bytes(file_bytes.clone().into())
//         .expect("image loaded")
//         .map_or((None, None), |dimg| (dimg.icc_profile(), dimg.exif()));

//     ()
// }
// /// get exif, icc profile from file path
// #[pyfunction]
// #[pyo3(text_signature = "(fields, bytes, /)")]
// fn get_metadata_from_path(file_path: &str) -> (&PyDict, &PyDict) {
//     // visio_exif.get_metadata_from_path(path) -> (exif, iccprofile)

//     todo!("get exif, icc profile from file path");
// }

// /// set exif, icc
// #[pyfunction]
// #[pyo3(text_signature = "(fields, bytes, /)")]
// fn set_metadata_from_bytes(file_bytes: &[u8], meta_data: (&PyDict, &PyDict)) -> Vec<u8> {
//     // visio_exif.set_metadata_from_bytes(bytes, (exif, iccprofile)) -> bytes

//     todo!("set exif, icc");
// }
// /// set exif, icc
// #[pyfunction]
// #[pyo3(text_signature = "(fields, bytes, /)")]
// fn set_metadata_from_path(file_path: &[u8], meta_data: (&PyDict, &PyDict)) -> Vec<u8> {
//     // visio_exif.ser_metadata_from_path(path, (exif, iccprofile))  -> bytes

//     todo!("set exif, icc");
// }

// #[pymodule]
// fn visio_exif(_py: Python, m: &PyModule) -> PyResult<()> {
//     m.add_function(wrap_pyfunction!(get_metadata_from_bytes, m)?)?;
//     m.add_function(wrap_pyfunction!(get_metadata_from_path, m)?)?;
//     m.add_function(wrap_pyfunction!(set_metadata_from_bytes, m)?)?;
//     m.add_function(wrap_pyfunction!(set_metadata_from_path, m)?)?;
//     Ok(())
// }
