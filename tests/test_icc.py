import visio_rust

def test_exp():
    with open('tests/images/22-canon_tags.jpeg', 'rb') as f:
        input_data = f.read()

        icc_profile, exif_data = visio_rust.get_metadata(input_data)
        output_data = visio_rust.set_metadata(input_data, icc_profile, exif_data)

        assert len(icc_profile)  == 0



def test_exp_2():
    assert (True, True)

