from __future__ import annotations

from typing import Callable, List, Optional
import string
import logging
import sys
logging.basicConfig(
    level=logging.INFO,
    format="%(levelname)7s %(name)20s: %(funcName)30s: %(message)s"
)
logger = logging.getLogger(__name__)


def str_to_bytes_with_length(s: str) -> bytes:
    return len(s).to_bytes(2, 'big') + s.encode()


def list_str_to_bytes(lst: List[str]) -> bytes:
    res = b''
    res += len(lst).to_bytes(2, 'big')
    for s in lst:
        res += str_to_bytes_with_length(s)
    return res


class Type:
    def __init__(self):
        self.suffixes: List[str] = []
        self.prefixes: List[str] = []
        self.basetype: Name = Name()

        self.arguments: Optional[List[Type]] = None
        self.length: int = 1

    def __bool__(self):
        return self.basetype != ""

    def __str__(self):
        ret = str(self.basetype)
        if self.suffixes:
            ret = ret + " " + " ".join(self.suffixes)
        if self.prefixes:
            ret = " ".join(self.prefixes) + " "+ret

        if self.arguments:
            ret += "(*)"
            ret += "("+", ".join(str(a) for a in self.arguments)+")"
        if self.length > 1:
            ret += f"[{self.length}]"
        return ret

    def copy(self):
        ret = Type()
        ret.suffixes = self.suffixes.copy()
        ret.prefixes = self.prefixes.copy()
        ret.basetype = self.basetype.copy()
        if self.arguments:
            ret.arguments = self.arguments.copy()
        ret.length = self.length
        return ret


class Namespace:
    def __init__(self):
        self.path: List[Name] = []

    def __str__(self):
        ret = "::".join([
            str(x)
            for x in self.path
        ])
        return ret

    def copy(self) -> Namespace:
        ret = Namespace()
        ret.path = self.path.copy()
        return ret


class Name:
    def __init__(self):
        self.name: str = ""
        self.namespace: Namespace = Namespace()
        self.template: List[Type] = []

    def __str__(self):
        ret = self.to_str_tail()
        if len(self.namespace.path) > 0:
            ret = "::".join([str(self.namespace), str(ret)])
        return ret

    def to_str_tail(self):
        ret = self.name
        if len(self.template) > 0:
            ret += "<" + ", ".join([str(x) for x in self.template])+">"
        return ret

    def to_type(self) -> Type:
        ret = Type()
        ret.basetype = self
        return ret

    @staticmethod
    def from_str(name):
        ret = Name()
        ret.name = name
        return ret

    def copy(self) -> Name:
        ret = Name()
        ret.name = self.name
        ret.namespace = self.namespace.copy()
        ret.template = self.template.copy()
        return ret


class Function:
    def __init__(self):
        self.name: Name = Name()
        self.args: List[Type] = []
        self.return_type: Type = Type()
        self.type: str = ""
        self.is_static: bool = False

    def __str__(self):
        ret = str(self.name)
        if self.type:
            if len(self.name.namespace.path) != 0:
                ret += self.type.replace("#",
                                         self.name.namespace.path[-1].name)
            else:
                ret += self.type.replace("#", "auto")

        if len(self.args) > 0:
            ret += "("
            ret += ", ".join([str(x) for x in self.args])
            ret += ")"

        if self.return_type:
            ret = str(self.return_type) + " " + ret

        if self.is_static:
            ret = "static " + ret
        return ret

    def to_type(self) -> Type:
        ret = self.return_type.copy()
        ret.arguments = self.args
        return ret


def flatten(lst):
    return [
        item
        for sublist in lst
        for item in sublist
    ]


class DemangleFailure(Exception):
    pass


