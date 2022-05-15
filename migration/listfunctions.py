import rpx
import ghs_demangle
import logging
import tqdm

logging.basicConfig(
    level=logging.INFO
)

demangler = ghs_demangle.Demangler()


with open("b", "w") as de:
    with open("a", "r") as fp:
        for line in tqdm.tqdm(fp.readlines()):
            addr = line.split(" ")[0][:-1].zfill(8)
            name = line[len(addr)+1:-1]

            # demangled = demangler.demangle(name)
            # demangled_name = str(demangled)

            de.write(str(addr))
            de.write(" ")
            de.write(name)
            de.write("\n")
