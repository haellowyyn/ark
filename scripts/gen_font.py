#! /usr/bin/python3
import string
import subprocess

from PIL import Image, ImageDraw, ImageFont


def gen_symbol(char):
    img = Image.new("L", (32, 60), 0)
    draw = ImageDraw.Draw(img)
    font = ImageFont.truetype("DejaVuSansMono.ttf", 48)
    draw.text((4, 0), char, font=font, fill=0xff)
    img = img.resize((8, 15), resample=Image.LANCZOS)

    data = img.getdata()
    return [p * 100 // 256 for p in data]


with open("font.rs", "w") as f:
    f.write("use super::Symbol;\n")
    f.write("\n")
    f.write("pub const SYMBOLS: &'static [&'static Symbol] = &[\n")

    symbols = "".join([string.digits, string.letters, string.punctuation, " "])
    for i in range(256):
        char = chr(i) if chr(i) in symbols else "?"
        sym = gen_symbol(char)
        f.write("\t&{},\n".format(sym))

    f.write("];\n")

subprocess.call(["rustfmt", "--write-mode", "Overwrite", "font.rs"])
