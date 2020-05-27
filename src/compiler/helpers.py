class RegisterHandler:
    def __init__(self):
        self.available_registers = {
            "A": False, 
            "B": False, 
            "C": False, 
            "D": False
        }
    
    def request_register(self):
        for name, status in self.available_registers.items():
            if status == False:
                self.available_registers[name] = True
                return name

        raise "No more registers!"

    def free_register(self, register):
        for key, value in self.available_registers.items():
            if key == register and value == True:
                self.available_registers[key] = False
                return
            elif key == register and value == False:
                raise AssertionError(f"Register {register} already free!")
        raise NameError(register)

    def assert_clear(self):
        for key, value in self.available_registers.items():
            if value == True:
                raise AssertionError(f"Register handler not clear: {self.available_registers}")
    
    def active_registers(self):
        registers = []
        for key, value in self.available_registers.items():
            if value == True:
                registers.append(key)

        return registers

class LabelHandler:
    def __init__(self):
        self.labels = []
        self.hints = {}
    
    def add_label(self, hint):
        if hint not in self.labels:
            self.labels.append(hint)
            self.hints[hint] = 0
            return hint
        
        self.hints[hint] += 1
        label = f"{hint}__{self.hints[hint]}"
        self.labels.append(label)
        return label
    
    def add_relative_label(self, hint="__internal_relative"):
        return self.add_label(f".{hint}")
    
    def add_absolute_label(self, hint="__internal_absolute"):
        return self.add_label(f":{hint}")
        