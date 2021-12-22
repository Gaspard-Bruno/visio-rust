import visio_img_meta

with open('images/22-canon_tags.jpeg', 'rb') as f:
    input_data = f.read()

    icc_profile, exif_data = visio_img_meta.get_metadata(input_data)
    output_data = visio_img_meta.set_metadata(input_data, icc_profile, exif_data)

    print(input_data == output_data)
