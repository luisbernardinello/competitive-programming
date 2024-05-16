package kata

import (
	"sort"
	"strconv"
	"strings"
)

func Order(sentence string) string {
	if sentence == "" {
		return ""
	}

	words := strings.Split(sentence, " ")
	sort.SliceStable(words, func(i, j int) bool {
		return getNumber(words[i]) < getNumber(words[j])
	})

	return strings.Join(words, " ")
}

func getNumber(word string) int {
	for _, char := range word {
		if char >= '1' && char <= '9' {
			num, _ := strconv.Atoi(string(char))
			return num
		}
	}
	return -1 // -1 no number found
}
