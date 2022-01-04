from visio_rust import get_metadata, set_metadata
from utils import md5_checksum

def test_extract_icc():
    with open("tests/images/P1133897_AdobeRGB.jpeg", "rb") as f:
        icc_profile, _ = get_metadata(f.read())
        assert icc_profile


def test_extract_icc_set_metadata():
    with open("tests/images/P1133897_AdobeRGB.jpeg", "rb") as bts_0:
        with open("tests/images/22-canon_tags.jpeg", "rb") as bts_1:
            data_0 = bts_0.read()
            icc_profile_0, _ = get_metadata(data_0)
            
            data_1 = bts_1.read()
            data_1 = bytes(set_metadata(data_1, icc_profile_0, []))
            icc_profile_1, _ = get_metadata(data_1)

            assert md5_checksum(bytes(icc_profile_0)) == md5_checksum(bytes(icc_profile_1))