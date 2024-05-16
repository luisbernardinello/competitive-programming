package kata

// 6 kyu problem - Stop gninnipS My sdroW!

import (
	"strings"
)

func ReverseString(str string) (resultString string) {
	for _, chr := range str {
		resultString = string(chr) + resultString
	}
	return resultString
}

func SpinWords(str string) string {
	words := strings.Split(str, " ")
	for i, word := range words {
		if len(word) >= 5 {
			words[i] = ReverseString(word)
		}
	}
	return strings.Join(words, " ")
}