class DemangleSession:
    def __init__(self, src: str):
        self.templates: List[List[Type]] = []
        self.reminder: str = src
        self.uncompressed: str = src
        self.src: str = src

        self.decompress()

    def update_source(self, src: str):
        self.reminder: str = src
        self.uncompressed: str = src
        self.src: str = src

    def decompress_CPR(self):
        decompressed_size = self.read_int()
        self.skip(2)  # skip __
        compressed = self.reminder[:]
        ret = ""
        for i, tok in enumerate(compressed.split("J")):
            if i % 2 == 1 and tok:
                offset = int(tok)
                self.reminder = ret[offset:]
                name = self.read_string()
                tok = str(len(name)) + name
            elif i % 2 == 1 and not tok:
                tok = "J"

            ret += tok
        if len(ret) != decompressed_size:
            print("WARNING: decompressed size mismatch")

        self.uncompressed = ret
        self.reminder = ret

    def decompress_thunk(self):
        self.expect("__ghs_thunk__")
        self.skip(len("0xffffff70__"))

    def decompress(self):
        if self.consume("__ghs_thunk__"):
            self.skip(12)
        if self.consume("__CPR"):
            self.decompress_CPR()
        self.src = self.reminder

    def consume(self, val: str, skip: bool = True) -> bool:
        ret = self.reminder.startswith(val)
        if ret and skip:
            self.skip(len(val))
        return ret

    def skip(self, size: int) -> bool:
        if len(self.reminder) < size:
            return False
        self.reminder = self.reminder[size:]
        return True

    def read(self, size: int) -> str:
        ret = self.reminder[:size]
        self.reminder = self.reminder[size:]
        return ret

    def expect(self, target: str) -> bool:
        if self.reminder[:len(target)] != target:
            self.error("expected %s" % target)
        self.skip(len(target))

    def peek(self, size: int) -> str:
        return self.reminder[:size]

    def has_data(self) -> bool:
        return bool(self.reminder)

    def read_until(
            self,
            target: str | Callable[[DemangleSession], bool]
    ) -> str:
        if isinstance(target, str):
            def target(x): return x.consume(target)
        ret = ""
        while True:
            if target(self):
                break
            if not self.has_data():
                break

            ret += self.read(1)
        return ret

    def read_int(self) -> int:
        j = 0
        for i, ch in enumerate(self.reminder):
            if ch in string.digits:
                j += 1
            else:
                break
        return int(self.read(j))

    def read_string(self) -> str:
        len = self.read_int()
        return self.read(len)

    def startswith_digit(self) -> bool:
        if not self.has_data():
            return False
        return self.reminder[0] in string.digits

    def __bool__(self):
        return self.has_data()

    def error(
            self,
            message: str = "",
            fatal: bool = False,
            ex: Exception = None):
        if not fatal:
            raise DemangleFailure(message)

        if not message and ex:
            message = str(ex)

        print("--------------------")
        if message:
            print("Error: %s" % message)
            print("")
        self.debug()

        if not ex:
            try:
                raise Exception()
            except Exception as ex_:
                ex = ex_

        print()
        print("trace back:")
        tb = ex.__traceback__
        while tb:
            frame = tb.tb_frame
            print("  %s:%s <%s>" % (
                frame.f_code.co_filename, frame.f_lineno,
                frame.f_code.co_name,
            ))
            tb = tb.tb_next

        if fatal:
            exit(1)

    def debug(self):
        print("raw      = %s" % self.uncompressed)
        print("src      = %s" % self.src)
        print("reminder = %s" % (
            " "*self.src.find(self.reminder) +
            self.reminder)
        )


name_prefixes = {
    "__vtbl": " virtual table",
    "__ct":   "#",
    "__dt":   "~#",
    "__as":   "operator=",
    "__eq":   "operator==",
    "__ne":   "operator!=",
    "__gt":   "operator>",
    "__lt":   "operator<",
    "__ge":   "operator>=",
    "__le":   "operator<=",
    "__pp":   "operator++",
    "__pl":   "operator+",
    "__apl":  "operator+=",
    "__mi":   "operator-",
    "__ami":  "operator-=",
    "__ml":   "operator*",
    "__amu":  "operator*=",
    "__dv":   "operator/",
    "__adv": "operator/=",
    "__nw":  "operator new",
    "__dl":  "operator delete",
    "__vn":  "operator new[]",
    "__vd":  "operator delete[]",
    "__md":  "operator%",
    "__amd": "operator%=",
    "__mm":  "operator--",
    "__aa":  "operator&&",
    "__oo":  "operator||",
    "__or":  "operator|",
    "__aor": "operator|=",
    "__er":  "operator^",
    "__aer": "operator^=",
    "__ad":  "operator&",
    "__aad": "operator&=",
    "__co":  "operator~",
    "__cl":  "operator",
    "__ls":  "operator<<",
    "__als": "operator<<=",
    "__rs":  "operator>>",
    "__ars": "operator>>=",
    "__rf":  "operator->",
    "__vc":  "operator[]",
}
basetypes = {
    'v': "void",
    'i': "int",
    's': "short",
    'c': "char",
    'w': "wchar_t",
    'b': "bool",
    'f': "float",
    'd': "double",
    'l': "long",
    'L': "long long",
    'e': "...",
    'r': "long double",
}
prefixes = {
    'U': "unsigned",
    'S': "signed",
    'J': "__complex",
    'M': '[M]'
}
suffixes = {
    'P': "*",
    'R': "&",
    'C': "const",
    'V': "volatile",
    'u': "restrict",
}


