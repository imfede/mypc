import ply.lex as lex
import ply.yacc as yacc
from pprint import pprint

tokens = [
    'FUNCTION',
    'RETURN',
    'SEMICOLON',
    'COMMA',

    'TYPE_INT',

    'IDENTIFIER',
    
    'OPEN_PAREN',
    'CLOSE_PAREN',
    'OPEN_BRACE',
    'CLOSE_BRACE',
    
    'EQUAL',
    'PLUS',
    
    'NUMBER',

    'COMMENT'
]

def Lexer():
    t_FUNCTION = r'function'
    t_RETURN = r'return'
    t_SEMICOLON = r';'
    t_COMMA = r","
    t_TYPE_INT = r'int'

    t_IDENTIFIER = r'(?!int)(?!function)(?!return)[a-zA-Z]+'

    t_OPEN_PAREN = r'\('
    t_CLOSE_PAREN = r'\)'
    t_OPEN_BRACE = r'\{'
    t_CLOSE_BRACE = r'\}'

    t_EQUAL = r"="
    t_PLUS = r"\+"

    def t_NUMBER(t):
        r'\d+'
        t.value = int(t.value)    
        return t

    t_ignore_COMMENT = r'\#.*'

    # Define a rule so we can track line numbers
    def t_newline(t):
        r'\n+'
        t.lexer.lineno += len(t.value)

    # A string containing ignored characters (spaces and tabs)
    t_ignore  = ' \t'

    # Error handling rule
    def t_error(t):
        print("Illegal character '%s'" % t.value[0])
        t.lexer.skip(1)

    return lex.lex()

def Parser():
    def p_function_list(p):
        """ function_list : function 
                          | function function_list"""
        if len(p) == 2:
            p[0] = ('functions', p[1])
        else:
            p[0] = ('functions', p[1], *p[2][1:])

    def p_function(p):
        "function : FUNCTION TYPE_INT IDENTIFIER argument_block block"
        p[0] = ('function', p[3], p[4], p[5])

    def p_argument_block(p):
        """argument_block : OPEN_PAREN CLOSE_PAREN
                          | OPEN_PAREN argument_list CLOSE_PAREN"""
        if len(p) == 3:
            p[0] = ('argblock', )
        else:
            p[0] = ('argblock', p[2])

    def p_argument_list(p):
        """argument_list : TYPE_INT IDENTIFIER
                         | TYPE_INT IDENTIFIER COMMA argument_list"""
        if len(p) == 3:
            p[0] = ('arglist', p[2])
        else:
            p[0] = ('arglist', p[2], *p[4][1:])

    def p_block(p):
        """block : OPEN_BRACE CLOSE_BRACE
                 | OPEN_BRACE statement_list CLOSE_BRACE"""
        if len(p) == 3:
            p[0] = ('block', )
        else:
            p[0] = ('block', p[2])

    def p_statement_list(p):
        """statement_list : statement SEMICOLON
                          | statement SEMICOLON statement_list""" 
        if len(p) == 3:
            p[0] = ('statements', p[1])
        else:
            p[0] = ('statements', p[1], *p[3][1:])
    
    def p_statement_declaration(p):
        "statement : TYPE_INT IDENTIFIER"
        p[0] = ('declare', p[2])

    def p_statement_assignment(p):
        "statement : IDENTIFIER EQUAL expression"
        p[0] = ('assign', p[1], p[3])

    def p_statement_return(p):
        "statement : RETURN expression"
        p[0] = ('return', p[2])

    def p_expression_paren(p):
        "expression : OPEN_BRACE expression CLOSE_BRACE"
        p[0] = p[2]

    def p_expression_identifier(p):
        "expression : IDENTIFIER"
        p[0] = ('identifier', p[1])

    def p_expression_number(p):
        "expression : NUMBER"
        p[0] = ('number', p[1])

    def p_expression_plus(p):
        "expression : expression PLUS expression"
        p[0] = ('plus', p[1], p[3])

    def p_expression_function_call(p):
        "expression : IDENTIFIER OPEN_PAREN expression_list CLOSE_PAREN"
        p[0] = ('func_call', p[1], p[3])

    def p_expression_list(p):
        """expression_list : expression
                           | expression COMMA expression_list"""
        p[0] = ('expr_list', p[1])

    return yacc.yacc(debug=True)

def main(fname):
    lexer = Lexer()
    parser = Parser()
    with open(fname, "r", encoding="utf-8") as f:   
        source = f.read()
        result = parser.parse(source, lexer=lexer)
        
        pprint(result)