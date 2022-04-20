import os
import zlib
import struct
from . import ghs_demangle
import capstone

import tqdm
unpack_prefix = ">"  # big

EM_PPC = 20
ELFDATA2LSB = 1  # Little endian
ELFDATA2MSB = 2  # Big endian


class ProgramHeader():
    def __init__(self, raw):
        (
            self.type,
            self.offset,
            self.virtual_address,
            self.physical_addr,
            self.file_size,
            self.mem_size,
            self.flags,
            self.align,
        ) = struct.unpack(unpack_prefix+"8L", raw)


class SectionHeader():
    def __init__(self, fp, raw):
        (
            self.name,
            self.type,
            self.flags,
            self.addr,
            self.offset,
            self.size,
            self.link,
            self.info,
            self.addr_align,
            self.ent_size
        ) = struct.unpack(unpack_prefix+"10L", raw)

        oldPos = fp.tell()
        fp.seek(self.offset)

        self.data = fp.read(self.size)
        if self.flags & 0x0800_0000:
            self.data = zlib.decompress(self.data[4:])

        fp.seek(oldPos, os.SEEK_SET)

    def get_string(self, offset):
        end = self.data.find(b"\x00", offset)
        return self.data[offset:end].decode("utf-8")


class ELFHeader():
    class Ident():
        def __init__(self, raw):
            global unpack_prefix

            (
                self.magic,
                self.classtype, self.endian,
                self.version,
                self.abi, self.abi_version,
                _
            ) = struct.unpack(unpack_prefix+"4s3c2sc6s", raw)

            assert self.magic == b"\x7fELF"
            assert self.endian == ELFDATA2MSB.to_bytes(1, "big")
            assert self.classtype == b"\x01"  # 32bit, 64bit=\x02
            assert self.version == b'\x01'  # EV_CURRENT
            unpack_prefix = ">" if self.endian == b"\x02" else "<"

    def __init__(self, raw):
        self.ident = ELFHeader.Ident(raw[0:16])
        raw = raw[16:]

        (
            self.type,
            self.machine,
            self.version,
            self.entry,
            self.ph_offset, self.sh_offset,
            self.flags,
            self.size,
            self.ph_ent_size, self.ph_count,
            self.sh_ent_size, self.sh_count,
            self.sh_strtab_index
        ) = struct.unpack(unpack_prefix+"2hll3l6h", raw)
        assert self.machine == EM_PPC

    def print(self):
        print("Elf header")
        print("  ELF Identifier")
        print("    abi        :", self.ident.abi)
        print("    abi version:", self.ident.abi_version)
        print("  Section header information")
        print("    offset   : 0x{:08x}".format(self.sh_offset))
        print("    entrysize: 0x{:08x}".format(self.sh_ent_size))
        print("    count    : {}".format(self.sh_count))
        print("  Segment header information")
        print("    offset   : 0x{:08x}".format(self.ph_offset))
        print("    entrysize: 0x{:08x}".format(self.ph_ent_size))
        print("    count    : {}".format(self.ph_count))
        print("  entry    : 0x{:08x}".format(self.entry))
        print("  flags    : 0x{:08x}".format(self.flags))
        print("  size     : 0x{:08x}".format(self.size))


class Symbol:
    def __init__(self):
        self.mangled = ""
        self.name = ""
        self.value = 0
        self.size = 0
        self.info = 0
        self.other = 0
        self.sh = None
        self.data: bytes = b""

    def __repr__(self):
        return "<Symbol {} {:#010x} {:#010x} {:#04x} {}>".format(
            self.name, self.value, self.size, self.info, self.other
        )

    def __str__(self) -> str:
        return f"{self.value:#010x} {self.size:#010x}| {self.name}"

    def disasm(self):
        cs = capstone.Cs(capstone.CS_ARCH_PPC, capstone.CS_MODE_BIG_ENDIAN)
        cs.detail = True
        return cs.disasm(self.data, self.value)