class Demangler:
    def __init__(self, src=""):
        self.session: DemangleSession = DemangleSession(src)

    def parse_template(self, src: str) -> List[Type]:  # parses tm__4_w2ab
        parser = Demangler(src)

        parser.session.expect("tm__")
        parser.session.update_source(parser.session.read_string()[1:])

        ret = parser.read_types()
        self.session.templates.append(ret)
        return ret

    def read_template(self) -> List[Type]:  # reads tm__4_w2ab
        logger.debug("%s" % self.session.reminder)

        self.session.expect("tm__")
        template = self.session.read_string()[1:]

        ret = Demangler(template).read_types()
        self.session.templates.append(ret)
        return ret

    def read_string(self) -> Name:
        ret = Name()
        ret.name = self.session.read_string()
        if "tm__" in ret.name:
            raw_template = ret.name[ret.name.find("tm__"):]
            ret.template = self.parse_template(raw_template)
            ret.name = ret.name[:ret.name.find("tm__")-2]
        return ret

    def read_class_ref(self) -> Type:
        logger.debug("%s" % self.session.reminder)

        ret = Type()
        self.session.expect("Z")
        _ = self.session.read_int()
        if self.session.consume("_"):
            self.session.error("Z#_#Z is not supported", fatal=False)
        self.session.expect("Z")
        ret.basetype.name = "char"
        # ret.basetype = flatten(self.session.templates)[no-1]
        return ret

    def read_type(self) -> Type:
        logger.debug("%s", self.session.reminder)

        first = self.session.peek(1)
        if first in prefixes:
            self.session.skip(1)

            ret = self.read_type()
            ret.prefixes.append(prefixes[first])
            return ret
        elif first in suffixes:
            self.session.skip(1)

            ret = self.read_type()
            ret.suffixes.append(suffixes[first])
            return ret
        elif first in basetypes:
            self.session.skip(1)
            ret = Type()
            ret.basetype.name = basetypes[first]
            return ret
        elif first in string.digits:
            ret = Type()
            ret.basetype = self.read_string().to_type()
            return ret
        elif first == "Q":
            ret = Type()

            tmp = self.read_namespace()
            ret.basetype = tmp.path[-1]
            ret.basetype.namespace.path = tmp.path[:-1]
            return ret
        elif first == "Z":
            return self.read_class_ref()
        elif first == "F":
            return self.read_funcinfo().to_type()
        elif first == "A":
            ret = Type()
            self.session.expect("A")
            if self.session.consume("_Z", skip=False):
                raise DemangleFailure()
            ret.length = self.session.read_int()
            self.session.expect("_")
            ret.basetype = self.read_type()
            return ret

        self.session.error("unknown type %s" % first)

        ret = Type()
        ret.basetype.name = "undefined"
        return ret

    def read_types(self) -> List[Type]:
        logger.debug("%s", self.session.reminder)

        ret = []
        while self.session and not self.session.consume("_", skip=False):
            if self.session.consume("T"):
                ret.append(ret[int(self.session.read(1))-1])
            elif self.session.consume("N"):
                count = int(self.session.read(1))
                index = int(self.session.read(1))
                ret += [ret[index-1]] * count
            elif self.session.consume("X"):
                if self.session.startswith_digit():
                    self.session.debug()
                    tmp = Type()
                    tmp.basetype.name = self.session.read_string()
                    ret.append(tmp)
                else:
                    t = self.read_type()
                    t.basetype.name = t.basetype.name.replace("#", " #")
                    val = ""
                    if self.session.consume("L"):
                        self.session.expect("_")
                        length = self.session.read_int()
                        self.session.expect("_")
                        val = self.session.read(length)
                    elif self.session.has_data():
                        val = self.session.reminder
                        self.session.reminder = ""
                    _ = val
            else:
                ret.append(self.read_type())
        return ret

    def read_name(self) -> Name:
        logger.debug("%s", self.session.reminder)

        ret = Name()
        if self.session.peek(1) in string.digits:
            ret = self.read_string()

        elif self.session.peek(1) == "Q":
            tmp = self.read_namespace()
            ret.name = tmp.path[-1]
            ret.namespace.path = tmp.path[0: -1]
        else:
            self.session.error("unknown name prefix type %s" %
                               self.session.peek(1))

        if self.session.consume("tm__"):
            ret.template = self.read_template()

        return ret

    def read_namespace(self) -> Namespace:
        logger.debug("%s", self.session.reminder)

        ret = Namespace()
        self.session.expect("Q")
        path_len = self.session.read_int()
        self.session.expect("_")
        for i in range(path_len):
            if self.session.consume("Z", skip=False):
                ret.path.append(self.read_class_ref().basetype)
                continue
            ret.path.append(self.read_name())
        return ret

    def read_funcinfo(self, obj: Function = None) -> Function:
        logger.debug("%s", self.session.reminder)

        if obj is None:
            obj = Function()

        if self.session.consume("F"):
            obj.args = self.read_types()

        if self.session.consume("_"):
            obj.return_type = self.read_type()

        return obj

    def read_name_prefix(self, obj: Function = None) -> Function:
        if obj is None:
            obj = Function()

        logger.debug(f"reminder = {self.session.reminder}")
        for prefix in name_prefixes:
            logger.debug("checking name prefix for {}".format(prefix))
            if self.session.reminder.startswith(prefix):
                logger.debug("match name prefix")
                self.session.skip(len(prefix))
                obj.type = name_prefixes[prefix]
                break

        return obj

    def read_function(self) -> Function:
        logger.debug("%s", self.session.reminder)
        ret = Function()

        ret = self.read_name_prefix(ret)

        logger.debug("reading name")
        logger.debug(f"reminder = {self.session.reminder}")
        ret.name.name = self.session.read_until(
            lambda session:
                session.consume("__", skip=False) and (
                    session.peek(3)[2] in string.digits or
                    session.peek(3)[2] == "Q" or
                    session.peek(3)[2] == "F" or
                    session.peek(4)[2:4] == "tm"
                ) and session.consume("__", skip=True)
        )
        if not self.session:
            return ret
        logger.debug(self.session.reminder)
        if self.session.consume("tm__", skip=False):
            ret.name.template = self.read_template()

        logger.debug("skipping some junk charactors")
        logger.debug("reminder = "+self.session.reminder)
        ch = self.session.peek(1)
        while not (ch == "Q" or ch in string.digits or ch == "F"):
            logger.debug("skipped: "+self.session.peek(1))
            self.session.skip(1)
            ch = self.session.peek(1)
        logger.debug("skipping some junk charactors - end")

        logger.debug("reading namespace")
        if self.session.consume("Q", skip=False):
            ret.name.namespace = self.read_namespace()
        elif self.session.peek(1) in string.digits:
            ret.name.namespace.path.append(self.read_string())
        else:
            logger.debug("function without namespace")
            logger.debug("reminder = " + self.session.reminder)

        self.session.consume("C")
        if self.session.consume("S"):
            ret.is_static = True

        logger.debug("reading function info")
        if self.session.consume("F", skip=False):
            ret = self.read_funcinfo(ret)
        else:
            logger.debug("function without function info")
            logger.debug("reminder = " + self.session.reminder)

        if self.session.has_data():
            self.session.error("enough data")

        return ret

    def demangle(self, src: str) -> Function:
        self.session = DemangleSession(src)
        try:
            return self.read_function()
        # except DemangleFailure as ex:
        #     self.session.error(ex=ex, fatal=False)
        except Exception:
            pass
        return src


