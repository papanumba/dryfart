/* garcol.c */

#include <stdio.h>
#include <stdlib.h>
#include "garcol.h"
#include "falloc.h"
#include "alzhmr.h"
#include "virmac.h"
#include "htable.h"

static struct Object **grey_stack = NULL;
static size_t grey_len = 0;
static size_t grey_cap = 0;

static void mark_roots(struct VirMac *);
static void mark_dfval(struct DfVal *);
static void mark_htable(struct Htable *);
static void mark_object(struct Object *);
static void trace_refs(void);
static void blacken_obj(struct Object *);

static void grey_push(struct Object *);
static struct Object *grey_pop(void);
static void grey_grow(size_t);


void garcol_init(void)
{
    grey_stack = NULL;
    grey_len = 0;
    grey_cap = 0;
}

void garcol_do(struct VirMac *vm)
{
#ifdef DEBUG
    puts("garcol starts");
#endif
    mark_roots(vm);
    trace_refs();
    falloc_sweep();
#ifdef DEBUG
    puts("garcol ends");
#endif
}

void garcol_exit(void)
{
    if (grey_stack != NULL)
        free(grey_stack);
    garcol_init();
}

/* marks globals & stack */
static void mark_roots(struct VirMac *vm)
{
    for (struct DfVal *v = vm->stack; v != vm->sp; ++v)
        mark_dfval(v);
}

static void mark_dfval(struct DfVal *v)
{
    if (v->type != VAL_O)
        return;
    mark_object(v->as.o);
#ifdef DEBUG
    puts("GC marked: ");
    values_print(v);
    puts("");
#endif
}

static void mark_htable(struct Htable *t)
{
    for (size_t i = 0; i < t->cap; ++i)
        mark_dfval(&t->ent[i].v);
}

/* mark as "grey", i.e. push to grey & set mark TRUE */
static void mark_object(struct Object *obj)
{
    if (obj == NULL || obj->gc_mark)
        return;
    obj->gc_mark = TRUE;
    grey_push(obj);
}

static void trace_refs(void)
{
    while (grey_len > 0) {
        struct Object *obj = grey_pop();
        blacken_obj(obj);
    }
}

/* mark neibrhood */
static void blacken_obj(struct Object *obj)
{
#ifdef DEBUG
    printf("blackening ");
    object_print(obj);
    puts("");
#endif
    switch (obj->type) {
      case OBJ_ARR:
        return;
      case OBJ_TBL:
        mark_htable(&OBJ_AS_TBL(obj)->tbl);
        return;
      case OBJ_PRO:
        /* FUTURE: mark upvalues */
        return;
    }
}


/********************* G R E Y   S T A C K   S T U F F ********************/

static void grey_push(struct Object *o)
{
    if (grey_cap < grey_len + 1)
        grey_grow(GROW_CAP(grey_cap));
    grey_stack[grey_len++] = o;
}

/* returns NULL if empty */
static struct Object *grey_pop(void)
{
    if (grey_len == 0)
        return NULL;
    else
        return grey_stack[--grey_len];
}

static void grey_grow(size_t newcap)
{
    size_t new_size = newcap * sizeof(struct Object *);
    grey_stack = realloc_or_free(grey_stack, new_size);
    grey_cap = newcap;
}
