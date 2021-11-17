import visio_exif


# with open('3Y7H3092.CR2', 'rb') as f:
with open('aa.jpg', 'rb') as f:
    b = f.read()
    exif = visio_exif.getexif(b)
    print(exif)
