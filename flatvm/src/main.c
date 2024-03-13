/* main.c */

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <fcntl.h>
#include <sys/mman.h>
#include "loader.h"
#include "virmac.h"
#include "disasm.h"

static int run_file(struct VirMac *vm, const char *);
static struct VmData * read_file_to_vmdata(const char *);
static int disasm(const char *);
static void wellcum();

int main(int argc, const char *argv[])
{
    int status = 0;
    struct VirMac vm;
    virmac_init(&vm);
    switch (argc) {
      case 1: wellcum(); break;
      case 2:
        status = !run_file(&vm, argv[1]);
        break;
      case 3: {
        if (strcmp(argv[1], "d") != 0) {
            fprintf(stderr, "illegal argument %s \n", argv[1]);
            status = 1;
        } else {
            status = !disasm(argv[2]);
        }
        break;
      }
      default:
        fprintf(stderr, "U idiot, provide a signle file to be run\n");
        status = 1;
    }
    virmac_free(&vm);
    return status;
}

static int run_file(struct VirMac *vm, const char *path)
{
    struct VmData *prog = read_file_to_vmdata(path);
    if (prog == NULL)
        return FALSE;
    enum ItpRes res = virmac_run(vm, prog);
    vmdata_free(prog);
    switch (res) {
      case ITP_OK: break;
      case ITP_RUNTIME_ERR:
        fprintf(stderr, "Der'z bin a runtime error\n");
        return FALSE;
      default:
        fprintf(stderr, "some error from virmac_run\n");
        return FALSE;
    }
    return TRUE;
}

/* returns new alloc'd VmData, NULL if error */
static struct VmData * read_file_to_vmdata(const char *path)
{
    struct VmData *prog = NULL;
    int file = open(path, O_RDONLY);
    if (file == -1) {
        fprintf(stderr, "ERROR: opening file %s\n", path);
        goto exit0;
    }
    size_t file_size = lseek(file, 0, SEEK_END);
    lseek(file, 0, SEEK_SET);
    uint8_t *buffer = mmap(NULL, file_size, PROT_READ, MAP_PRIVATE, file, 0);
    if (buffer == MAP_FAILED) {
        eputln("ERROR@read_file: mallocating buffer");
        goto exit1;
    }
    /* load */
    prog = vmdata_from_dfc(buffer, file_size);
    if (prog == NULL)
        fprintf(stderr, "ERROR: couldn't load %s valid\n", path);
    /* exit */
    munmap(buffer, file_size);
exit1:
    close(file);
exit0:
    return prog;
}

static int disasm(const char *path)
{
    struct VmData *vmd = read_file_to_vmdata(path);
    if (vmd == NULL)
        return FALSE;
    disasm_vmdata(vmd, path);
    vmdata_free(vmd);
    return TRUE;
}

static void wellcum(void)
{
    const char msg[] =
        "Wellcome to the FlatVM: The VM for the DryFart language\n\n"
        "usage:\n"
        "    to run bytecode: ./flatvm example.dfc\n"
        "    to disassemble:  ./flatvm d example.dfc\n"
    ;

    if (-1 == write(STDOUT_FILENO, msg, sizeof(msg)))
        exit(1);
}
