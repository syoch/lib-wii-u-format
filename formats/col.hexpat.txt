#pragma endian big
#include <std/mem.pat>
#define UNDEFINED_COLOR cc8888

#pragma pattern_limit 200000000

struct String {
  u16 len;
  char string[len];
};

struct WString {
  u16 len;
  char16 string[len];
};

struct Entry {
    String name;
    u32 color;
};

struct Entry2 {
    String b;
    u32 water_color_with_opacity;
    u32 water_color;
    u32 e;
};

struct File {
    u32 a;
    
    u32 entry_count;
    Entry entries[entry_count];
    
    u32 b;
    Entry2 entries2[b];
};

File file@0;