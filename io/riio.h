#include<stdint.h>

#ifndef RUST_IO_H
#define RUST_IO_H



typedef struct LibrawOpaqueDatastream LibrawOpaqueDatastream;

int32_t lod_valid(struct LibrawOpaqueDatastream *this_);

int32_t lod_read(struct LibrawOpaqueDatastream *this_,
                 const void *buffer,
                 uintptr_t sz,
                 uintptr_t nmemb);

int32_t lod_seek(struct LibrawOpaqueDatastream *this_, int64_t offset, uint32_t whence);

int64_t lod_tell(struct LibrawOpaqueDatastream *this_);

int32_t lod_eof(struct LibrawOpaqueDatastream *this_);

int64_t lod_size(struct LibrawOpaqueDatastream *this_);

int lod_get_char(struct LibrawOpaqueDatastream *this_);

char *lod_gets(struct LibrawOpaqueDatastream *this_, char *buffer, int size);

#endif /* RUST_IO_H */
