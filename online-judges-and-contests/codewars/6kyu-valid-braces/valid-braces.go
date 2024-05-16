package kata

func ValidBraces(str string) bool {
	stack := make([]rune, 0)

	bracePairs := map[rune]rune{ // store mappings
		')': '(',
		']': '[',
		'}': '{',
	}

	for _, char := range str {
		switch char {
		case '(', '[', '{':
			stack = append(stack, char)
		case ')', ']', '}':
			if len(stack) == 0 || stack[len(stack)-1] != bracePairs[char] {
				return false // order invalid
			}
			stack = stack[:len(stack)-1]
		}
	}
	return len(stack) == 0
}
