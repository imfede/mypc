class Instruction:
    def __init__(self, name, arity, target, mask, steps):
        self.name = name
        self.arity = arity
        self.target = target
        self.mask = mask
        self.steps = steps
    
    def isThis(self, instruction):
        return (instruction & self.mask) == self.target

    def __str__(self):
        return f"{self.name} [{self.arity}]: {self.target}/{self.mask}"