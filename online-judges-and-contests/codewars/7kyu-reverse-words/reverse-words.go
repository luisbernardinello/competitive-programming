package kata

// Reverse words 7kyu problem

import (
	"strings"
)

func Reverse(str string) (resultString string) {
	for _, chr := range str {
		resultString = string(chr) + resultString
	}
	return resultString
}

func ReverseWords(str string) string {
	words := strings.Split(str, " ")
	for i, word := range words {
		words[i] = Reverse(string(word))
	}

	return strings.Join(words, " ")
}
