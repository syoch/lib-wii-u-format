import os
from typing import Dict
import zlib
import struct
unpack_prefix = ">"  # big


class Phdr():
    def __init__(self, raw):
        (
            self.p_type,
            self.p_offset,
            self.p_vaddr,
            self.p_paddr,
            self.p_filesz,
            self.p_memsz,
            self.p_flags,
            self.p_align,
        ) = struct.unpack(unpack_prefix+"8L", raw)


class Shdr():
    def __init__(self, raw):
        (
            self.sh_name,
            self.sh_type,
            self.sh_flags,
            self.sh_addr,
            self.sh_offset,
            self.sh_size,
            self.sh_link,
            self.sh_info,
            self.sh_addralign,
            self.sh_entsize
        ) = struct.unpack(unpack_prefix+"10L", raw)

    def load(self, fp):
        oldPos = fp.tell()
        fp.seek(self.sh_offset)

        data = fp.read(self.sh_size)
        if self.sh_flags & 0x0800_0000:
            data = zlib.decompress(data[4:])

        fp.seek(oldPos, os.SEEK_SET)
        return data


class Ehdr():
    class ident():
        def __init__(self, raw):
            global unpack_prefix

            (
                self.magic,
                self.classtype, self.endian,
                self.version,
                self.abi, self.abiver,
                _
            ) = struct.unpack(unpack_prefix+"4s3c2sc6s", raw)
            assert self.magic == b"\x7fELF"
            assert self.classtype == b"\x01"  # 32bit, 64bit=\x02
            assert self.version == b'\x01'  # EV_CURRENT
            unpack_prefix = ">" if self.endian == b"\x02" else "<"

    def __init__(self, raw):
        self.e_ident = Ehdr.ident(raw[0:16])
        raw = raw[16:]

        (
            self.e_type,
            self.e_machine,
            self.e_version,
            self.e_entry,
            self.e_phoff, self.e_shoff,
            self.e_flags,
            self.e_ehsize,
            self.e_phentsize, self.e_phnum,
            self.e_shentsize, self.e_shnum,
            self.e_shstrndx
        ) = struct.unpack(unpack_prefix+"2hll3l6h", raw)
        # assert self.e_machine == 0x0014


class Elf:
    def __init__(self, fname):
        self.fp = open(fname, "rb")
        self.ehdr = Ehdr(self.fp.read(52))
        self.print_ehdr()
        self.header_table: Dict[str, Shdr] = {}
        self.read_all_sections()
        self.read_all_phs()
        # self.print_segments()

    def print_ehdr(self):
        print("Elf header")
        print("  ELF Identifier")
        print("    abi        :", self.ehdr.e_ident.abi)
        print("    abi version:", self.ehdr.e_ident.abiver)
        print("  Section header informations")
        print("    offset   : 0x{:08x}".format(self.ehdr.e_shoff))
        print("    entrysize: 0x{:08x}".format(self.ehdr.e_shentsize))
        print("    hdrcount : {}".format(self.ehdr.e_shnum))
        print("  Segment header informations")
        print("    offset   : 0x{:08x}".format(self.ehdr.e_phoff))
        print("    entrysize: 0x{:08x}".format(self.ehdr.e_phentsize))
        print("    hdrcount : {}".format(self.ehdr.e_phnum))
        print("  entry    : 0x{:08x}".format(self.ehdr.e_entry))
        print("  flags    : 0x{:08x}".format(self.ehdr.e_flags))
        print("  ehsize   : 0x{:08x}".format(self.ehdr.e_ehsize))

    def print_sections(self):
        print("Section List")
        print(" address   |size      |offset    |name")
        print(" ----------|----------|----------|")
        for name, header in self.header_table.items():
            print(" {:#010x}|{:#010x}|{:#010x}|{}".format(
                header.sh_addr, header.sh_size,
                header.sh_offset,
                name
            ))

    def read_all_sections(self):
        self.header_table = {}

        self.fp.seek(self.ehdr.e_shoff +
                     self.ehdr.e_shentsize * self.ehdr.e_shstrndx)
        strtab = Shdr(self.fp.read(self.ehdr.e_shentsize)).load(self.fp)

        self.fp.seek(self.ehdr.e_shoff)
        for i in range(self.ehdr.e_shnum):
            header = Shdr(self.fp.read(self.ehdr.e_shentsize))
            name = strtab[header.sh_name:].split(b"\0", 1)[0].decode()
            self.header_table[name] = header

    def getFunctions(self):
        class func:
            def __init__(self):
                self.name = b""
                self.value = 0
                self.sh = b""

        symtab = self.header_table[".symtab"].load(self.fp)
        strtab = self.header_table[".strtab"].load(self.fp)

        f = func()
        o = 0
        while len(symtab) != o:
            symbol = symtab[o:o+16]
            o += 16
            if symbol[12] != 2:
                continue

            f.value = int.from_bytes(symbol[4:8], "big")

            f.name = int.from_bytes(symbol[0:4], "big")
            f.name = strtab[f.name:strtab.find(b"\0", f.name)]

            f.sh = int.from_bytes(symbol[14:16], "big")
            f.sh = strtab[f.sh:strtab.find(b"\0", f.sh)]

            # if f.sh == b"TEXT" and f.name[0] != ord("."):
            yield f

    def read_all_phs(self):
        self.ph_list: List[Phdr] = []

        self.fp.seek(self.ehdr.e_phoff)
        for i in range(self.ehdr.e_phnum):
            header = Phdr(self.fp.read(self.ehdr.e_phentsize))
            self.ph_list.append(header)

    def print_segments(self):
        print("Segment List")
        print("           |          |Size                 |          |          |")
        print(" Virt Addr |Phy Addr  |Memory    |File      |offset    | End      |")
        print(" ----------|----------|----------|----------|----------|----------+")
        for header in self.ph_list:
            print(" {:#010x}|{:#010x}|{:#010x}|{:#010x}|{:#010x}|{:#010x}|".format(
                header.p_vaddr, header.p_paddr,
                header.p_memsz, header.p_filesz,
                header.p_offset, header.p_filesz+header.p_offset
            ))

    def symbolCount(self):
        return self.header_table[".symtab"].sh_size//16


if __name__ == "__main__":
    print("target   : rpx/Minecraft.Client.rpx")
    Elf("rpx/Minecraft.Client.rpx")
