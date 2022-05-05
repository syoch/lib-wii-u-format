import sys
import os
import pathlib

from .range_manager import RangeManager
from .binary import StreamReader

with open("/shared/Resources/WiiUBinarys/MediaWiiU0.arc", "rb") as fp:
    data = fp.read()

reader = StreamReader(data)

entries = reader.read_int(4)

ram = pathlib.Path("ram")

for i in range(entries):
    path = reader.read_n_sized_string(2).decode()
    offset = reader.read_int(4)
    size = reader.read_int(4)

    path = ram / pathlib.Path(path.replace("\\", "/"))
    path = str(path)

    os.makedirs(os.path.dirname(path), exist_ok=True)
    with open(path, "wb") as fp:
        fp.write(data[offset:offset+size])
