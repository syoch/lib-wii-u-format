from .binary import StreamReader

from typing import Any, List


class Tag:
    def __init__(self, reader, name, opcode):
        self.opcode: int = opcode
        self.name: str = name
        self.data: Any

        self.read_payload(reader)

    def __repr__(self) -> str:
        return self.__str__()

    def __str__(self) -> str:
        return f"{self.name}: {self.data}"

    def read_payload(self, reader: StreamReader):
        raise NotImplementedError


class IntTag(Tag):
    def read_payload(self, reader: StreamReader):
        self.data = reader.read_int(2 ** (self.opcode - 1))


class EndTag(Tag):
    def __str__(self) -> str:
        return "End"

    def read_payload(self, reader: StreamReader):
        self.data = None


class FloatTag(Tag):
    def read_payload(self, reader: StreamReader):
        self.data = reader.read_float()


class DoubleTag(Tag):
    def read_payload(self, reader: StreamReader):
        self.data = reader.read_double()


class StringTag(Tag):
    def read_payload(self, reader: StreamReader):
        self.data = reader.read_n_sized_string(2)


class ListTag(Tag):
    def __str__(self) -> str:
        lines = [f"List {self.name}({len(self.data)}):"]
        for tag in self.data:
            for line in str(tag).split("\n"):
                lines += [f"  {line}"]
        return "\n".join(lines)

    def read_payload(self, reader: StreamReader):
        self.data: List[Tag] = []

        opcode = reader.read_int(1)
        count = reader.read_int(4)

        for _ in range(count):
            self.data.append(reader.read_tag(opcode=opcode, name=""))


class ByteArrayTag(Tag):
    def __str__(self) -> str:
        return f"ByteArray: {self.data}"

    def read_payload(self, reader: StreamReader):
        self.data = reader.read(reader.read_int(4))


class CompoundTag(Tag):
    def __str__(self) -> str:
        lines = [f"Compound {self.name}:"]
        for tag in self.data:
            for line in str(tag).split("\n"):
                lines += [f"  {line}"]
        return "\n".join(lines)

    def read_payload(self, reader: StreamReader):
        self.data: List[Tag] = []
        while reader.peek(1) != b'\x00':
            self.data.append(reader.read_tag())
        reader.offset += 1


class NBTReader(StreamReader):
    def read_tag(self, opcode=None, name=None):
        if not opcode:
            opcode = self.read_int(1)

        if opcode == 0x00:
            return EndTag(self, name, opcode)

        if name is None:
            name = self.read_n_sized_string(2).decode("utf-8", errors="ignore")

        if 0x01 <= opcode <= 0x04:
            cls = IntTag
        elif opcode == 0x05:
            cls = FloatTag
        elif opcode == 0x06:
            cls = DoubleTag
        elif opcode == 0x07:
            cls = ByteArrayTag
        elif opcode == 0x08:
            cls = StringTag
        elif opcode == 0x09:
            cls = ListTag
        elif opcode == 0x0a:
            cls = CompoundTag
        else:
            raise Exception("Unknown opcode: 0x{:02x}".format(opcode))

        return cls(self, name, opcode)
