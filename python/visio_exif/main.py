import visio_img_meta


# with open('3Y7H3092.CR2', 'rb') as f:
with open('ab.jpg', 'rb') as f:
    b = f.read()
    exif = visio_img_meta.export_metadata(b)
    print(exif)
