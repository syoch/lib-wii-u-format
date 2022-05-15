from . import rpx
import tqdm


def main():
    elf = rpx.Elf("rpx/Minecraft.Client.rpx")
    symtab = elf.header_table[".symtab"].load(elf.fp)
    strtab = elf.header_table[".strtab"].load(elf.fp)

    dest = open("rpx/mc.ld", "w")
    dest.write("SECTIONS{\n")

    # skip
    while True:
        name = int.from_bytes(symtab[0:4], "big")
        if strtab[name] == ord("."):
            break
        symtab = symtab[16:]
    while True:
        name = int.from_bytes(symtab[0:4], "big")
        if strtab[name] != ord("."):
            break
        symtab = symtab[16:]
    # skip -end
    for i in tqdm.tqdm(range(len(symtab)//16)):
        offset = int.from_bytes(symtab[0:4], "big")
        endoff = strtab.find(0, offset)
        funcname = strtab[offset:endoff].decode()
        name = ".text."+funcname
        val = symtab[4:8].hex()
        symtab = symtab[16:]

        dest.write(f"  {name} :{{\n")
        dest.write(f"    *({name})\n")
        dest.write(f"    {funcname} = 0x{val};\n")
        dest.write("  }}\n")

    dest.write("}\n")
    print("")
