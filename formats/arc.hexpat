#pragma endian big
#include <std/mem.pat>
#pragma pattern_limit 200000000

struct String {
  u16 len;
  char string[len];
};

struct MetaEntry {
  String string;
  u32 ptr;
  u32 len;
};

struct File {
  u32 entries;
  MetaEntry metas[entries];
};

File file@ 0x00;