class Elf:
    def __init__(self, fname, load_symbols=True):
        self.fp = open(fname, "rb")
        self.elf_header = ELFHeader(self.fp.read(52))

        self.read_section_headers()
        self.read_program_headers()

        if load_symbols:
            self.load_symbols_and_functions()

    def load_symbols_and_functions(self):
        self.load_symbols()

        self.functions = {
            name: value
            for (name, value) in self.symbols.items()
            if value.info & 0x0f == 0x2
        }

    def read_string_from_strtab(self, offset):
        return self.section_header_table[".strtab"].get_string(offset)

    def read_section_headers(self):
        self.fp.seek(self.elf_header.sh_offset)
        self.headers = [
            SectionHeader(self.fp, self.fp.read(self.elf_header.sh_ent_size))
            for _ in range(self.elf_header.sh_count)
        ]

        sh_str_table = self.headers[self.elf_header.sh_strtab_index]
        self.section_header_table = {
            sh_str_table.get_string(header.name): header
            for header in self.headers
        }

    def read_program_headers(self):
        self.fp.seek(self.elf_header.ph_offset)
        self.ph_list = [
            ProgramHeader(self.fp.read(self.elf_header.ph_ent_size))
            for _ in range(self.elf_header.ph_count)
        ]

    def load_symbols(self):
        self.symbols = {
            symbol.mangled: symbol
            for symbol in self.get_symbols()
        }

    def print_sections(self):
        print("Section List")
        print(" address   |size      |offset    |name")
        print(" ----------|----------|----------|")
        for name, header in self.section_header_table.items():
            print(" {:#010x}|{:#010x}|{:#010x}|{}".format(
                header.addr, header.size,
                header.offset,
                name
            ))

    def print_segments(self):
        print("Segment List")
        print("           |Size                 |          |")
        print("   VAddr   |Memory    |File      |offset    |")
        print(" ----------|----------|----------|----------|")
        for header in self.ph_list:
            print(" {:#010x}|{:#010x}|{:#010x}|{:#010x}|{:#010x}|".format(
                header.virtual_address,
                header.mem_size, header.file_size,
                header.offset
            ))

    def get_symbols(self):
        symtab = self.section_header_table[".symtab"].data
        demangler = ghs_demangle.Demangler()

        for i in tqdm.trange(0, len(symtab), 16):
            symbol = symtab[i:i+16]

            f = Symbol()

            name = int.from_bytes(symbol[0:4], "big")

            f.mangled = self.read_string_from_strtab(name)
            f.name = str(demangler.demangle(f.mangled))
            f.value = int.from_bytes(symbol[4:8], "big")
            f.size = int.from_bytes(symbol[8:12], "big")
            f.info = symbol[12]
            f.other = symbol[13]

            section = int.from_bytes(symbol[14:16], "big")

            f.sh = section
            f.data = self.headers[section].data[
                f.value - self.headers[section].addr:
                f.value - self.headers[section].addr + f.size
            ]

            yield f

    def _used_blocks(self):
        yield range(0, self.elf_header.size)

        offset = self.elf_header.sh_offset
        if offset != 0:
            count = self.elf_header.sh_count
            ent_size = self.elf_header.sh_ent_size
            yield range(
                offset,
                offset + count * ent_size
            )

        offset = self.elf_header.ph_offset
        if offset != 0:
            ent_size = self.elf_header.ph_ent_size
            count = self.elf_header.ph_count
            yield range(offset, offset + count * ent_size)

        for name, header in self.section_header_table.items():
            if name == ".bss":
                continue
            yield range(
                header.offset,
                header.offset + header.size
            )

        for program_header in self.ph_list:
            yield range(program_header.offset, program_header.offset + program_header.file_size)

    def used_blocks(self):
        table = {
            r.start: r.stop
            for r in self._used_blocks()
        }
        while True:
            for k in table:
                v = table[k]
                if v in table:
                    table[k] = table[v]
                    del table[v]
                    break
            else:
                break

        return [
            (k, table[k])
            for k in table
        ]


if __name__ == "__main__":
    print("target   : rpx/Minecraft.Client.rpx")
    elf = Elf("/usr/bin/ls")
    elf.print_segments()