def main():
    global logger
    logger.setLevel(logging.DEBUG)
    demangler = Demangler()
    with open("../functions", "r") as fp:
        lst = []
        for line in fp:
            line = line.strip()[9:]  # strip leading 'xxxxxxxx '
            demangled_name = demangler.demangle(line)

            if isinstance(demangled_name, str):
                lst.append("00000000 "+line)
            else:
                print("demangled=", demangled_name)

    if not lst:
        return

    with open("../functions", "w") as fp:
        for line in lst:
            fp.write(line + "\n")


def run(
    remove_ret_type=False,
    remove_argv=False,
    as_binary=False
):
    demangler = Demangler()
    logger.setLevel(logging.INFO)
    while True:
        x = input()
        if x.startswith("____"):
            x = x[2:]

        try:
            ret = demangler.demangle(x)
        except:
            ret = x

        if type(ret) == str:
            print(ret)
            continue

        if remove_ret_type:
            ret.return_type = None

        if remove_argv:
            ret.args = []

        if not as_binary:
            print(ret)
            continue

        res = b''
        res += ret.is_static.to_bytes(1, 'little')

        res += list_str_to_bytes([
            *[str(x) for x in ret.name.namespace.path],
            ret.name.to_str_tail()
        ])

        res += list_str_to_bytes([str(x) for x in ret.args])

        res += str_to_bytes_with_length(
            str(ret.return_type)
        )

        sys.stdout.buffer.write(res)


if __name__ == "__main__":
    remove_ret_type = False
    remove_argv = False
    as_binary = False

    for modeflag in sys.argv[1]:
        if modeflag == "r":
            remove_ret_type = True
        elif modeflag == "a":
            remove_argv = True
        elif modeflag == "b":
            as_binary = True

    run(
        remove_ret_type=remove_ret_type,
        remove_argv=remove_argv,
        as_binary=as_binary
    )
