from setuptools import setup

from setuptools_rust import Binding, RustExtension

setup(
    name="visio_rust",
    version="0,1.0",
    rust_extensions=[
        RustExtension(
            {"libvisio_rust.so": "visio_rust"},
            binding=Binding.Exec,
            script=True,
        )
    ],
    # rust extensions are not zip safe, just like C-extensions.
    zip_safe=False,
)
