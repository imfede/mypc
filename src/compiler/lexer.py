from enum import Enum, auto
import re

class TokenTyp(Enum):
    LEFT_BRACE = auto()
    RIGHT_BRACE = auto()
    LEFT_PAREN = auto()
    RIGHT_PAREN = auto()
    SEMICOLON = auto()
    PLUS = auto()
    MINUS = auto()
    NAND = auto()
    XOR = auto()
    COMMENT = auto()
    COMMA = auto()
    EQUAL = auto()

    STRING = auto()
    FUNCTION = auto()
    RETURN = auto()
    TYPE = auto()
    IDENTIFIER = auto()
    NUMBER = auto()

class Token:
    def __init__(self, typ, line, content):
        self.typ = typ
        self.line = line
        self.content = content
    
    def __repr__(self):
        return f"{self.typ.name} (line {self.line}: '{self.content}')"

def lexString(line, pos):
    content = ''
    pos += 1 # move to first string char
    while pos < len(line):
        if line[pos] == "\\":
            raise AssertionError("No support for escapes yet!")
        elif line[pos] == '"':
            return content, pos+1
        else:
            content += line[pos]
    raise AssertionError("EOL without terminating string!")

def lexIdentifier(line, pos):
    content = ''
    while re.match("[A-Za-z0-9]", line[pos]):
        content += line[pos]
        pos += 1
    return content, pos

def lexNumber(line, pos):
    content = ''
    while re.match("[0-9]", line[pos]):
        content += line[pos]
        pos += 1
    return content, pos

def lex(strings):
    tokens = []
    for number, line in enumerate(strings):
        line = line.strip()
        pos = 0

        while pos < len(line):
            if line[pos] == ' ':
                # discard spaces
                pos += 1
            elif line[pos] == '{':
                tokens.append(Token(TokenTyp.LEFT_BRACE, number, line[pos]))
                pos += 1
            elif line[pos] == '}':
                tokens.append(Token(TokenTyp.RIGHT_BRACE, number, line[pos]))
                pos += 1
            elif line[pos] == '(':
                tokens.append(Token(TokenTyp.LEFT_PAREN, number, line[pos]))
                pos += 1
            elif line[pos] == ')':
                tokens.append(Token(TokenTyp.RIGHT_PAREN, number, line[pos]))
                pos += 1
            elif line[pos] == ';':
                tokens.append(Token(TokenTyp.SEMICOLON, number, line[pos]))
                pos += 1
            elif line[pos] == "+":
                tokens.append(Token(TokenTyp.PLUS, number, line[pos]))
                pos += 1
            elif line[pos] == "-":
                tokens.append(Token(TokenTyp.MINUS, number, line[pos]))
                pos += 1
            elif line[pos] == "&":
                tokens.append(Token(TokenTyp.NAND, number, line[pos]))
                pos += 1
            elif line[pos] == "^":
                tokens.append(Token(TokenTyp.XOR, number, line[pos]))
                pos += 1
            elif line[pos] == "#":
                tokens.append(Token(TokenTyp.COMMENT, number, line[pos+1:].strip()))
                pos = len(line)
            elif line[pos] == ",":
                tokens.append(Token(TokenTyp.COMMA, number, line[pos]))
                pos += 1
            elif line[pos] == "=":
                tokens.append(Token(TokenTyp.EQUAL, number, line[pos]))
                pos += 1
            elif line[pos] == '"':
                content, pos = lexString(line, pos)
                tokens.append(Token(TokenTyp.STRING, number, content))
            else:
                # word
                if pos + 3 < len(line) and line[pos:pos+4] == "func":
                    tokens.append(Token(TokenTyp.FUNCTION, number, "func"))
                    pos += 4
                elif pos + 5 < len(line) and line[pos:pos+6] == "return":
                    tokens.append(Token(TokenTyp.RETURN, number, "return"))
                    pos += 6
                elif pos + 2 < len(line) and line[pos:pos+3] == "int":
                    tokens.append(Token(TokenTyp.TYPE, number, "int"))
                    pos += 4
                elif re.match("[0-9]", line[pos]):
                    content, pos = lexNumber(line, pos)
                    tokens.append(Token(TokenTyp.NUMBER, number, content))
                elif re.match("[A-Za-z]", line[pos]):
                    content, pos = lexIdentifier(line, pos)
                    tokens.append(Token(TokenTyp.IDENTIFIER, number, content))
                else:
                    print(f"Skipping: {line[pos]}")
                    pos += 1
    return tokens