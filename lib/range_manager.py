from wiiu.utils import overlayed

from typing import List, Tuple


Range = Tuple[int, int]

class RangeManager:
    def __init__(self, ranges=[]):
        self.ranges: List[Range] = ranges

    def add(self, r: Range):
        self.ranges.append(r)

    def combine_neighboring_ranges_once(self):
        for a in self.ranges:
            for b in self.ranges:
                if a[1] == b[0]:
                    self.ranges.remove(a)
                    self.ranges.remove(b)
                    self.ranges.append([a[0], b[1]])
                    return True
        return False

    def combine_overlayed_ranges_once(self):
        for a in self.ranges:
            for b in self.ranges:
                ret = overlayed(a[0], a[1], b[0], b[1])
                if ret is not None:
                    self.ranges.remove(a)
                    self.ranges.remove(b)
                    self.ranges.append(ret)
                    return True
        return False

    def combine_ranges(self):
        while self.combine_overlayed_ranges_once() \
            | self.combine_neighboring_ranges_once():
            pass

    def get_inverted(self):
        ret = []
        for r in self.ranges:
            ret.append([r[1], r[0]])

        return RangeManager(ret)

    def sort_ranges(self):
        self.ranges.sort(key=lambda r: r[0])