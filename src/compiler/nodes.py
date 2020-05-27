from typing import List
from dataclasses import dataclass

@dataclass
class Argument:
    name: str
    typ: str

@dataclass
class Statement:
    pass

@dataclass
class Expression:
    pass

@dataclass
class Function:
    name: str
    arguments: List[Argument]
    return_type: str
    block: List[Statement]

@dataclass
class Program:
    function_list: List[Function]

@dataclass
class Declaration(Statement):
    name: str
    typ: str

@dataclass
class Assignment(Statement):
    name: str
    expression: Expression

@dataclass
class Return:
    expression: Expression

@dataclass
class NumberLiteral(Expression):
    value: int

@dataclass
class IdentifierValue(Expression):
    name: str

@dataclass
class FunctionCall(Expression):
    name: str
    arguments: List[Expression]

@dataclass
class ExpressionPlus(Expression):
    lhs: Expression
    rhs: Expression