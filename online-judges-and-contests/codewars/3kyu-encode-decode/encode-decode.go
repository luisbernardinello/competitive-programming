package kata

func Encode(s string, n int) string {
	if n == 1 {
		return s
	}

	rails := make([][]rune, n)
	for i := range rails {
		rails[i] = []rune{}
	}

	index := 0
	direction := 1

	for _, char := range s {
		rails[index] = append(rails[index], char)
		index += direction

		if index == 0 || index == n-1 {
			direction *= -1
		}
	}

	encoded := ""
	for _, rail := range rails {
		encoded += string(rail)
	}

	return encoded
}

func Decode(s string, n int) string {
	if n == 1 {
		return s
	}

	railLengths := make([]int, n)
	index := 0
	direction := 1

	for i := 0; i < len(s); i++ {
		railLengths[index]++
		index += direction

		if index == 0 || index == n-1 {
			direction *= -1
		}
	}

	rails := make([][]rune, n)
	pos := 0
	for i, length := range railLengths {
		rails[i] = []rune(s[pos : pos+length])
		pos += length
	}

	result := make([]rune, len(s))
	index = 0
	direction = 1
	for i := range result {
		result[i] = rails[index][0]
		rails[index] = rails[index][1:]
		index += direction

		if index == 0 || index == n-1 {
			direction *= -1
		}
	}

	return string(result)
}
