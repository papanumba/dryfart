/* loader.c */

#include <stdio.h>
#include <string.h>
#include "loader.h"
#include "alzhmr.h"

static void vmdata_init(struct VmData *);

static int check_magic_df(const uint8_t **);
static int load_idf     (struct Idents *, const uint8_t **);
static int load_ctn     (struct Values *, const uint8_t **);
static int load_pag     (struct VmData *, const uint8_t **);
static int load_one_idf (struct Idents *, const uint8_t **);
static int load_one_ctn (struct Values *, const uint8_t **);
static int init_one_pag (struct VmData *, struct Norris *, const uint8_t **);
static void load_val_n  (struct Values *, const uint8_t **);
static void load_val_z  (struct Values *, const uint8_t **);
static void load_val_r  (struct Values *, const uint8_t **);

struct VmData * vmdata_from_dfc(const uint8_t *buff, size_t len)
{
    const uint8_t *rp = buff; /* reading pointer */
    assert(buff != NULL && len != 0);
    struct VmData *vmd = realloc_or_free(NULL, sizeof(struct VmData));
    vmdata_init(vmd);
    if (!check_magic_df(&rp))
        return NULL;
    if (!load_idf(&vmd->idf, &rp))
        return NULL;
    if (!load_ctn(&vmd->ctn, &rp))
        return NULL;
    if (!load_pag(vmd, &rp))
        return NULL;
    return vmd;
}

void vmdata_free(struct VmData *data)
{
    idents_free(&data->idf);
    values_free(&data->ctn);
    norvec_free(&data->pag);
    realloc_or_free(data, 0);
}

static void vmdata_init(struct VmData *data)
{
    idents_init(&data->idf);
    values_init(&data->ctn);
    norvec_init(&data->pag);
}

static int check_magic_df(const uint8_t **rpp)
{
    static uint8_t magic[] = {0xDF, 'D', 'R', 'Y', 'F', 'A', 'R', 'T'};
    const uint8_t *rp = *rpp;
    for (uint i = 0; i < 8; ++i) {
        if (rp[i] != magic[i]) {
            eputln("ERROR: file doesn't have correct magic number");
            return FALSE;
        }
    }
    *rpp += sizeof(magic);
    return TRUE;
}

static int load_idf(struct Idents *idf, const uint8_t **rpp)
{
    uint len = read_u16(rpp);
    idents_w_cap(idf, len);
    for (uint i = 0; i < len; ++i) {
        if (!load_one_idf(idf, rpp))
            return FALSE;
    }
    return TRUE;
}

static int load_ctn(struct Values *ctn, const uint8_t **rpp)
{
    uint len = read_u16(rpp);
    values_w_cap(ctn, len);
    for (uint i = 0; i < len; ++i) {
        if (!load_one_ctn(ctn, rpp))
            return FALSE;
    }
    return TRUE;
}

static int load_pag(struct VmData *vmd, const uint8_t **rpp)
{
    struct NorVec *pag = &vmd->pag;
    uint len = read_u16(rpp);
    norvec_w_cap(pag, len);
    for (uint i = 0; i < len; ++i) {
        struct Norris n;
        if (!init_one_pag(vmd, &n, rpp))
            return FALSE;
        norvec_push(pag, n);
    }
    return TRUE;
}

static int load_one_idf(struct Idents *idf, const uint8_t **rpp)
{
    size_t len = read_u8(rpp);
    if ((*rpp)[len] != '\0') {
        eputln("ERROR: incorrect format in identifiers");
        return FALSE;
    }
    idents_push(idf, dfidf_from_chars((char *)(*rpp), len));
    *rpp += len + 1;
    return TRUE;
}

static int load_one_ctn(struct Values *ctn, const uint8_t **rpp)
{
    uint8_t type = read_u8(rpp);
    switch (type) {
      case VAL_N: load_val_n(ctn, rpp); break;
      case VAL_Z: load_val_z(ctn, rpp); break;
      case VAL_R: load_val_r(ctn, rpp); break;
      default:
        fprintf(stderr, "found constant of type %c\n", (char) type);
        return FALSE;
    }
    return TRUE;
}

static int init_one_pag(
    struct VmData *vmd,
    struct Norris *nor,
    const uint8_t **rpp)
{
    nor->ari = read_u8(rpp);
    nor->lne = read_u32(rpp);
    switch (read_u8(rpp)) {
      case 0x00:
        nor->nam = NULL;
        break;
      case 0xFF:
        nor->nam = &vmd->idf.arr[read_u16(rpp)];
        break;
      default:
        eputln("error: one of pages is not correct format");
        return FALSE;
    }
    size_t len = read_u32(rpp);
    if ((*rpp)[len] != 0) {
        eputln("error: one of pages is not correct format");
        return FALSE;
    }
    norris_cpy_buff(nor, *rpp, len);
    *rpp += len + 1;
    return TRUE;
}

static void load_val_n(struct Values *ctn, const uint8_t **rpp)
{
    struct DfVal val;
    val.type = VAL_N;
    val.as.n = read_u32(rpp);
    values_push(ctn, val);
}

static void load_val_z(struct Values *ctn, const uint8_t **rpp)
{
    struct DfVal val;
    val.type = VAL_Z;
    val.as.z = read_i32(rpp);
    values_push(ctn, val);
}

static void load_val_r(struct Values *ctn, const uint8_t **rpp)
{
    struct DfVal val;
    val.type = VAL_R;
    val.as.r = read_f32(rpp);
    values_push(ctn, val);
}
