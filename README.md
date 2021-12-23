

## Usage

```python
from visio_rust import get_metadata, set_metadata

with open('xxxx.jpeg', 'rb') as f:
    input_data = f.read()

    icc_profile, exif_data = get_metadata(input_data)
    output_data = set_metadata(input_data, icc_profile, exif_data)

    print(input_data == output_data)

```


# Build Steps development
### Build
```
make build
```

### install maturin
```
pip install maturin
```

### test python package
```
maturin develop
```

### build python package
```
maturin build

```

## Testing

To test install tox globally and run

```shell
tox -e py
```