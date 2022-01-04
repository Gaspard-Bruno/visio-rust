# Visio-Rust

This project aims to compile computer vision libs writen in rust into python bindings.

### Clone the project
* HTTP
  ```bash
  git clone https://github.com/Gaspard-Bruno/visio-rust.git
  ```
* SSH
  ```bash
  git clone git@github.com:Gaspard-Bruno/visio-rust.git
  ```

## Installation
```
cd visio-rust/ && make install
```

## Testing
```
cd visio-rust/ && tox
```
## Usage

```python
from visio_rust import get_metadata, set_metadata

with open('xxxx.jpeg', 'rb') as f:
    input_data = f.read()

    icc_profile, exif_data = get_metadata(input_data)
    output_data = set_metadata(input_data, icc_profile, exif_data)

    print(input_data == output_data)

```

## 🚀 Author
- [@AndersonCancado](https://www.github.com/andycancado)
- [@MarcoJoao](https://www.github.com/marcojoao)