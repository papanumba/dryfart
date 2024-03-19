/* bitarr.h */

#ifndef BITARR_H
#define BITARR_H

struct BitArr {
    uint8_t *buff;
    uint32_t blen;
};

#define bitarr_len(ba) ((ba)->blen / 8)

int bitarr_set(struct BitArr *, uint32_t, int);

#endif /* BITARR_H */
