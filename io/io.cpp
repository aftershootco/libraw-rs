#include "io.h"
#include "riio.h"
#include <libraw.h>
#include <libraw/libraw_const.h>
#define ID lr->get_internal_data_pointer()->internal_data

class LibrawIO : public LibRaw_abstract_datastream {
public:
  LibrawIO(LibrawOpaqueDatastream *lod) { this->inner = lod; }
  virtual ~LibrawIO() {
    printf("LibrawIO::~LibrawIO()\n");
    lod_drop(this->inner);
  }

  virtual int valid() { return lod_valid(inner); }
  virtual int read(void *ptr, size_t size, size_t nmemb) {
    return lod_read(inner, ptr, size, nmemb);
  }
  virtual int seek(INT64 o, int whence) { return lod_seek(inner, o, whence); }
  virtual INT64 tell() { return lod_tell(inner); }
  virtual INT64 size() { return lod_size(inner); }
  virtual int scanf_one(const char *fmt, void *val) {
    return lod_scanf_one(inner, fmt, val);
  }
  virtual int get_char() { return lod_get_char(inner); }
  virtual char *gets(char *str, int maxlen) {
    return lod_gets(inner, str, maxlen);
  }
  virtual int eof() { return lod_eof(inner); }

  LibrawOpaqueDatastream *inner;
};

/* int libraw_open_io(libraw_data_t *libraw, LibrawOpaqueDatastream *io) { */
/*   LibRaw *lr = (LibRaw *)libraw->parent_class; */
/*   LibrawIO *stream; */
/*   stream = new LibrawIO(io); */
/*   if (!stream || !stream->valid()) { */
/*     delete stream; */
/*     return LIBRAW_IO_ERROR; */
/*   } */
/*   ID.input_internal = 0; // preserve from deletion on error */
/*   int ret = lr->open_datastream(stream); */
/*   if (ret == LIBRAW_SUCCESS) */
/*   { */
/*     ID.input_internal = 1; // flag to delete datastream on recycle */
/*   } */
/*   else */
/*   { */
/*     delete stream; */
/*     ID.input_internal = 0; */
/*   } */
/*   return ret; */
/* } */
int libraw_open_io(libraw_data_t *libraw, LibrawOpaqueDatastream *io) {
  LibRaw *lr = (LibRaw *)libraw->parent_class;
  LibRaw_abstract_datastream *stream;
  stream = new LibrawIO(io);
  if (!stream->valid()) {
    delete stream;
    return LIBRAW_IO_ERROR;
  }
  /* ID.input_internal = 0; // preserve from deletion on error */
  int ret = lr->open_datastream(stream);
  /* if (ret == LIBRAW_SUCCESS) { */
  /*   ID.input_internal = 1; // flag to delete datastream on recycle */
  /* } else { */
  /*   delete stream; */
  /*   ID.input_internal = 0; */
  /* } */
  return ret;
}

int libraw_valid_check(libraw_data_t *libraw) {
  LibRaw *lr = (LibRaw *)libraw->parent_class;
  return ID.input->valid();
}
