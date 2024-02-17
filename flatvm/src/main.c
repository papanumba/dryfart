/* main.c */

/*#define _POSIX_C_SOURCE 1*/

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include "loader.h"
#include "virmac.h"
#include "disasm.h"

static void run_file(struct VirMac *vm, const char *);
static struct VmData * read_file_to_vmdata(const char *);
static void disasm(const char *);
static void wellcum();

int main(int argc, const char *argv[])
{
    struct VirMac vm;
    virmac_init(&vm);
    switch (argc) {
      case 1: wellcum(); break;
      case 2: run_file(&vm, argv[1]); break;
      case 3: {
        if (strcmp(argv[1], "d") != 0) {
            fprintf(stderr, "illegal argument %s \n", argv[1]);
            virmac_free(&vm);
            return 1;
        }
        disasm(argv[2]);
        break;
      }
      default:
        fprintf(stderr, "U idiot, provide a signle file to be run\n");
        virmac_free(&vm);
        return 1;
    }
    virmac_free(&vm);
    return 0;
}

static void run_file(struct VirMac *vm, const char *path)
{
    struct VmData *prog = read_file_to_vmdata(path);
    enum ItpRes res = virmac_run(vm, prog);
    vmdata_free(prog);
    switch (res) {
      case ITP_OK:
//        printf("all ok\n");
        break;
      case ITP_RUNTIME_ERR:
        fprintf(stderr, "Der'z bin a runtime error\n");
        exit(1);
      default:
        fprintf(stderr, "some error from virmac_run\n");
        break;
    }
}

static struct VmData * read_file_to_vmdata(const char *path)
{
    FILE *file = fopen(path, "rb");
    if (file == NULL) {
        fprintf(stderr, "ERROR@read_file: opening file %s\n", path);
        exit(1);
    }
    fseek(file, 0L, SEEK_END);
    size_t file_size = ftell(file);
    rewind(file);
    uint8_t *buffer = malloc(file_size + 1);
    if (buffer == NULL) {
        fprintf(stderr, "ERROR@read_file: mallocating buffer\n");
        exit(1);
    }
    size_t bytes_read = fread(buffer, sizeof(uchar), file_size, file);
    if (bytes_read < file_size) {
        fprintf(stderr, "ERROR@read_file: could not read file %s\n", path);
        exit(1);
    }
    /* load */
    struct VmData *prog = vmdata_from_dfc(buffer, bytes_read);
    if (prog == NULL) {
        fprintf(stderr, "ERROR: couldn't load %s valid\n", path);
        exit(1);
    }
    fclose(file);
    free(buffer);
    return prog;
}

static void disasm(const char *path)
{
    struct VmData *vmd = read_file_to_vmdata(path);
    disasm_vmdata(vmd, path);
    vmdata_free(vmd);
}

static void wellcum(void)
{
    puts("Wellcome to the FlatVM: The VM for the DryFart language");
    puts("usage: ./flatvm [dfc-file]");
}
