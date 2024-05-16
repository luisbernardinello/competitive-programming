package kata

// 6 kyu problem - Array.diff

func ArrayDiff(a, b []int) []int {
	result := make([]int, 0)
	for _, num := range a {
		if !contains(b, num) {
			result = append(result, num)
		}
	}
	return result
}

func contains(arr []int, num int) bool {
	for _, n := range arr {
		if n == num {
			return true
		}
	}
	return false
}
