/* reader.c */

#include <stdlib.h>
#include <unistd.h>
#include <fcntl.h>
#include <sys/mman.h>
#include "reader.h"

static inline void reader_reset(struct Reader *r)
{
    r->buf = NULL;
    r->len = 0;
}

enum ReadRes reader_open(const char *path, struct Reader *r)
{
    if (path == NULL || r == NULL)
        return READRES_ENULL;
    int fd = open(path, O_RDONLY);
    if (fd == -1)
        return READRES_EOPEN;
    size_t file_size = (size_t) lseek(fd, 0, SEEK_END);
    lseek(fd, 0, SEEK_SET);
    cbyte_p buffer = mmap(NULL, file_size, PROT_READ, MAP_PRIVATE, fd, 0);
    if (buffer == MAP_FAILED) {
        close(fd);
        return READRES_EMMAP;
    }
    /* return */
    r->buf = buffer;
    r->len = file_size;
    /* free */
    (void) close(fd); /* local fd, safe to close */
    return READRES_OK;
}

enum ReadRes reader_free(struct Reader *r)
{
    if (r->buf == NULL || r->len == 0)
        return READRES_ENULL;
    if (0 != munmap((void *) r->buf, r->len))
        return READRES_EMMAP;
    reader_reset(r);
    return READRES_OK;
}

const char * readres_what(enum ReadRes rr)
{
    switch (rr) {
      case READRES_OK:    return "OK";
      case READRES_ENULL: return "passed a null pointer or empty file";
      case READRES_EOPEN: return "couldn't open the file";
      case READRES_EMMAP: return "failed mapping file to memory";
      default: unreachable();
    }
}
