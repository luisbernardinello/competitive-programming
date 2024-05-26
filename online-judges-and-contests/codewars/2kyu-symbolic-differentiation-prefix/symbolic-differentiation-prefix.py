import re

def parse_expression(expr):
    tokens = re.findall(r'\(|\)|\w+|\S', expr)
    def parse(tokens):
        token = tokens.pop(0)
        if token == '(':
            result = []
            while tokens[0] != ')':
                result.append(parse(tokens))
            tokens.pop(0)  # pop the closing ')'
            return result
        elif token.isnumeric() or re.match(r'-?\d+(\.\d+)?', token):
            return float(token)
        else:
            return token
    return parse(tokens)

def to_string(expr):
    if isinstance(expr, list):
        return f"({' '.join(map(to_string, expr))})"
    elif isinstance(expr, float) and expr.is_integer():
        return str(int(expr))
    return str(expr)

def simplify(expr):
    if isinstance(expr, list):
        if len(expr) == 3 and expr[0] in ('+', '-', '*', '/'):
            left = simplify(expr[1])
            right = simplify(expr[2])
            if expr[0] == '+':
                if left == 0:
                    return right
                if right == 0:
                    return left
            if expr[0] == '-':
                if right == 0:
                    return left
            if expr[0] == '*':
                if left == 0 or right == 0:
                    return 0
                if left == 1:
                    return right
                if right == 1:
                    return left
                if isinstance(left, (int, float)) and isinstance(right, list) and right[0] == '*':
                    return simplify(['*', left * right[1], right[2]])
                if isinstance(right, (int, float)) and isinstance(left, list) and left[0] == '*':
                    return simplify(['*', right * left[1], left[2]])
            if expr[0] == '/':
                if left == 0:
                    return 0
                if right == 1:
                    return left
                if isinstance(left, (int, float)) and isinstance(right, (int, float)):
                    return left / right
                if isinstance(left, list) and isinstance(right, float):
                    return simplify(['*', left, 1/right])
            if isinstance(left, (int, float)) and isinstance(right, (int, float)):
                if expr[0] == '+':
                    return left + right
                if expr[0] == '-':
                    return left - right
                if expr[0] == '*':
                    return left * right
                if expr[0] == '/':
                    return left / right
            return [expr[0], left, right]
        elif len(expr) == 2 and expr[0] in ('cos', 'sin', 'exp', 'ln', 'tan'):
            arg = simplify(expr[1])
            return [expr[0], arg]
        elif len(expr) == 3 and expr[0] == '^':
            base = simplify(expr[1])
            exponent = simplify(expr[2])
            if exponent == 0:
                return 1
            if exponent == 1:
                return base
            return [expr[0], base, exponent]
        else:
            return expr
    return expr

def differentiate(expr):
    if isinstance(expr, str):
        if expr == 'x':
            return 1
        else:
            return 0
    elif isinstance(expr, (int, float)):
        return 0
    elif isinstance(expr, list):
        op = expr[0]
        if op == '+':
            return simplify(['+', differentiate(expr[1]), differentiate(expr[2])])
        elif op == '-':
            return simplify(['-', differentiate(expr[1]), differentiate(expr[2])])
        elif op == '*':
            u, v = expr[1], expr[2]
            return simplify(['+', ['*', differentiate(u), v], ['*', u, differentiate(v)]])
        elif op == '/':
            u, v = expr[1], expr[2]
            if isinstance(v, (int, float)):
                return simplify(['/', differentiate(u), v])
            return simplify(['/', ['-', ['*', differentiate(u), v], ['*', u, differentiate(v)]], ['^', v, 2]])
        elif op == '^':
            base, exponent = expr[1], expr[2]
            if isinstance(exponent, (int, float)):
                if exponent == 0:
                    return 0
                if exponent == 1:
                    return differentiate(base)
                return simplify(['*', exponent, ['*', ['^', base, exponent - 1], differentiate(base)]])
            else:
                raise NotImplementedError("Exponentiation with non-constant exponent is not supported.")
        elif op == 'cos':
            return simplify(['*', -1, ['*', differentiate(expr[1]), ['sin', expr[1]]]])
        elif op == 'sin':
            return simplify(['*', differentiate(expr[1]), ['cos', expr[1]]])
        elif op == 'exp':
            return simplify(['*', differentiate(expr[1]), ['exp', expr[1]]])
        elif op == 'ln':
            return simplify(['/', differentiate(expr[1]), expr[1]])
        elif op == 'tan':
            return simplify(['*', differentiate(expr[1]), ['+', 1, ['^', ['tan', expr[1]], 2]]])
        else:
            raise ValueError(f"Unknown operator: {op}")
    else:
        raise ValueError(f"Invalid expression: {expr}")

def diff(expr):
    parsed_expr = parse_expression(expr)
    differentiated_expr = differentiate(parsed_expr)
    simplified_expr = simplify(differentiated_expr)
    return to_string(simplified_expr)