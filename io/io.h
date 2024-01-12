#include "riio.h"
#include <libraw.h>

#ifdef __cplusplus
extern "C" {
#endif

int libraw_open_io(libraw_data_t *libraw,
                              LibrawOpaqueDatastream *io);

int libraw_valid_check(libraw_data_t *libraw);
#ifdef __cplusplus
}
#endif
