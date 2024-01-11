#include<stdint.h>

#ifndef RUST_IO_H
#define RUST_IO_H



struct LibrawOpaqueDatastream;

extern "C" {

int32_t lod_valid(LibrawOpaqueDatastream *this_);

int32_t lod_read(LibrawOpaqueDatastream *this_, const void *buffer, uintptr_t sz, uintptr_t nmemb);

int32_t lod_seek(LibrawOpaqueDatastream *this_, int64_t offset, uint32_t whence);

int64_t lod_tell(LibrawOpaqueDatastream *this_);

int32_t lod_eof(LibrawOpaqueDatastream *this_);

int64_t lod_size(LibrawOpaqueDatastream *this_);

int lod_get_char(LibrawOpaqueDatastream *this_);

char *lod_gets(LibrawOpaqueDatastream *this_, char *buffer, int size);

int lod_scanf_one(LibrawOpaqueDatastream *this_, const char *fmt, void *val);

} // extern "C"

#endif // RUST_IO_H
