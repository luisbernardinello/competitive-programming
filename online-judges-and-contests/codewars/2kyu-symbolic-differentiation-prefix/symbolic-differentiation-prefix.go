package kata


import (
	"fmt"
	"regexp"
	"strconv"
	"strings"
)

// Function to parse expression
func parseExpression(expr string) interface{} {
	re := regexp.MustCompile(`\(|\)|\w+|\S`)
	tokens := re.FindAllString(expr, -1)
	result, _ := parse(tokens)
	return result
}

func parse(tokens []string) (interface{}, []string) {
	token := tokens[0]
	tokens = tokens[1:]
	if token == "(" {
		var result []interface{}
		for tokens[0] != ")" {
			var subExpr interface{}
			subExpr, tokens = parse(tokens)
			result = append(result, subExpr)
		}
		tokens = tokens[1:] // Pop the closing ')'
		return result, tokens
	} else if num, err := strconv.ParseFloat(token, 64); err == nil {
		return num, tokens
	} else {
		return token, tokens
	}
}

// Function to convert expression to string
func toString(expr interface{}) string {
	switch v := expr.(type) {
	case []interface{}:
		var str []string
		for _, e := range v {
			str = append(str, toString(e))
		}
		return "(" + strings.Join(str, " ") + ")"
	case float64:
		if v == float64(int(v)) {
			return fmt.Sprintf("%d", int(v))
		}
		return fmt.Sprintf("%g", v)
	case string:
		return v
	default:
		return ""
	}
}

// Function to simplify expression
func simplify(expr interface{}) interface{} {
	switch v := expr.(type) {
	case []interface{}:
		if len(v) == 3 {
			op, left, right := v[0], simplify(v[1]), simplify(v[2])
			switch op {
			case "+":
				if left == 0.0 {
					return right
				}
				if right == 0.0 {
					return left
				}
				if l, ok := left.(float64); ok {
					if r, ok := right.(float64); ok {
						return l + r
					}
				}
			case "-":
				if right == 0.0 {
					return left
				}
				if l, ok := left.(float64); ok {
					if r, ok := right.(float64); ok {
						return l - r
					}
				}
			case "*":
				if left == 0.0 || right == 0.0 {
					return 0.0
				}
				if left == 1.0 {
					return right
				}
				if right == 1.0 {
					return left
				}
				if l, ok := left.(float64); ok {
					if r, ok := right.([]interface{}); ok && r[0] == "*" {
						return simplify([]interface{}{"*", l * r[1].(float64), r[2]})
					}
				}
				if r, ok := right.(float64); ok {
					if l, ok := left.([]interface{}); ok && l[0] == "*" {
						return simplify([]interface{}{"*", r * l[1].(float64), l[2]})
					}
				}
			case "/":
				if left == 0.0 {
					return 0.0
				}
				if right == 1.0 {
					return left
				}
				if l, ok := left.(float64); ok {
					if r, ok := right.(float64); ok {
						return l / r
					}
				}
				if rightExpr, ok := right.([]interface{}); ok && rightExpr[0] == "^" {
					return []interface{}{"/", left, rightExpr}
				}
			case "^":
				if right == 1.0 {
					return left
				}
				if right == 0.0 {
					return 1.0
				}
				if l, ok := left.(float64); ok {
					if r, ok := right.(float64); ok {
						return []interface{}{"^", l, r}
					}
				}
			}
			return []interface{}{op, left, right}
		} else if len(v) == 2 {
			op, arg := v[0], simplify(v[1])
			return []interface{}{op, arg}
		}
		return v
	default:
		return v
	}
}

// Function to differentiate expression
func differentiate(expr interface{}) interface{} {
	switch v := expr.(type) {
	case string:
		if v == "x" {
			return 1.0
		} else {
			return 0.0
		}
	case float64:
		return 0.0
	case []interface{}:
		op := v[0]
		switch op {
		case "+":
			return simplify([]interface{}{"+", differentiate(v[1]), differentiate(v[2])})
		case "-":
			return simplify([]interface{}{"-", differentiate(v[1]), differentiate(v[2])})
		case "*":
			u, w := v[1], v[2]
			return simplify([]interface{}{"+", []interface{}{"*", differentiate(u), w}, []interface{}{"*", u, differentiate(w)}})
		case "/":
			u, w := v[1], v[2]
			if r, ok := w.(float64); ok {
				return simplify([]interface{}{"/", differentiate(u), r})
			}
			return simplify([]interface{}{"/", []interface{}{"-", []interface{}{"*", differentiate(u), w}, []interface{}{"*", u, differentiate(w)}}, []interface{}{"^", w, 2.0}})
		case "^":
			base, exponent := v[1], v[2]
			if exp, ok := exponent.(float64); ok {
				if exp == 0 {
					return 0.0
				}
				if exp == 1 {
					return differentiate(base)
				}
				return simplify([]interface{}{"*", exp, []interface{}{"*", []interface{}{"^", base, exp - 1.0}, differentiate(base)}})
			}
			panic("Exponentiation with non-constant exponent is not supported.")
		case "cos":
			return simplify([]interface{}{"*", -1.0, []interface{}{"*", differentiate(v[1]), []interface{}{"sin", v[1]}}})
		case "sin":
			return simplify([]interface{}{"*", differentiate(v[1]), []interface{}{"cos", v[1]}})
		case "exp":
			return simplify([]interface{}{"*", differentiate(v[1]), []interface{}{"exp", v[1]}})
		case "ln":
			return simplify([]interface{}{"/", differentiate(v[1]), v[1]})
		case "tan":
			return simplify([]interface{}{"*", differentiate(v[1]), []interface{}{"+", 1.0, []interface{}{"^", []interface{}{"tan", v[1]}, 2.0}}})
		default:
			panic(fmt.Sprintf("Unknown operator: %s", op))
		}
	default:
		panic(fmt.Sprintf("Invalid expression: %v", expr))
	}
}

func Diff(expr string) string {
	parsedExpr := parseExpression(expr)
	differentiatedExpr := differentiate(parsedExpr)
	simplifiedExpr := simplify(differentiatedExpr)
	return toString(simplifiedExpr)
}
