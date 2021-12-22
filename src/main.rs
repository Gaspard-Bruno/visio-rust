// use std::collections::HashMap;
// use std::env::args;
// use std::fs;

// use std::process::exit;
// use img_parts::{DynImage, ImageEXIF, ImageICC, Bytes};


// fn main() {
//     let mut args = args();
//     args.next();

//     let input_path = match args.next() {
//         Some(path) => path,
//         None => {
//             eprintln!("Please specify the input file path (must be a jpeg, png or webp)");
//             exit(1);
//         }
//     };

//     let input_buf = fs::read(input_path).expect("read input file");
    
//     let nodata = Bytes::new().to_vec();
//     let meta = export_metadata(input_buf.clone());
//     let output_buf = import_metadata(input_buf.clone(), meta["ICC_PROFILE"].clone(), nodata.clone());
//     let meta = export_metadata(output_buf.clone());
//     //let output_buf
//     println!("DATA: {:?}", meta)
// }


// pub fn import_metadata(buf:Vec<u8>, icc_profile: Vec<u8>, exif_data: Vec<u8>) ->  Vec<u8> {
//     let original_image = buf.clone();
//     let bts = Bytes::from(buf);
//     let iccp = Bytes::from(icc_profile);
//     let exif = Bytes::from(exif_data);
//     let image = DynImage::from_bytes(bts).expect("image loaded");
//     if image.is_some() {
//         let mut out = image.unwrap();
//         out.set_icc_profile(iccp.into());
//         out.set_exif(exif.into());
//         let out_bts = out.encoder().bytes();
//         out_bts.to_vec()
//     } else {
//         original_image
//     }
// }

// pub fn export_metadata(buf: Vec<u8>) -> HashMap<&'static str, Vec<u8>> {
//     // extract ICC and EXIF metadata
//     let  (iccp, exif) = DynImage::from_bytes(buf.into())
//         .expect("image loaded")
//         .map_or((None, None), |dimg| (dimg.icc_profile(), dimg.exif()));
        
//     let icc_profile = iccp.unwrap_or(Bytes::new()).to_vec();
//     let exif_data = exif.unwrap_or(Bytes::new()).to_vec();
//     let metadata = HashMap::from([
//         ("ICC_PROFILE", icc_profile),
//         ("EXIF_DATA", exif_data),
//     ]);
//     metadata
// }



// // /// saves the image to the specified `path`
// // ///
// // /// If `iccp` *and* `exif` are None, the image is written directly to `path`, else
// // /// it is buffered in memory, modified by img-parts and then written to `path`.
// // fn save(img: image::DynamicImage, path: &Path, iccp: Option<img_parts::Bytes>, exif: Option<img_parts::Bytes>) {
// //     let out_format = image::ImageFormat::from_path(&path).expect("detected output format");
// //     let mut out_file = File::create(path).expect("create output file");

// //     if iccp.is_some() || exif.is_some() {
// //         let mut out = BytesMut::new().writer();
// //         img.write_to(&mut out, out_format).expect("image encoded");
// //         let out = out.into_inner().freeze();

// //         match DynImage::from_bytes(out.clone()).expect("image loaded") {
// //             Some(mut dimg) => {
// //                 dimg.set_icc_profile(iccp);
// //                 dimg.set_exif(exif);
// //                 dimg.encoder()
// //                     .write_to(out_file)
// //                     .expect("output file written");
// //             }
// //             None => out_file.write_all(&out).expect("output file written"),
// //         };
// //     } else {
// //         img.write_to(&mut out_file, out_format)
// //             .expect("image encoded without ICCP or EXIF");
// //     }
// // }