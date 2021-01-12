#!/usr/bin/env python
# -*- coding: utf-8 -*-

from shutil import get_terminal_size
try:
    from PIL import Image
except:
    from sys import stderr
    stderr.write('[E] PIL not installed')
    exit(1)

from drawille import Canvas

if __name__ == '__main__':
    i = Image.open("/home/divy/Downloads/logo.jpg").convert('L')
    term_size = get_terminal_size()
    w, h = i.size
    tw = term_size[0] * 2
    th = term_size[1] * 2
    chars = []
    if tw < w:
        ratio = tw / float(w)
        w = tw
        h = int(h * ratio)
        i = i.resize((w, h), Image.ANTIALIAS)
    out = []
    can = Canvas()
    x = y = 0
    i_converted = i.tobytes()
    for pix in i_converted:
        if pix > 128:
            can.set(x, y)
        x += 1
        if x >= w:
            y += 1
            x = 0
    print(can.frame())

