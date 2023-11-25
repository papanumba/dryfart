/* values.c */

#include <stdio.h>
#include "values.h"
#include "alzhmr.h"

void values_init(struct Values *v)
{
    v->arr = NULL;
    v->len = 0;
    v->cap = 0;
}

void values_grow(struct Values *v, uint newcap)
{
    size_t new_size = newcap * sizeof(struct DfVal);
    v->arr = realloc_or_free(v->arr, new_size);
    v->cap = newcap;
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
        values_grow(v, new_cap);
    }
    v->arr[v->len] = value;
    v->len++;
}

void values_print(struct DfVal value)
{
    switch (value.type) {
      case VAL_B:
        if (value.as.b)
            printf("T");
        else
            printf("F");
        break;
      case VAL_C:
        printf("%c", value.as.c);
        break;
      case VAL_Z:
        printf("%d", value.as.z);
        break;
      case VAL_R:
        printf("%f", value.as.r);
        break;
      default:
        printf("something went rrong in value.type");
        break;
    }
}
