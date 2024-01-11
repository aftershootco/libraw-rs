#include "riio.h"
#include <libraw.h>

#ifdef __cplusplus
extern "C" {
#endif

int libraw_open_io(libraw_data_t *libraw,
                              LibrawOpaqueDatastream *io);

#ifdef __cplusplus
}
#endif
