package kata

import (
	"unicode"
)

func wave(words string) []string {
	var result []string
	for i, chr := range words {
		if unicode.IsLetter(chr) {
			result = append(result, words[:i]+string(unicode.ToUpper(chr))+words[i+1:])
		}
	}
	return result
}
