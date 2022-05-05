class Item:
    def __init__(self, offset):
        global msscmp
        (
            folder_name_ptr,
            info_ptr
        ) = unpack("II", msscmp[offset:offset+0x8])

        self.folder_name = read_string(folder_name_ptr)
        self.info = Info(info_ptr)

class Info:
    def __init__(self, info_ptr):
        global msscmp
        (
            folder_name_ptr,
            file_name_ptr,  # + info_ptr
            offset,
            self.unk_1,

            ffff_ffff,
            self.sampling_rate,
            size,
            self.unk_2,

            null_0001,
            self.unk_3,
            null_0002, null_0003,

            null_0004,
            self.playrate,
            null_0005,
        ) = unpack("IIII IIII IIII IfI", msscmp[info_ptr:info_ptr+0x3C])

        self.folder_name = read_string(folder_name_ptr)
        self.file_name = read_string(file_name_ptr + info_ptr)

        offset = rev_32(offset)
        self.data = msscmp[offset:offset+size]

        self.offset = offset
        self.size = size

    def path(self) -> str:
        return self.folder_name + "/" + self.file_name
