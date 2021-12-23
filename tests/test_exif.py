from visio_rust import get_metadata, set_metadata
from utils import md5_checksum
import os
from io import BytesIO


def test_extract_exif_set_metadata():
    with open("tests/images/22-canon_tags.jpeg", "rb") as f:
        with open("tests/images/P1133897.jpg", "rb") as icc_file:

            input_data = f.read()
            icc_file_data = icc_file.read()
            x = BytesIO(icc_file_data)

            icc_profile, exif_data = get_metadata(input_data)
            output_data = set_metadata(x.getbuffer(), icc_profile, exif_data)
            icc_profile1, exif_data1 = get_metadata(output_data)

            assert md5_checksum(bytes(exif_data1)) == md5_checksum(bytes(exif_data))


def test_extract_exif():
    with open("tests/images/P1133897_AdobeRGB.jpeg", "rb") as f:

        icc_profile, exif_data = get_metadata(f.read())

        assert exif_data
