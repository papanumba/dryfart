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
    uint i;
    for (i = 0; i < v->len; ++i) {
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

void values_print(struct DfVal *value)
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

char valt2char(enum ValType t)
{
    switch (t) {
        case VAL_V: return 'V';
        case VAL_B: return 'B';
        case VAL_C: return 'C';
        case VAL_N: return 'N';
        case VAL_Z: return 'Z';
        case VAL_R: return 'R';
        case VAL_O: return 'O';
        default:
            fprintf(stderr, "unknown type %x\n", t);
            return '\0';
    }
}

static void grow(struct Values *v, uint newcap)
{
    size_t new_size = newcap * sizeof(struct DfVal);
    v->arr = realloc_or_free(v->arr, new_size);
    v->cap = newcap;
}
