from setuptools import setup

from setuptools_rust import Binding, RustExtension

setup(
    name="visio_rust",
    version="0.1.2",
    rust_extensions=[RustExtension("visio_rust", debug=False)],
    # rust extensions are not zip safe, just like C-extensions.
    zip_safe=False,
)
