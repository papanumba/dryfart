#!/usr/bin/env python3


################################################################################
###############################  D F a r t E d  ################################
################################################################################


import sys, os, subprocess
from subprocess import Popen, PIPE
from PyQt5 import uic, QtGui
from PyQt5.QtWidgets import *
from PyQt5.QtCore import QProcess, QLibrary, QObject, pyqtSignal, pyqtSlot
import hieliter

INFARTER_PATH = "../infarter/target/release/infarter"
FLATVM_PATH   = "../flatvm/flatvm"

get_name = lambda f: os.path.basename(f) if f is not None else "New File"

class Main(QMainWindow):
    edit_file   = None
    temp_file   = None
    saved       = False
    df_lib      = QLibrary("df-lib")
    fvm         = QProcess()
    fvm_term    = pyqtSignal()

    def __init__(self, cwd):
        super().__init__()
        self.cwd = cwd + "/"
        uic.loadUi(self.cwd + "main.ui", self) # load the qt5 gui
        self.editor.setStyleSheet("""QPlainTextEdit{
            font-family:'Monospace';
            font-size: 16pt;
            color: #e2e2e2;
            background-color: #1B1B1B;}"""
        )
        self.stdout.setStyleSheet("""QPlainTextEdit{
            font-family:'Monospace';
            font-size: 14pt;
            color: #e2e2e2;
            background-color: #1b1b1b;}"""
        )
        self.stderr.setStyleSheet("""QPlainTextEdit{
            font-family:'Monospace';
            font-size: 14pt;
            color: #ef8070;
            background-color: #1b1b1b;}"""
        )
        # paþs
        self.infarter_path = os.path.abspath(self.cwd + INFARTER_PATH)
        self.flatvm_path   = os.path.abspath(self.cwd + FLATVM_PATH)
        # load DF-lib
        if not self.df_lib.load():
            print(self.df_lib.errorString(), file=sys.stderr)
            sys.exit(1)
        # FlatVM worker & its þread
        self.fvm.setProgram(self.flatvm_path)
        self.fvm_term.connect(self.fvm.terminate)
        self.fvm.finished.connect(self.fvm_finished)
        # TODO: why did I put þis 2 signals?
        self.fvm.readyReadStandardOutput.connect(self.append_fvm_out)
        self.fvm.readyReadStandardError .connect(self.append_fvm_err)
        # init title as "New File"
        self.update_title()
        # set þe hiȝliȝting to þe editor widget
        self.highlight = hieliter.DFHieliter(self.editor.document())
        # connect buttons to meþods
        self.button_open.clicked.connect(self.open_file)
        self.button_save.clicked.connect(self.save_file)
        self.button_new .clicked.connect(self.new_file)
        self.button_kill.clicked.connect(self.kill)
        self.button_run .clicked.connect(self.run)
        # shortcuts
        self.sc_run  = QShortcut(QtGui.QKeySequence("Ctrl+R"), self)
        self.sc_run .activated.connect(self.run)
        self.sc_kill = QShortcut(QtGui.QKeySequence("Ctrl+."), self)
        self.sc_kill.activated.connect(self.kill)
        self.sc_save = QShortcut(QtGui.QKeySequence("Ctrl+S"), self)
        self.sc_save.activated.connect(self.save_file)
        # check
        self.check_fvm_exists()

    def check_fvm_exists(self):
        if os.path.isfile(self.flatvm_path):
            return # OK
        # WARN
        self.button_run.setEnabled(False)
        self.update_result(
            "",
            "FlatVM binary not found\nexpected at " + self.flatvm_path,
        )

    def update_title(self):
        self.setWindowTitle("DFartEd - " + get_name(self.edit_file))

    def update_result(self, out, err):
        self.stdout.setPlainText(out)
        self.stderr.setPlainText(err)

    @pyqtSlot()
    def append_fvm_out(self):
        o = self.fvm \
            .readAllStandardOutput() \
            .data() \
            .decode("utf-8")
        self.stdout.appendPlainText(o)

    @pyqtSlot()
    def append_fvm_err(self):
        e = self.fvm \
            .readAllStandardError() \
            .data() \
            .decode("utf-8")
        self.stderr.appendPlainText(e)

    @pyqtSlot(int, QProcess.ExitStatus)
    def fvm_finished(self, e_code, e_status):
        self.append_fvm_out()
        self.append_fvm_err()
        self.enable_non_kill()

    # setEnabled for all buttons oþer þan Kill
    def enable_non_kill(self, e = True):
        self.button_open.setEnabled(e)
        self.button_new .setEnabled(e)
        self.button_save.setEnabled(e)
        self.button_run .setEnabled(e)

    def new_file(self):
        # edited opened file must be saved
        if self.edit_file is not None:
            self.save_file()
        # edited new file w/ chars other than whitespace must be saved
        else:
            if self.editor.toPlainText().strip() != '':
                self.save_file()
        self.saved = False
        self.edit_file = None
        self.temp_file = None
        self.editor.setPlainText('')
        self.stdout.setPlainText('')
        self.update_title()

    def open_file(self):
        if self.temp_file is not None:
#            subprocess.run(["rm", self.temp_file])
            self.temp_file = None
        name = QFileDialog.getOpenFileName(self, 'Open File', '',
            'DryFart script (*.df)')
        if name == '': # pressed `Cancel`
            return
        self.edit_file = name[0]
        self.saved = False
        openfile = open(self.edit_file, 'r')
        self.update_title()
        self.editor.setPlainText(openfile.read())
        openfile.close()
        self.temp_file = name[0] + ".tmp"

    def save_file(self):
        if self.saved:
            return
        if self.edit_file is None or self.edit_file == '':
            name = QFileDialog.getSaveFileName(
                self, 'Save File', '', 'DryFart script (*.df)'
            )
            if name == '': # pressed `Cancel`
                return
            self.edit_file = name[0]
            self.setWindowTitle("DFartEd - " + get_name(self.edit_file))
        savefile = open(self.edit_file, 'w')
        savefile.write(self.editor.toPlainText())
        savefile.close()
        self.saved = True

    def run(self):
        self.kill()
        if self.edit_file is None:
            self.temp_file = ".tmp.df"
        # save a temp file, then send it to infarter & output result
        tempfile = open(self.temp_file, 'w')
        tempfile.write(self.editor.toPlainText())
        tempfile.close()
        # compile it, þis should be fast, so no need to þread
        result = subprocess.run(
            [self.infarter_path, "to", self.temp_file],
            capture_output=True
        )
        if result.stderr != b'':
            e = "ERROR from InFarter\n" + result.stderr.decode("utf-8")
            self.update_result("", e)
            return
        self.update_result("", "")
        self.fvm.setArguments([self.temp_file + "c"])
        self.fvm.start()
        self.enable_non_kill(False)

    def kill(self):
        if self.fvm.state() == QProcess.NotRunning:
            return
        self.fvm.terminate() # or kill() ?
        self.update_result("", "successfully killed")


if __name__ == "__main__":
    argv = sys.argv
    app = QApplication(argv)
    cwd = os.path.abspath(os.path.dirname(argv[0]))
    gui = Main(cwd)
    gui.show()
    status = app.exec()
    sys.exit(status)
