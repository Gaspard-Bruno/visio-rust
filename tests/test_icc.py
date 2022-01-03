from visio_rust import get_metadata, set_metadata
from utils import md5_checksum

def test_extract_icc_set_metadata():
    with open("./images/lux_iccprofile_test.png", "rb") as input_bytes:
        with open("./images/lux_iccprofile_test_2.jpg", "rb") as input_2_bytes:
            with open("./images/lux_iccprofile_test_output.png", "wb") as output_bytes:
                input_data = input_bytes.read()
                input_2_data = input_2_bytes.read()
                
                icc_profile, _ = get_metadata(input_data)
                output_data = bytes(set_metadata(input_2_data, icc_profile, []))
                icc_profile1, _ = get_metadata(output_data)
                output_bytes.write(output_data)
                
                assert md5_checksum(bytes(icc_profile)) == md5_checksum(bytes(icc_profile1))
                
def test_extract_icc():
    with open("tests/images/P1133897_AdobeRGB.jpeg", "rb") as f:

        icc_profile, _ = get_metadata(f.read())

        assert icc_profile
