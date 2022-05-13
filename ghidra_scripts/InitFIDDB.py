# TODO write a description for this script
# @author
# @category MyScript
# @keybinding
# @menupath
# @toolbar

import subprocess
import os

from ghidra.program.model.symbol import SourceType
from ghidra.program.model.listing import CodeUnit

GHS_DEMANGLE_PY = r"d:\\ghs_demangle.py"

GHS_DEMANGLE_CMDLINE = [
    "python3",
    GHS_DEMANGLE_PY,
    "rab",
]

namespace_root = currentProgram.globalNamespace
funcmgr = currentProgram.getFunctionManager()
demangler = subprocess.Popen(
    GHS_DEMANGLE_CMDLINE, stdout=subprocess.PIPE, stdin=subprocess.PIPE)


class Function:
    def __init__(self, is_static, path, args):
        self.is_static = is_static
        self.path = path
        self.args = args


class FunctionReader:
    def __init__(self, data):
        self.data = data
        self.pos = 0

    def read(self, n):
        ret = self.data[self.pos:self.pos+n]
        self.pos += n
        return ret

    def read_u16(self):
        return int.from_bytes(self.read(2), byteorder='little')

    def read_list_string(self):
        length = self.read_u16()
        ret = []
        for i in range(length):
            length = self.read_u16()
            ret.append(self.read(length).decode('utf-8'))
        return ret

    def read_function(self):
        is_static = self.read(1) == b'\x01'
        path = self.read_list_string()
        args = self.read_list_string()
        return Function(is_static, path, args)


def demangle(s):
    global demangler
    demangler.stdin.write(s.encode('utf-8'))
    demangler.stdin.write(b'\r\n')
    demangler.stdin.flush()

    ret = demangler.stdout.read(10000)
    return FunctionReader(ret).read_function()


def create_namespace(root, name):
    try:
        namesp = currentProgram.symbolTable.createNameSpace(
            root,
            name,
            ghidra.program.model.symbol.SourceType.USER_DEFINED
        )
        return namesp
    except ghidra.util.exception.DuplicateNameException as ex:
        return currentProgram.symbolTable.getNamespace(name, root)


def make_namespaces(path, root=namespace_root):
    for x in path:
        root = create_namespace(root, x)
    return root


# Get functions in ascending order
monitor.initialize(funcmgr.getFunctionCount())
for f in funcmgr.getFunctions(True):
    monitor.incrementProgress(1)
    f_name = f.getName()

    if "::" in f_name:
        continue

    previous_comment = f.getComment()
    if not previous_comment:
        previous_comment = ""

    if previous_comment.startswith("demangled\n"):
        continue

    comment = "demangled\n"+previous_comment+"\n"

    demangled = demangle(f_name)

    comment += "Function: "
    if demangled.is_static:
        comment += "STATIC"
    comment += "\n"

    namespace = make_namespaces(demangled.path)

    comment += "Args: \n"
    for arg in demangled.args:
        comment += "  "+arg+"\n"

    f.setParentNamespace(namespace)
    f.setComment(comment)
    try:
        f.setName(signature, SourceType.ANALYSIS)
    except:
        print(signature)
