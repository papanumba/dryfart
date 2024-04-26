/* loader.c */

#include <stdexcept>
#include "loader.h"
#include "alzhmr.h"
#include "native.h"
//#include "object.h"

static bool check_magic_df(cbyte_p *);
static void load_idf    (DynArr<DfIdf> &, cbyte_p *);
static void load_ctn    (DynArr<DfVal> &, cbyte_p *);
static void load_pag    (       VmData &, cbyte_p *);
static void load_one_idf(DynArr<DfIdf> &, cbyte_p *);
static void load_one_ctn(DynArr<DfVal> &, cbyte_p *);
static void load_one_pag(       VmData &, cbyte_p *);
static void load_val_c  (DynArr<DfVal> &, cbyte_p *);
static void load_val_n  (DynArr<DfVal> &, cbyte_p *);
static void load_val_z  (DynArr<DfVal> &, cbyte_p *);
static void load_val_r  (DynArr<DfVal> &, cbyte_p *);
static void load_nat_tb (DynArr<DfVal> &, cbyte_p *);
static void load_array  (DynArr<DfVal> &, cbyte_p *);

VmData::VmData(cbyte_p buf, size_t len)
{
    if (buf == NULL || len == 0)
        throw std::runtime_error("Empty buffer");
    cbyte_p rp = buf; /* reading pointer */
    if (!check_magic_df(&rp))
        throw std::runtime_error("Magic number not found");
    load_idf( this->idf, &rp);
    load_ctn( this->ctn, &rp);
    load_pag(*this,      &rp);
    if (rp != buf + len)
        throw std::runtime_error("file size doesn't match");
}

VmData::~VmData()
{
    auto idflen = this->idf.len();
    TIL(i, idflen)
        this->idf[i].~DfIdf();
    auto norlen = this->pag.len();
    TIL(i, norlen)
        this->pag[i].~Norris();
    // no need to delete DfVal
}

static bool check_magic_df(cbyte_p *rpp)
{
#define MAGIC_LEN sizeof(magic)
    static uint8_t magic[] = {0xDF, 'D', 'R', 'Y', 'F', 'A', 'R', 'T'};
    cbyte_p rp = *rpp;
    TIL(i, MAGIC_LEN) {
        if (rp[i] != magic[i])
            return false;
    }
    *rpp += MAGIC_LEN;
    return true;
#undef MAGIC_LEN
}

static void load_idf(DynArr<DfIdf> &idf, cbyte_p *rpp)
{
    uint len = read_u16(rpp);
    idf = DynArr<DfIdf>(len);
    TIL(i, len) load_one_idf(idf, rpp);
}

static void load_ctn(DynArr<DfVal> &ctn, cbyte_p *rpp)
{
    uint len = read_u16(rpp);
    ctn = DynArr<DfVal>(len);
    TIL(i, len) load_one_ctn(ctn, rpp);
}

static void load_pag(VmData &vmd, cbyte_p *rpp)
{
    uint len = read_u16(rpp);
    vmd.pag = DynArr<Norris>(len);
    TIL(i, len) load_one_pag(vmd, rpp);
}

static void load_one_idf(DynArr<DfIdf> &idf, cbyte_p *rpp)
{
    size_t len = read_u8(rpp);
    if ((*rpp)[len] != (uint8_t) '\0')
        throw std::runtime_error("Incorrect format identifier (no \\0)");
    idf.push(DfIdf((*rpp), len));
    *rpp += len + 1;
}

static void load_one_ctn(DynArr<DfVal> &ctn, cbyte_p *rpp)
{
    uint8_t type = read_u8(rpp);
    switch (type) {
      case 0x02: load_val_c(ctn, rpp); break;
      case 0x03: load_val_n(ctn, rpp); break;
      case 0x04: load_val_z(ctn, rpp); break;
      case 0x05: load_val_r(ctn, rpp); break;
      case 0x07: load_nat_tb(ctn, rpp); break;
      case 0x08: load_array(ctn, rpp); break;
      default: throw std::runtime_error("Constant of unknown type\n");
    }
}

static void load_one_pag(VmData &vmd, cbyte_p *rpp)
{
    uint8_t  ari = read_u8(rpp);
    uint8_t  uvs = read_u8(rpp);
    uint32_t lne = read_u32(rpp);
    const DfIdf *nam = nullptr;
    switch (read_u8(rpp)) {
      case 0x00: /* ok NULL */ break;
      case 0xFF:
        nam = &vmd.idf[read_u16(rpp)];
        break;
      default:
        throw std::runtime_error("Incorrect format Norris: Anon. byte");
    }
    size_t len = read_u32(rpp);
    if ((*rpp)[len] != 0)
        throw std::runtime_error("Incorrect format Norris: end \\0");
    vmd.pag.push(Norris(*rpp, len, lne, ari, uvs, nam));
    *rpp += len + 1;
}

static void load_val_c(DynArr<DfVal> &ctn, cbyte_p *rpp)
{
    ctn.push(read_u8(rpp));
}

static void load_val_n(DynArr<DfVal> &ctn, cbyte_p *rpp)
{
    ctn.push(read_u32(rpp));
}

static void load_val_z(DynArr<DfVal> &ctn, cbyte_p *rpp)
{
    ctn.push(read_i32(rpp));
}

static void load_val_r(DynArr<DfVal> &ctn, cbyte_p *rpp)
{
    ctn.push(read_f32(rpp));
}

static void load_nat_tb(DynArr<DfVal> &ctn, cbyte_p *rpp)
{
    todo("load nat tb");
    uint32_t num = read_u32(rpp);
    switch (num) {
      /* mega fall-þru */
      case DF_STD:
      case DF_STD_IO:
      {
/*        struct DfVal v;
        v.type = VAL_O;
        v.as.o = (void *) objtbl_new_nat((enum NatTb) num);
        values_push(ctn, v);*/
        break;
      }
      default:
        throw std::runtime_error("unknown native table");
    }
}

static void load_array(DynArr<DfVal> &ctn, cbyte_p *rpp)
{
    todo("load array");
/*    uint val_type = read_u8(rpp);
    struct DfVal aux;
    switch (val_type) {
      case 0x02: aux.type = VAL_C; break;
      case 0x03: aux.type = VAL_N; break;
      case 0x04: aux.type = VAL_Z; break;
      case 0x05: aux.type = VAL_R; break;
      default:
        fprintf(stderr, "unknown array type %u", val_type);
        return FALSE;
    }
    size_t len = read_u16(rpp);
    struct ObjArr *arr = objarr_new();
    for (size_t i = 0; i < len; ++i) {
        switch (val_type) {
          case 0x02: aux.as.c = read_u8 (rpp); break;
          case 0x03: aux.as.n = read_u32(rpp); break;
          case 0x04: aux.as.z = read_i32(rpp); break;
          case 0x05: aux.as.r = read_f32(rpp); break;
          default: unreachable(); return FALSE;
        }
        objarr_try_push(arr, &aux);
    }
    aux.type = VAL_O;
    aux.as.o = (void *) arr;
    values_push(ctn, aux);
    return TRUE;*/
}
