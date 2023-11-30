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
    realloc_or_free(v->arr, 0);
    values_init(v); /* set all to 0 */
}

void values_push(struct Values *v, struct DfVal value)
{
    if (v->cap < v->len + 1) {
        uint new_cap = GROW_CAP(v->cap);
        grow(v, new_cap);
    }
    v->arr[v->len] = value;
    v->len++;
}

void values_print(struct DfVal value)
{
    switch (value.type) {
      case VAL_V: fputs("Void", stdout);        break;
      case VAL_B: putchar(value.as.b?'T':'F');  break;
      case VAL_C: putchar(value.as.c);          break;
      case VAL_N: printf("%u", value.as.n);     break;
      case VAL_Z: printf("%d", value.as.z);     break;
      case VAL_R: printf("%f", value.as.r);     break;
      case VAL_O: object_print(value.as.o);     break;
      default:
        printf("something went rrong in value.type");
        break;
    }
}

char values_type_to_char(enum ValType t)
{
    switch (t) {
        case VAL_V: return 'V';
        case VAL_B: return 'B';
        case VAL_C: return 'C';
        case VAL_N: return 'N';
        case VAL_Z: return 'Z';
        case VAL_R: return 'R';
        case VAL_O: return 'O';
        default: return '\0';
    }
}

static void grow(struct Values *v, uint newcap)
{
    size_t new_size = newcap * sizeof(struct DfVal);
    v->arr = realloc_or_free(v->arr, new_size);
    v->cap = newcap;
}

