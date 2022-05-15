#pragma endian big
#include <std/mem.pat>
#define UNDEFINED_COLOR cc8888

#pragma pattern_limit 200000000

namespace PNG {
struct header_t
{
    u8 highBitByte;
	char signature[3];
	char dosLineEnding[2];
	char dosEOF;
	char unixLineEnding;
};

struct ihdr_t
{
    u32 width [[comment("Image width")]];
    u32 height [[comment("Image height")]];
    u8 bit_depth;
    u8 color_type [[comment("PNG Image Type\n0: greyscale\n2: truecolour\n3: indexed-color\n4: greyscale with alpha\n6: truecolour with alpha")]];
    u8 compression_method;
    u8 filter_method;
    u8 interlace_method [[comment("values 0 \"no interlace\" or 1 \"Adam7 interlace\"")]];
};

struct palette_entry_t {
    u8 r;
    u8 g;
    u8 b;
};

struct phys_t {
    u32 ppu_x [[comment("Pixels per unit, X axis")]];
    u32 ppu_y [[comment("Pixels per unit, Y axis")]];
    u8 unit [[comment("Unit Specifier\n0: unit is unknown\n1: unit is the metre")]];
};

struct itxt_t {
    char keyword[];
    u8 compression_flag;
    u8 compression_method;
    char language_tag[];
    char translated_keyword[];
    char text[parent.length - ($ - addressof(keyword))];
};

struct iccp_t {
	char profile [];
	u8 compression_method;
	u8 compressed_profile[parent.length - ($ - addressof(profile))];
};

struct chunk_t {
    u32 length [[color("17BECF")]];
    char type[4];

    #define IHDR_k "IHDR"
    #define PLTE_k "PLTE"
    #define pHYs_k "pHYs"
    #define iTXt_k "iTXt"
    #define IDAT_k "IDAT"
    #define IEND_k "IEND"
    #define gAMA_k "gAMA"
    #define iCCP_k "iCCP"

    if (type == IHDR_k) {
        ihdr_t ihdr [[comment("Image Header chunk"), name("IHDR")]];
    } else if (type == PLTE_k) {
        palette_entry_t entries[length / 3];
    } else if (type == pHYs_k) {
        phys_t phys;
    } else if (type == iTXt_k) {
        itxt_t text;
    } else if (type == gAMA_k) {
    	u32 gamma [[name("image gamma"), comment("4 byte unsigned integer representing gamma times 100000")]];
    } else if (type == iCCP_k) {
    	iccp_t iccp;
    } else {
        u8 data[length];
    }

    u32 crc;
};

struct PNG {
header_t header [[comment("PNG file signature"), name("Signature")]];
chunk_t ihdr_chunk [[comment("PNG Header chunk"), name("IHDR")]];
chunk_t chunk_set[while(builtin::std::mem::read_string($ + 4, 4) != "IEND")] [[comment("PNG file chunks"), name("Chunks"), inline]];
chunk_t iend_chunk [[name("IEND"), comment("Image End Chunk")]];
};
}
using PNG = PNG::PNG;



namespace common {
struct WString {
  u16 l [[color("88ccff")]];
  char16 string[l] [[color("8888cc")]];
};

struct String {
  u16 l [[color("88ccff")]];
  char string[l] [[color("8888cc")]];
};
}
using WString = common::WString;
using String = common::String;



namespace locale {
struct LocaleMeta {
  String locale_name;
  u32 string_count [[color("ff88cc")]];
};

struct LocaleDefinetion {
  u32 locDef_1 [[color("cc8888")]];
  u8 locDef_0 [[color("cc8888")]];
  LocaleMeta locale_meta;
  String strings[locale_meta.string_count];
  
};

struct IDS {
  u8 count [[color("ff88cc")]];
  String string[count];
};

struct Locale {
  u32 locale_0 [[color("cc8888")]];
  u32 locale_1 [[color("cc8888")]];
  u32 locales [[color("ff88cc")]];
  u32 locale_2 [[color("cc8888")]];
  IDS ids;
  LocaleMeta metas[locales];
  LocaleDefinetion definetions[locales];
};
}
using Locale = locale::Locale;



namespace meta_database {
struct MetaDatabaseEntry {
  u32 index [[color("cc8888")]];
  u16 pad0 [[color("cc8888")]];
  WString name;
  u32 e_1 [[color("cc8888")]];
};

struct MetaDatabase {
  u32 metas [[color("ff88cc")]];
  MetaDatabaseEntry meta_database[metas];
};
}
using MetaDatabase = meta_database::MetaDatabase;



namespace entry {
struct MetaEntry {
    u32 meta_index; // ref. metadatabase
    u16 meta_ent_0 [[color("cc8888")]];
    WString value;
    u32 meta_ent_1 [[color("cc8888")]];
};

struct MetaDatas {
  u32 metas [[color("ff88cc")]];
  MetaEntry entries[metas];
};

struct EmptyEntry {
  MetaDatas meta;
};

struct PNGEntry {
  PNG data;
  MetaDatas meta;
};


struct LocaleEntry {
  Locale data;
  MetaDatas meta;
};
}
using PNGEntry = entry::PNGEntry;
using LocaleEntry = entry::LocaleEntry;
using EmptyEntry = entry::EmptyEntry;


struct FileInfo {
  u32 filesize;
  u32 e2_0 [[color("cc8888")]];
  u16 e2_1 [[color("cc8888")]];
  WString name;
  u32 e2_2 [[color("cc8888")]];
};

struct File0 {
  u32 file_0 [[color("cc8888")]];
  MetaDatabase meta_database;
  u32 entries;
  FileInfo fileinfos[entries];
  
  EmptyEntry file_0;
  LocaleEntry locale;
  PNGEntry entries[entries - 2];
};

struct File1 {
  u32 file_0 [[color("cc8888")]];
  MetaDatabase meta_database;
  u32 a;
  u32 entries;
  FileInfo fileinfos[entries];
  u32 a;
  
  PNGEntry entry[entries];
};

File file@ 0;