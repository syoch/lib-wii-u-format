import string
import struct

printable = (string.ascii_letters + string.digits +
             string.punctuation + " ").encode()


def _hexdump(dat, offset=0):
    global printable
    for i in range(0, len(dat), 16):
        line = f"{offset+i:08x}: "
        for j in range(0, 16, 4):
            for k in range(4):
                line += hex(dat[i + j + k])[2:].zfill(2)
            line += " "

        for j in range(16):
            ch = dat[i+j].to_bytes(1, "big")
            line += ch.decode("utf-8") if ch in printable else "."

        print(line)


class StreamReader:
    def __init__(self, data):
        self.data = data
        self.offset = 0

    @staticmethod
    def from_path(path):
        with open(path, "rb") as f:
            return StreamReader(f.read())

    def hexdump(self, length=0x100):
        _hexdump(self.peek(length), self.offset)

    def peek(self, size) -> bytes:
        return self.data[self.offset:self.offset+size]

    def read(self, n) -> bytes:
        data = self.peek(n)
        self.offset += n
        return data

    def read_int(self, size) -> int:
        return int.from_bytes(self.read(size), "big")

    def read_n_sized_string(self, n) -> bytes:
        length = self.read_int(n)
        return self.read(length)

    def read_sz_string(self) -> bytes:
        start = self.offset
        end = self.data.find(b"\x00", start)
        if end == -1:
            end = len(self.data)
        self.offset = end + 1
        return self.data[start:end]

    def read_from_format(self, fmt, reversed_endian=False):
        size = struct.calcsize(fmt)
        data = self.read(size)
        if reversed_endian:
            data = data[::-1]
        return struct.unpack(fmt, data)[0]

    def read_float(self) -> float:
        return self.read_from_format("f")

    def read_double(self) -> float:
        return self.read_from_format("d")
