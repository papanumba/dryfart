# hieliter.py

#import sys
from PyQt5.QtCore import QRegExp
from PyQt5.QtGui import QColor, QTextCharFormat, QFont, QSyntaxHighlighter


# returns a QTextCharFormat with the given attributes
def format(color, style=''):
    _color = QColor()
    if type(color) is not str:
        _color.setRgb(color[0], color[1], color[2])
    else:
        _color.setNamedColor(color)
    format = QTextCharFormat()
    format.setForeground(_color)
    if 'bold' in style:
        format.setFontWeight(QFont.Bold)
    if 'italic' in style:
        format.setFontItalic(True)
    return format


# luma: best 70
# chroma: best 60
colors = {
    'black':    [  0,   0,   0],
    'red':      [254, 119, 112],
    'green':    [ 56, 194, 113],
    'yellow':   [194, 169,  52],
    'blue':     [124, 174, 255],
    'magenta':  [241, 132, 227],
    'cyan':     [ 22, 189, 207],
    'white':    [171, 171, 171],
    'bblack':   [119, 119, 119],
}

'''    'bred'
    'bgreen'
    'byellow'
    'bblue'
    'bmagenta'
    'bcyan'
    'bwhite' '''

# Syntax styles specific by token type
STYLES = {
    'func':     format(colors['green']  , 'bold'),
    'proc':     format(colors['cyan']   , 'bold'),
    'table':    format(colors['red']    , 'bold'),
    'array':    format(colors['magenta'], 'bold'),
    'control':  format(colors['yellow'] , 'bold'),
    'number':   format(colors['cyan'])  ,
    'string':   format(colors['yellow']),
    'type':     format(colors['blue']   , 'bold'),
    'comment':  format(colors['bblack'] , 'italic'),
}


class DFHieliter(QSyntaxHighlighter):
    def __init__(self, document):
        QSyntaxHighlighter.__init__(self, document)
        rules = [
            (r'(\b|_)[0-9][0-9]*(u|\.[0-9]+\b)?(\b|_)', 0, STYLES['number']),
            (r'[()]',           0, STYLES['string']),
            (r'(=>|\[|\]|@)',   0, STYLES['control']),
            (r'(\b|_)[BCNZR]%',     0, STYLES['type']),
            (r'\b[TFV]\b',      0, STYLES['table']),
            (r'#(@[0-9]*)?',    0, STYLES['func']),
            (r'(\b|_)[A-Za-z][A-Za-z\d]*#',    0, STYLES['func']),
            (r'!(@[0-9]*)?',    0, STYLES['proc']),
            (r'[A-Za-z][A-Za-z\d]*!',    0, STYLES['proc']),
            (r'\$(@[0-9]*)?',   0, STYLES['table']),
            (r'(\b|_)[A-Za-z][A-Za-z\d]*\$',   0, STYLES['table']),
            (r'_',              0, STYLES['array']),
            (r'"([^"$]*["$NT]\$)*[^"$]*"', 0, STYLES['string']),
            (r"'[^\n]*", 0, STYLES['comment'])
        ]
        # Build a QRegExp for each pattern
        self.rules = [(QRegExp(pat), index, fmt)
                      for (pat, index, fmt) in rules]

    # Apply syntax highlighting to the given block of text.
    def highlightBlock(self, text):
        for expression, nth, format in self.rules:
            index = expression.indexIn(text, 0)
            while index >= 0:
                # We actually want the index of the nth match
                index = expression.pos(nth)
                length = len(expression.cap(nth))
                self.setFormat(index, length, format)
                index = expression.indexIn(text, index + length)
        self.setCurrentBlockState(0)
