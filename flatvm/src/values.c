/* values.c */

#include <stdio.h>
#include "values.h"
#include "object.h"
#include "alzhmr.h"

static void grow(struct Values *, uint);

void values_init(struct Values *v)
{
    v->arr = NULL;
    v->len = 0;
    v->cap = 0;
}

void values_free(struct Values *v)
{
    for (size_t i = 0; i < v->len; ++i) {
        if (v->arr[i].type == VAL_O)
            object_free(v->arr[i].as.o);
    }
    realloc_or_free(v->arr, 0);
    values_init(v); /* set all to 0 */
}

void values_push(struct Values *v, struct DfVal value)
{
    if (v->cap < v->len + 1)
        grow(v, GROW_CAP(v->cap));
    v->arr[v->len] = value;
    v->len++;
}

int values_eq(struct DfVal *v, struct DfVal *w)
{
    if (v->type != w->type)
        return FALSE;
    switch (v->type) {
      case VAL_V: return TRUE;
      case VAL_B: return !!v->as.b == !!v->as.b; /* for oþer non-0 values */
      case VAL_C: return v->as.c == w->as.c;
      case VAL_N: return v->as.n == w->as.n;
      case VAL_Z: return v->as.z == w->as.z;
      case VAL_R: return FALSE;
      case VAL_O: return object_eq(v->as.o, w->as.o);
      default:
        fputs("unknown type in values_eq\n", stderr);
        return FALSE;
    }
}

void values_print(const struct DfVal *value)
{
    switch (value->type) {
      case VAL_V: fputs("Void", stdout);         break;
      case VAL_B: putchar(value->as.b?'T':'F');  break;
      case VAL_C: putchar(value->as.c);          break;
      case VAL_N: printf("%luu", (ulong)value->as.n);     break;
      case VAL_Z: printf("%ld",   (long)value->as.z);     break;
      case VAL_R: printf("%f", value->as.r);     break;
      case VAL_O: object_print(value->as.o);     break;
      default:
        fprintf(stderr, "unknown value.type %d\n", value->type);
        break;
    }
}

enum DfType val2type(const struct DfVal *v)
{
    enum DfType t = DFTYPE_V;
    switch (v->type) {
        case VAL_V: t = DFTYPE_V; break;
        case VAL_B: t = DFTYPE_B; break;
        case VAL_C: t = DFTYPE_C; break;
        case VAL_N: t = DFTYPE_N; break;
        case VAL_Z: t = DFTYPE_Z; break;
        case VAL_R: t = DFTYPE_R; break;
        case VAL_O: t = object_get_type(v->as.o); break;
    }
    return t;
}

static void grow(struct Values *v, uint newcap)
{
    size_t new_size = newcap * sizeof(struct DfVal);
    v->arr = realloc_or_free(v->arr, new_size);
    v->cap = newcap;
}
