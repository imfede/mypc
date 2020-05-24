from .lexer import TokenTyp
from .commons import listSplit

class Function:
    def __init__(self, name, line, returnType, arguments, statements):
        self.name = name
        self.line = line
        self.returnType = returnType
        self.arguments = arguments
        self.statements = statements
    
    def getLocalVariables(self):
        return list(filter(lambda s: isinstance(s, VariableDeclaration), self.statements))

    def __repr__(self):
        return f"function {self.name}: arguments({self.arguments}) -> {self.returnType}\n{self.statements}" 

class Argument:
    def __init__(self, typ, name):
        self.typ = typ
        self.name = name
    
    def __repr__(self):
        return f"{self.name}: {self.typ}"

class Statement:
    def __init__(self, tokens):
        self.tokens = tokens

    def generate(self, variables):
        return []

    def __repr__(self):
        return f"Generic statement: {self.tokens}"

class VariableDeclaration(Statement):
    @staticmethod
    def validate(tokens):
        return (len(tokens) == 2) and (tokens[0].typ == TokenTyp.TYPE) and (tokens[1].typ == TokenTyp.IDENTIFIER)

    def getName(self):
        return self.tokens[1]
    
    def getType(self):
        return self.tokens[0]
    
    def __repr__(self):
        return f"Declaration: {self.getName().content} of type {self.getType().content}"

class VariableAssignment(Statement):
    @staticmethod
    def validate(tokens):
        return (len(tokens) > 2) and (tokens[0].typ == TokenTyp.IDENTIFIER) and (tokens[1].typ == TokenTyp.EQUAL)
    
    def getName(self):
        return self.tokens[0]
    
    def getValue(self):
        return getStatement(self.tokens[2:])

    def generate(self, variables):
        assert self.getName().content in variables
        offset = variables[self.getName().content]

        return self.getValue().generate(variables) + [
            f"SPOF {offset}",
            f"MEMW A"
        ]

    def __repr__(self):
        return f"Assignment: to {self.getName().content} from {self.getValue()}"

class FunctionCall(Statement):
    @staticmethod
    def validate(tokens):
        return ((len(tokens) > 2) and 
            (tokens[0].typ == TokenTyp.IDENTIFIER) and 
            (tokens[1].typ == TokenTyp.LEFT_PAREN) and 
            (tokens[len(tokens)-1].typ == TokenTyp.RIGHT_PAREN))

    def getName(self):
        return self.tokens[0]
    
    def getArguments(self):
        return list(map(getStatement, listSplit(self.tokens[2:-1], lambda t: t.typ == TokenTyp.COMMA)))

    def generate(self, variables):
        code = []
        for arg in self.getArguments():
            code += arg.generate(variables)
            code += [
                "PUSH",
                "MEMW A"
            ]
        code += [
            f"PJMP :{self.getName().content}",
            "JAL",
            "PULL",
            "RTRL",
            "PULL",
            "RTRH"
        ]
        return code

    def __repr__(self):
        return f"Calling: function {self.getName().content} with arguments {self.getArguments()}"

class Return(Statement):
    @staticmethod
    def validate(tokens):
        return (len(tokens) > 1) and (tokens[0].typ == TokenTyp.RETURN)
    
    def getValue(self):
        return getStatement(self.tokens[1:])
    
    def generate(self, variables):
        code = self.getValue().generate(variables)
        code += [
            "RET"
        ]
        return code

    def __repr__(self):
        return f"Return: {self.getValue()}"

class RawValue(Statement):
    @staticmethod
    def validate(tokens):
        return (len(tokens) == 1) and (tokens[0].typ == TokenTyp.NUMBER)
    
    def getValue(self):
        return self.tokens[0]
    
    def generate(self, variables):
        value = self.getValue().content
        return [
            f"LI A, {value}"
        ]

    def __repr__(self):
        return f"Raw: {self.getValue()}"

class VariableValue(Statement):
    @staticmethod
    def validate(tokens):
        return (len(tokens) == 1) and (tokens[0].typ == TokenTyp.IDENTIFIER)

    def getValue(self):
        return self.tokens[0]
    
    def generate(self, variables):
        assert self.getValue().content in variables
        offset = variables[self.getValue().content]
        return [
            f"SPOF {offset}",
            f"MEMR A"
        ]

    def __repr__(self):
        return f"Value of: {self.getValue()}"

def getStatement(tokens):
    if VariableDeclaration.validate(tokens):
        return VariableDeclaration(tokens)
    elif VariableAssignment.validate(tokens):
        return VariableAssignment(tokens)
    elif FunctionCall.validate(tokens):
        return FunctionCall(tokens)
    elif Return.validate(tokens):
        return Return(tokens)
    elif RawValue.validate(tokens):
        return RawValue(tokens)
    elif VariableValue.validate(tokens):
        return VariableValue(tokens)

    return Statement(tokens)