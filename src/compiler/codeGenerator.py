from pprint import pprint

def generate(ast):
    ret = []
    ret += programHeader()
    for func in map(generateFunction, ast):
        ret += func
    ret += programFooter()
    return ret

def programHeader():
    return [
        "LI A, 0xFF",
        "SPSL A",
        "SPSH A",
        "ZERO A",
        "PJMP :main",
        "JMP"
    ]

def programFooter():
    return [
        ":exit",
        "HLT"
    ]

def generateFunction(func):
    body = [
        f":{func.name.content}",
        "PUSH",
        "RTWH",
        "PUSH",
        "RTWL"
    ]
    variables = {} # dict<string:int> mapping vars (and args) to offsets
    offset = len(func.arguments) + len(func.getLocalVariables()) + 2
    for arg in func.arguments:
        variables[arg.name.content] = offset
        offset -= 1
    offset -= 2
    for var in func.getLocalVariables():
        variables[var.getName().content] = offset
        body += [
            "PUSH",
        ]
        offset -= 1
    for statement in func.statements:
        body += statement.generate(variables)
    return body
