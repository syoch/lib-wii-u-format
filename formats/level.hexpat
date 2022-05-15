#pragma endian big
#include <std/mem.pat>
#define UNDEFINED_COLOR cc8888

#pragma pattern_limit 200000000

struct WString {
  u16 l [[color("88ccff")]];
  char16 string[l] [[color("8888cc")]];
};

struct String {
  u16 l [[color("88ccff")]];
  char string[l] [[color("8888cc")]];
};

struct NBT {
  u8 type [[color("ffcc88")]];
  String name;
  
  if (type == 0x03) {
    u32 value [[color("88cc00")]];
  } else if (type == 0x01) {
    u8 value [[color("88cc00")]];
  } else if (type == 0x04) {
    u64 value [[color("88cc00")]];
  } else if (type == 0x08) {
    String value [[color("88cc00")]];
  } else {
    u64 a[10];
  }
};

struct DB1 {
  char16 s[64] [[color("8888cc")]];
  u32 size;
  u8 *ptr: u32;
  u32 a;
  u32 b;
};

struct FileTag {
  /* NBT */
  u8 _0a [[color("ffcc88")]];
  String name;

  u8 _0a [[color("ffcc88")]];
  String name;
  NBT tags[0x22];
  u8 terminator;
};

struct File {
  DB1 *a: u32;
  u32 entries [[color("88ccff")]];
  u32 c [[color("cc8888")]];
};

File file@ 0;
DB1 db1[file.entries]@ 0xe21f29;

u8 level_dat[0x00000268]@ 0x0c;
u8 dimH1r_m1_m1[0x00000000]@ 0x00000274;
u8 dimH1r_p0_m1[0x0004DFC6]@ 0x00000274;
u8 dimH1r_p0_p0[0x00000000]@ 0x0004e23a;
u8 dimH1r_m1_p0[0x00000000]@ 0x0004e23a;
u8 dim1Sr_m1_m1[0x0005232B]@ 0x0004e23a;
u8 dim1Sr_p0_m1[0x0005234B]@ 0x000a0565;
u8 dim1Sr_p0_p0[0x00052383]@ 0x000f28b0;
u8 r_m1_m1[0x00309fd9]@ 0x00196f6C;
u8 r_p0_m1[0x0031CFFE]@ 0x004A0F45;
u8 r_p0_p0[0x00351DEC]@ 0x007BDF43;
u8 r_m1_p0[0x00311FFA]@ 0x00B0FF2F;