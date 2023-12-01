/* object.c */

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include "object.h"

static int objstr_eq(struct ObjStr *, struct ObjStr *);

void object_print(struct Object *obj)
{
    switch (obj->type) {
      case OBJ_STR:
        puts(((struct ObjStr *)obj)->str);
        break;
    }
}

int object_eq(struct Object *o0, struct Object *o1)
{
    if (o0->type != o1->type)
        return FALSE;
    switch (o0->type) {
      case OBJ_STR:
        return objstr_eq((struct ObjStr *)o0, (struct ObjStr *)o1);
    }
    return FALSE;
}

/*struct ObjStr objstr_from_chars(const char *str, size_t len)
{
    struct ObjStr objstr;
    char *newstr;
    newstr = calloc(sizeof(char), len + 1);
    memcpy(newstr, str, len);
    objstr.obj.type = OBJ_STR;
    objstr.len = len;
    objstr.str = newstr;
    return objstr;
}*/

static int objstr_eq(struct ObjStr *s0, struct ObjStr *s1)
{
    if (s0->len == s1->len)
        return memcmp(s0->str, s1->str, s0->len) == 0;
    return FALSE;
}

/*static struct ObjStr * objstr_new(char *str, size_t len, uint hsh)
{
    hast_
}
*/
