#!/usr/bin/env python3


################################################################################
###############################  D F a r t E d  ################################
################################################################################


import sys, os, subprocess
from PyQt5 import uic, QtGui
from PyQt5.QtWidgets import *
import hieliter


INFARTER_PATH = "../infarter/target/release/infarter"
FLATVM_PATH   = "../flatvm/flatvm"

def get_name(f):
    if f is None:
        return "New File"
    else:
        return os.path.basename(f)


class Main(QMainWindow):
    def __init__(self):
        super().__init__()
        uic.loadUi("main.ui", self) # load the qt5 gui
        self.editor.setStyleSheet("""QPlainTextEdit{
            font-family:'Monospace';
            font-size: 16pt;
            color: #e2e2e2;
            background-color: #1B1B1B;}"""
        )
        self.output.setStyleSheet("""QPlainTextEdit{
            font-family:'Monospace';
            font-size: 14pt;
            color: #e2e2e2;
            background-color: #1b1b1b;}"""
        )
        self.edit_file = None
        self.temp_file = None
        self.saved = False
        self.update_title() # "New File"
        # set the highlighting to the editor widget
        self.highlight = hieliter.DFHieliter(self.editor.document())
        self.button_open.clicked.connect(self.open_file)
        self.button_save.clicked.connect(self.save_file)
        self.button_new.clicked.connect(self.new_file)
        # set analize() to the button_analize
        self.button_analize.clicked.connect(self.analize)
        # shortcuts
        self.sc_run = QShortcut(QtGui.QKeySequence("Ctrl+R"), self)
        self.sc_run.activated.connect(self.analize)
        self.sc_save = QShortcut(QtGui.QKeySequence("Ctrl+S"), self)
        self.sc_save.activated.connect(self.save_file)

    def update_title(self):
        self.setWindowTitle("DFartEd - " + get_name(self.edit_file))

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
        self.output.setPlainText('')
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
        savefile = open(self.edit_file,'w')
        savefile.write(self.editor.toPlainText())
        savefile.close()
        self.saved = True

    def analize(self):
        if self.edit_file is None:
            self.temp_file = ".tmp.df"
        # save a temp file, then send it to infarter & output result
        tempfile = open(self.temp_file, 'w')
        tempfile.write(self.editor.toPlainText())
        tempfile.close()
        result = subprocess.run(
            [INFARTER_PATH, "to", self.temp_file],
            capture_output=True
        )
        if result.stderr == b'':
            fvm = subprocess.run(
                [FLATVM_PATH, self.temp_file + "c"],
                capture_output=True
            )
            if fvm.stderr == b'':
                self.output.setPlainText(fvm.stdout.decode("utf-8"))
            else:
                self.output.setPlainText(fvm.stderr.decode("utf-8"))
        else:
            e = "ERROR from InFarter\n"+result.stderr.decode("utf-8")
            #self.output.clear()
            self.output.setPlainText(e)


if __name__=='__main__':
    app = QApplication(sys.argv)
    gui = Main()
    gui.show()
    sys.exit(app.exec_())
