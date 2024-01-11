#include "io.h"
#include "riio.h"
#include <libraw.h>

class LibrawIO : public LibRaw_abstract_datastream {
public:
  LibrawIO(LibrawOpaqueDatastream *lod) { this->inner = lod; }
  virtual ~LibrawIO() {}

  virtual int valid() { return lod_valid(inner); }
  virtual int read(void *ptr, size_t size, size_t nmemb) {
    return lod_read(inner, ptr, size, nmemb);
  }
  virtual int seek(INT64 o, int whence) { return lod_seek(inner, o, whence); }
  virtual INT64 tell() { return lod_tell(inner); }
  virtual INT64 size() { return lod_size(inner); }
  virtual int scanf_one(const char *fmt, void *val) {
    /* return lod_scanf_one(inner); */
    return 0;
  }
  virtual int get_char() { return lod_get_char(inner); }
  virtual char *gets(char *str, int maxlen) {
    return lod_gets(inner, str, maxlen);
  }
  virtual int eof() { return lod_eof(inner); }

  LibrawOpaqueDatastream *inner;
};

extern "C" int libraw_open_io(libraw_data_t *libraw,
                              LibrawOpaqueDatastream *io) {
  LibrawIO* libraw_io = new LibrawIO(io);
  LibRaw *lr = (LibRaw *)libraw->parent_class;

  return lr->open_datastream(libraw_io);
}
