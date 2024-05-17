package kata

func FindMissingLetter(chars []rune) (missingLetter rune) {
	for i := 0; i < len(chars)-1; i++ {
		if chars[i+1]-chars[i] != 1 {
			missingLetter = chars[i] + 1
			break
		}
	}
	return missingLetter
}
