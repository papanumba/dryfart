/* main.cpp */

#include <cstdio>
#include <cstdlib>
#include <unistd.h>
#include <exception>
#include "loader.h"
#include "reader.h"
#include "virmac.h"
#include "disasm.h"

static bool run_file(VirMac *vm, const char *);
static VmData * read_file_to_vmdata(const char *);
static bool disasm(const char *);
static void wellcum();

int main(int argc, const char *argv[])
{
    int status = 0;
    VirMac vm;
    switch (argc) {
      case 1: wellcum(); break;
      case 2:
        status = !run_file(&vm, argv[1]);
        break;
      case 3: {
        if (strcmp(argv[1], "d") != 0) {
            fprintf(stderr, "illegal argument %s \n", argv[1]);
            status = 1;
            break;
        }
        status = !disasm(argv[2]);
        break;
      }
      default:
        fprintf(stderr, "U idiot, provide a signle file to be run\n");
        status = 1;
    }
    return status;
}

static bool run_file(VirMac *vm, const char *path)
{
    VmData *prog = read_file_to_vmdata(path);
    if (prog == nullptr)
        return false;
    ItpRes res = vm->run(prog);
    delete prog;
    switch (res) {
      case ITP_OK: break;
      case ITP_RUNTIME_ERR:
        eputln("Der'z bin a runtime error");
        return false;
      case ITP_NULLPTR_ERR:
        eputln("nullptr error from virmac");
        return false;
      default:
        eputln("some error from virmac_run");
        return false;
    }
    return true;
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
