/* main.cpp */

#include <cstdio>
#include <cstdlib>
#include <unistd.h>
#include <exception>
#include "loader.h"
#include "reader.h"
//#include "virmac.h"
#include "disasm.h"

//static int run_file(struct VirMac *vm, const char *);
static VmData * read_file_to_vmdata(const char *);
static bool disasm(const char *);
static void wellcum();

int main(int argc, const char *argv[])
{
    int status = 0;
//    struct VirMac vm;
//    virmac_init(&vm);
    switch (argc) {
      case 1: wellcum(); break;
      case 2:
//        status = !run_file(&vm, argv[1]);
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
//    virmac_free(&vm);
    return status;
}

/*static int run_file(struct VirMac *vm, const char *path)
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
}*/

/* returns new alloc'd VmData, NULL if error */
VmData * read_file_to_vmdata(const char *path)
{
    Reader reader;
    auto res = reader_open(path, &reader);
    if (res != READRES_OK) {
        eputln("ERROR reading file:");
        eputln(readres_what(res));
        return nullptr;
    }
    VmData *prog = nullptr;
    /* load */
    try {
        prog = new VmData(reader.buf, reader.len);
    } catch (std::exception &e) {
        eputln("ERROR loading dfc:");
        eputln(e.what());
    }
    /* exit */
    (void) reader_free(&reader);
    return prog;
}

static bool disasm(const char *path)
{
    VmData *vmd = read_file_to_vmdata(path);
    if (vmd == nullptr)
        return false;
    disasm_vmdata(vmd, path);
    delete vmd;
    return true;
}

static void wellcum(void)
{
    const char msg[] =
        "Wellcome to the FlatVM: The VM for the DryFart language\n\n"
        "usage:\n"
        "    to run bytecode: ./flatvm example.dfc\n"
        "    to disassemble:  ./flatvm d example.dfc\n"
    ;

    if (-1 == write(1, msg, sizeof(msg)))
        exit(1);
}
