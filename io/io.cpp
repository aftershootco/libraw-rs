#include "io.h"
#include <libraw.h>

class LibrawIO : public LibRaw_abstract_datastream {
public:
  virtual ~LibrawIO() {}

  virtual int valid() { return lod_valid(inner); }
  virtual int read(void *ptr, size_t size, size_t nmemb) {
    return lod_read(inner, ptr, size, nmemb);
  }
  virtual int seek(INT64 o, int whence) { return lod_seek(inner, o, whence); }
  virtual INT64 tell() { return lod_tell(inner); }
  virtual INT64 size() { return lod_size(inner); }

protected:
  LibrawOpaqueDatastream *inner;
};
