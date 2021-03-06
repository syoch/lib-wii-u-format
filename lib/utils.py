def rev_32(x):
    ret = 0
    ret |= ((x >> 0x00) & 0xFF) << 0x18
    ret |= ((x >> 0x08) & 0xFF) << 0x10
    ret |= ((x >> 0x10) & 0xFF) << 0x08
    ret |= ((x >> 0x18) & 0xFF) << 0x00
    return ret


assert rev_32(0x12345678) == 0x78563412, hex(rev_32(0x12345678))
assert rev_32(0x78563412) == 0x12345678, hex(rev_32(0x89563412))

def overlayed(aa, ab, ba, bb):
        # aa ab ba bb
    if ab < ba:
        return None

        # ba bb aa ab
    if bb > aa:
        return None

    return [
        min(aa, ba),
        max(ab, bb)
    ]
