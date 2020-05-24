from pprint import pprint

from .lexer import TokenTyp
from .parserDTO import *

def parseFunction(tokens, pos):
    assert tokens[pos].typ == TokenTyp.FUNCTION
    function = tokens[pos]
    pos += 1

    assert tokens[pos].typ == TokenTyp.TYPE
    returnType = tokens[pos]
    pos += 1

    assert tokens[pos].typ == TokenTyp.IDENTIFIER
    name = tokens[pos]
    pos += 1

    assert tokens[pos].typ == TokenTyp.LEFT_PAREN
    pos += 1

    arguments = []
    while tokens[pos].typ != TokenTyp.RIGHT_PAREN:
        assert tokens[pos].typ == TokenTyp.TYPE
        assert tokens[pos+1].typ == TokenTyp.IDENTIFIER
        arguments.append(Argument(tokens[pos], tokens[pos+1]))
        pos += 2
        if tokens[pos].typ == TokenTyp.COMMA:
            pos += 1

    assert tokens[pos].typ == TokenTyp.RIGHT_PAREN
    pos += 1

    assert tokens[pos].typ == TokenTyp.LEFT_BRACE
    pos += 1

    statements = []
    currentStatements = []
    while tokens[pos].typ != TokenTyp.RIGHT_BRACE:
        if tokens[pos].typ == TokenTyp.COMMENT:
            pos += 1
            continue

        currentStatements.append(tokens[pos])
        pos += 1

        if tokens[pos].typ == TokenTyp.SEMICOLON:
            statements.append(currentStatements)
            currentStatements = []
            pos += 1

    body = list(map(getStatement, statements))

    return pos, Function(name, function.line, returnType, arguments, body)

def parse(tokens):
    ast = []
    pos = 0
    while pos < len(tokens):
        token = tokens[pos]
        if token.typ == TokenTyp.FUNCTION:
            pos, func = parseFunction(tokens, pos)
            ast.append(func)
        else:
            pos += 1

    return ast