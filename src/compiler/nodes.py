from typing import List
from dataclasses import dataclass
from textwrap import dedent

from .helpers import RegisterHandler, LabelHandler

@dataclass
class Node:
    def generate_code(self, label_handler):
        return f"ERROR!"

@dataclass
class Argument(Node):
    name: str
    typ: str

@dataclass
class Variable:
    name: str
    typ: str

@dataclass
class FunctionContext:
    register_handler: RegisterHandler
    label_handler: LabelHandler
    variables: List[Variable]
    arguments: List[Variable]
    ret_label: str

    def get_offset(self, variable_name):
        lst = list(reversed(self.variables))
        for index, var in enumerate(lst):
            if var.name == variable_name:
                return index + 1 # 1 for the stack offset
        
        lst = list(reversed(self.arguments))
        for index, var in enumerate(lst):
            if var.name == variable_name:
                return index + 7 + len(self.variables) # 2 for the ret_address + 4 for the register + 1 for the stack offset + n for the local variables on the stack

        raise IndexError(f"Variable {variable_name} not valid!")

@dataclass
class Statement(Node):
    def generate_code(self, context):
        return "ERROR!"

@dataclass
class Expression(Node):
    def generate_code(self, context):
        return ("No register", "ERROR!")

@dataclass
class Function(Node):
    name: str
    arguments: List[Argument]
    return_type: str
    block: List[Statement]

    def generate_code(self, label_handler):
        args = [Variable(name=arg.name, typ=arg.typ) for arg in self.arguments]

        # label for the header and footer of the function
        function_label = label_handler.add_absolute_label(hint=f"function_{self.name}")
        ret_label = label_handler.add_absolute_label(hint=f"function_ret_{self.name}")

        # the context shared by all the statements/expressions of this function
        context = FunctionContext(RegisterHandler(), label_handler, [], args, ret_label)

        # the actual code generation
        statments_code = "\n".join([statement.generate_code(context) for statement in self.block])

        # check that we are not forgetting about registers
        context.register_handler.assert_clear()

        # before returning we need to free the stack from local variables (arguments will be poped by the caller)
        pull_slide = "PULL\n" * len(context.variables)

        # code needed to save the ip on the stack
        save_ip_code = dedent("""
            # saving ip
            PUSH
            RTWL
            PUSH
            RTWH
            """)
        
        # code needed to read the ip from the stack
        restore_ip_code = dedent("""
            # restoring ip
            PULL
            RTRH
            PULL
            RTRL
            """)

        return "\n".join([
            "\n",
            f"# {self.name} {self.arguments} -> {self.return_type}",
            function_label,
            save_ip_code,
            statments_code,
            "HLT", # this is a safety feature. if you reach it it means your function did not return properly
            ret_label,
            pull_slide,
            restore_ip_code,
            "RET"
        ])
        
@dataclass
class Program(Node):
    function_list: List[Function]

    def generate_code(self, label_handler):
        functions_code = [function.generate_code(label_handler) for function in self.function_list]
        header_code = """
        LI A, 0xFF
        SPSL A
        SPSH A
        ZERO A
        PJMP :function_main
        JAL
        HLT
        """
        return dedent(header_code) + "\n".join(functions_code)

@dataclass
class Declaration(Statement):
    name: str
    typ: str

    def generate_code(self, context):
        context.variables.append(Variable(name=self.name, typ=self.typ))
        return f"PUSH # for var {self.name}"

@dataclass
class Assignment(Statement):
    name: str
    expression: Expression

    def generate_code(self, context):
        expression_register, expression_code = self.expression.generate_code(context)
        context.register_handler.free_register(expression_register)
        variable_offset = context.get_offset(self.name)

        return expression_code + dedent(f"""
            SPOF {hex(variable_offset)} # var: {self.name}
            MEMW {expression_register}
            """)

@dataclass
class Return(Statement):
    expression: Expression

    def generate_code(self, context):
        expression_register, expression_code = self.expression.generate_code(context)
        context.register_handler.free_register(expression_register)
        context.register_handler.assert_clear()
        return expression_code + dedent(f"""
            MV A, {expression_register}
            PJMP {context.ret_label}
            JMP
        """)

@dataclass
class NumberLiteral(Expression):
    value: int

    def generate_code(self, context):
        register = context.register_handler.request_register()
        return (
            register, 
            dedent(f"""\
            LI {register}, {hex(self.value)}
            """)
        )

@dataclass
class IdentifierValue(Expression):
    name: str

    def generate_code(self, context):
        offset = context.get_offset(self.name)
        register = context.register_handler.request_register()
        return (
            register, 
            dedent(f"""\
            SPOF {hex(offset)} # var {self.name}
            MEMR {register}
            """)
        )

@dataclass
class FunctionCall(Expression):
    name: str
    arguments: List[Expression]

    def generate_code(self, context):
        # the stack will contain (from the bottom up): the arguments, the current registers, the return address
        arguments_code = []
        for index, argument in enumerate(self.arguments):
            argument_register, argument_code = argument.generate_code(context)
            context.register_handler.free_register(argument_register)
            arguments_code.append(argument_code + dedent(f"""
                PUSH # arg #{index}
                MEMW {argument_register}
            """))
        
        pop_arguments_code = "PULL\n" * len(self.arguments)

        active_registers = context.register_handler.active_registers()
        # note: the push/pull without read/write are used to pad the stack so that this section always occupy 4 bytes
        save_registers_code = [f"PUSH\nMEMW {r}\n" for r in active_registers] + ["PUSH\n" * (4 - len(active_registers))]
        restore_registers_code = ["PULL\n" * (4 - len(active_registers))] + [f"PULL\nMEMR {r}\n" for r in reversed(active_registers)]

        register = context.register_handler.request_register()
        return (
            register,
            "\n".join([
                "",
                f"# calling {self.name}",
                "\n".join(arguments_code),
                "# saving registers",
                "\n".join(save_registers_code),
                dedent(f"""
                PJMP :function_{self.name}
                JAL

                # moving return type to correct register
                MV {register}, A

                # restoring registers
                """),
                "\n".join(restore_registers_code),
                "# popping arguments",
                pop_arguments_code
            ])
        )

@dataclass
class ExpressionPlus(Expression):
    lhs: Expression
    rhs: Expression

    def generate_code(self, context):
        lhs_register, lhs_code = self.lhs.generate_code(context)
        rhs_register, rhs_code = self.rhs.generate_code(context)
        context.register_handler.free_register(rhs_register)
        return (lhs_register, lhs_code + rhs_code + f"ADD {lhs_register}, {rhs_register}\n")
