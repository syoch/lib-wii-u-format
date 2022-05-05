import rpx


class WrapperException(Exception):
    pass


class NotFoundException(WrapperException):
    pass


class NotSymbolFound(NotFoundException):
    pass


class NotSectionFound(NotFoundException):
    pass


class Wrapper:
    def __init__(
        self,
        path: str,

    ):
        self.file = rpx.Elf(path)

    def search(self, classname, query):
        if classname:
            classquery = "__"+str(len(classname))+classname
        else:
            classquery = ""

        classquery = classquery.lower()
        query = query.lower()

        found = [
            symbol
            for key, symbol in self.file.symbols.items()
            if query in key.lower() and classquery in key.lower()
        ]

        if len(found) == 0:
            raise NoSymbolFound()

        return sorted(found,  key=lambda x: x.name)

    def get_symbol_by_addr(self, addr: int):
        symbol = [
            symbol
            for (_, symbol) in self.file.symbols.items()
            if symbol.value == addr
        ]
        if len(symbol) == 0:
            raise NotSymbolFound()

        return symbol[0]

    def disasm_by_addr(self, addr: int):
        symbol = self.get_symbol_by_addr(addr)
        return symbol.disasm()

    def get_section_by_address_contain(self, addr: int):
        for section in self.file.headers:
            if section.addr <= addr < section.addr + section.size:
                return section
        raise NotSectionFound()

    def get_symbol_by_addr_contain(self, addr: int):
        symbols = [symbol for (_, symbol) in self.file.symbols.items() if symbol.value <= addr < symbol.value + symbol.size]
        if len(symbols) == 0:
            raise NotSymbolFound()
        return sorted(symbols, key = lambda s: s.size)[0]
