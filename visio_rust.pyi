from typing import List, Tuple, Union

def get_metadata(buf: bytes) -> Tuple[List[int], List[int]]:
    """Get image icc_profile and exif profile
    :param bytes buf: input image bytes

    :returns: Image icc_profile and exif data
    :rtype: Tuple[List[int], List[int]]
    """
    ...  # noqa

def set_metadata(
    buf: bytes, icc_prfile: Union[bytes, List[int]], _exif_data: Union[bytes, List[int]]
) -> List[int]:
    """Set image icc_profile and exif profile
    :param bytes buf: input image bytes
    :param bytes icc_prfile: input icc_prfile bytes
    :param bytes _exif_data: input _exif_data bytes

    :returns: Image encoded with icc_profile and exif data
    :rtype: List[int]
    """
    ...  # noqa
