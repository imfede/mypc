#!/usr/bin/env python3.7

from sys import argv
from pprint import pprint

from .lexer import lex
from .parser import parse
from .codeGenerator import generate

def main(fname):
    tokens = []
    with open(fname, "r") as f:
        tokens = lex(f.readlines())

    ast = parse(tokens)

    ass = generate(ast)

    with open("source.as", "w") as f:
        for code in ass:
            f.write(f"{code}\n")