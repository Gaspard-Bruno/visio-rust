from typing import Tuple
import hashlib
import os
import io


def md5_checksum(data: Tuple[str, bytearray, bytes, io.BufferedReader, io.FileIO]) -> str:
    """
    create md5 checksum
    :param data: input data to check md5 checksum
    :type data: str, bytearray, bytes, io.BufferedReader, io.FileIO
    :return: md5 hash
    :rtype: str
    """
    # byte
    if isinstance(data, (bytes, bytearray)):
        return hashlib.md5(data).hexdigest()

    # file
    elif isinstance(data, str) and os.access(data, os.R_OK):
        return hashlib.md5(open(data, "rb").read()).hexdigest()

    # file object
    elif isinstance(data, (io.BufferedReader, io.FileIO)):
        return hashlib.md5(data.read()).hexdigest()

    # string
    elif isinstance(data, str):
        return hashlib.md5(data.encode()).hexdigest()

    else:
        raise ValueError("invalid input. input must be string, byte or file")
