import toml
from setuptools import setup
from setuptools_rust import RustExtension

with open("README.md", encoding="utf-8", errors="ignore") as fp:
    long_description = fp.read()

with open("Cargo.toml") as fp:
    pkg = toml.load(fp).get("package")
    pkg_name = pkg["name"]
    pkg_version = pkg["version"]

setup(
    name=pkg_name,
    version=pkg_version,
    description="mode",
    long_description=long_description,
    long_description_content_type="text/markdown",
    url="https://github.com/Gaspard-Bruno/visio-rust",
    author="Anderson Cançado, Marco João",
    author_email="anderson.cancado@gaspardbruno.com, marco.joao@gaspardbruno.com",
    keywords="machine learning, computer vision, visio, ml",
    python_requires=">=3.6",
    rust_extensions=[RustExtension(pkg_name, debug=False)],
    zip_safe=False,
)