import re
import operator

def is_numeric(n):
    try:
        float(n)
        return True
    except ValueError:
        return False

class Interpreter:
    def __init__(self):
        self.vars = {}
        self.functions = {}

    def tokenize(self, program):
        if program == "":
            return []

        regex = re.compile(r'\s*(=>|[-+*/%=\(\)]|[A-Za-z_][A-Za-z0-9_]*|[0-9]*\.?[0-9]+)\s*')
        return [token for token in regex.split(program) if token and not token.isspace()]

    def input(self, expr):
        tokens = self.tokenize(expr)
        if not tokens:
            return ''

        if tokens[0] == 'fn':
            self.define_function(tokens)
            return ''

        if tokens[0] in self.functions:
            return self.evaluate_function(tokens)

        return self.evaluate_expression(tokens)
    
    def define_function(self, tokens):
        separator = tokens.index('=>') if '=>' in tokens else -1
        if separator == -1 or tokens[1] in self.vars:
            raise ValueError("Error")
        args = tokens[2:separator]
        body = ' '.join(tokens[separator + 1:])
        if len(set(args)) != len(args):
            raise ValueError("Error")
        for char in body:
            if char.isalpha() and char not in args:
                raise ValueError("Error")
        self.functions[tokens[1]] = {'args': args, 'body': body}
    
    def evaluate_function(self, tokens):
        while tokens:
            function_name_index = 0
            for i, token in enumerate(tokens):
                if token in self.functions:
                    function_name_index = i
            current_function = tokens[function_name_index:function_name_index + len(self.functions[tokens[function_name_index]]['args']) + 1]
            tail = tokens[function_name_index + len(self.functions[tokens[function_name_index]]['args']) + 1:]
            tokens = tokens[:function_name_index]
            args = self.functions[current_function[0]]['args']
            if len(args) != len(current_function) - 1:
                raise ValueError("Error")
            for i, arg in enumerate(args):
                self.vars[arg] = self.input(current_function[i + 1])
            function_value = self.input(self.functions[current_function[0]]['body'])
            tokens.append(str(function_value))
            tokens += tail
            if len(tokens) == 1:
                return float(tokens[0])
    
    def evaluate_expression(self, tokens):
        stack = []
        op = '+'
        preops = []
        unassigned_vars = []
        nest_vars = None
        is_nest = False

        operators = {
            '+': operator.add,
            '-': operator.sub,
            '*': operator.mul,
            '/': operator.truediv,
            '%': operator.mod
        }

        def apply_operator(stack, op, value):
            if op == '+':
                stack.append(value)
            elif op == '-':
                stack.append(-value)
            elif op in operators:
                if not stack:
                    raise ValueError("Error: not enough values in stack")
                temp1 = stack.pop()
                stack.append(operators[op](temp1, value))

        # Add a check for invalid input
        for i in range(len(tokens) - 1):
            if is_numeric(tokens[i]) and is_numeric(tokens[i + 1]):
                raise ValueError("Error: Invalid input")

        for i, token in enumerate(tokens):
            token_no_space = token.replace(" ", "")
            if ' ' in token and not is_numeric(token_no_space):
                raise ValueError("Error: Invalid input")
            if is_numeric(token_no_space):
                token = token_no_space
            if is_numeric(token):
                apply_operator(stack, op, float(token))
            elif re.match(r'^[a-zA-Z]+$', token):
                if token not in self.vars:
                    if i + 1 >= len(tokens) or tokens[i + 1] != '=':
                        raise ValueError("Error")
                    self.vars[token] = 0
                    if is_nest:
                        nest_vars = token
                    else:
                        unassigned_vars.append(token)
                else:
                    if i + 1 >= len(tokens) or tokens[i + 1] != '=':
                        apply_operator(stack, op, self.vars[token])
                    else:
                        if is_nest:
                            nest_vars = token
                        else:
                            unassigned_vars.append(token)
            elif token == '=':
                if i == len(tokens) - 1 or tokens[i + 1] == '=':
                    raise ValueError("Error")
                continue
            elif token == '(':
                is_nest = True
                preops.append(op)
                op = '+'
                stack.append(token)
            elif token == ')':
                temp = []
                while True:
                    t = stack.pop()
                    if t == '(':
                        break
                    temp.append(t)
                tempsum = sum(temp)
                if nest_vars:
                    self.vars[nest_vars] = tempsum
                    nest_vars = None
                is_nest = False
                preop = preops.pop() if preops else '+'
                apply_operator(stack, preop, tempsum)
            else:
                op = token

        res = sum(stack)
        for var in unassigned_vars:
            self.vars[var] = res
        return res
