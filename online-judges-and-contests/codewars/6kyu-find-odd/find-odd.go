package kata

// 6 kyu problem - Find the odd int

func FindOdd(seq []int) int {
	counts := make(map[int]int)
	for _, num := range seq {
		counts[num]++
	}

	for num, count := range counts {
		if count%2 != 0 {
			return num
		}
	}

	return 0 // return 0 when no odd count is found
}
