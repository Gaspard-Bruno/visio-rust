from visio_rust import get_metadata, set_metadata
from utils import md5_checksum

def test_extract_exif():
    with open("tests/images/Canon_EOS_6D_Mark_II.jpeg", "rb") as f:
        _, exif_data = get_metadata(f.read())
        assert exif_data

def test_extract_exif_set_metadata():
    with open("tests/images/Canon_EOS_6D_Mark_II.jpeg", "rb") as bts_0:
        with open("tests/images/Iphone_13.jpeg", "rb") as bts_1:
            # Image (A) original
            data_0 = bts_0.read()
            _, exif_data_0 = get_metadata(data_0)
            assert exif_data_0
            
            # Image (B) original
            data_1 = bts_1.read()
            _, exif_data_1 = get_metadata(data_1)
            assert exif_data_1
            
            assert md5_checksum(bytes(exif_data_0)) != md5_checksum(bytes(exif_data_1))   # type: ignore
            
            # Image (B) no data
            data_1 = bytes(set_metadata(data_1, [], []))
            _, no_exif_data_1 = get_metadata(data_1)
            assert not no_exif_data_1
            
            # Image (B) new data
            data_1 = bytes(set_metadata(data_1, [], exif_data_0))
            _, new_exif_data_1 = get_metadata(data_1)
            assert new_exif_data_1
            assert md5_checksum(bytes(exif_data_0)) == md5_checksum(bytes(new_exif_data_1))    # type: ignore 