/* main.c */

/*#define _POSIX_C_SOURCE 1*/

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include "common.h"
#include "norris.h"
#include "virmac.h"

static void run_file(struct VirMac *vm, const char *);
static struct Norris read_file_to_norris(const char *);
static void run_example(struct VirMac *vm);

int main(int argc, const char *argv[])
{
    struct VirMac vm;
    virmac_init(&vm);
    switch (argc) {
      case 1: /* example */
        run_example(&vm);
        break;
      case 2: /* <exefile> <bytecode> */
        run_file(&vm, argv[1]);
        break;
      default:
        fprintf(stderr, "U idiot, provide a signle file to be run\n");
        virmac_free(&vm);
        return 1;
    }
    virmac_free(&vm);
    return 0;
}

static void run_example(struct VirMac *vm)
{
    uint i;
    enum OpCode c[] = {
        OP_LZ1, OP_LZ1, OP_SUB, OP_LZ0, OP_CNE, OP_NOT, OP_LBT, OP_AND, OP_RET
    };
    struct Norris code;
    enum ItpRes res;
    norris_init(&code);
    for (i = 0; i < 9; ++i)
        norris_push_byte(&code, c[i]);
    res = virmac_run(vm, &code);
    norris_free(&code);
    switch (res) {
      case ITP_OK:
        printf("all ok\n");
        break;
      case ITP_RUNTIME_ERR:
        fputs("Der'z bin a runtime error\n", stderr);
        exit(1);
      default:
        fputs("some unknown error from virmac_run\n", stderr);
        break;
    }
}

static void run_file(struct VirMac *vm, const char *path)
{
    struct Norris source;
    enum ItpRes res;
    source = read_file_to_norris(path);
    res = virmac_run(vm, &source);
    norris_free(&source);
    switch (res) {
      case ITP_OK:
        printf("all ok\n");
        break;
      case ITP_RUNTIME_ERR:
        fprintf(stderr, "Der'z bin a runtime error\n");
        exit(1);
      default:
        fprintf(stderr, "some error from virmac_run\n");
        break;
    }
}

static struct Norris read_file_to_norris(const char *path)
{
    uint i;
    size_t file_size, bytes_read;
    uchar *buffer = NULL;
    FILE *file = NULL;
    struct Norris res;
    file = fopen(path, "rb");
    if (file == NULL) {
        fprintf(stderr, "ERROR@read_file: opening file %s\n", path);
        exit(1);
    }
    fseek(file, 0L, SEEK_END);
    file_size = ftell(file);
    rewind(file);
    buffer = malloc(file_size + 1);
    if (buffer == NULL) {
        fprintf(stderr, "ERROR@read_file: mallocating buffer\n");
        exit(1);
    }
    bytes_read = fread(buffer, sizeof(uchar), file_size, file);
    if (bytes_read < file_size) {
        fprintf(stderr, "ERROR@read_file: could not read file %s\n", path);
        exit(1);
    }
    /* copy to norris */
    norris_init(&res);
    for (i = 0; i < bytes_read; ++i)
        norris_push_byte(&res, buffer[i]);
    fclose(file);
    free(buffer);
    return res;
}
