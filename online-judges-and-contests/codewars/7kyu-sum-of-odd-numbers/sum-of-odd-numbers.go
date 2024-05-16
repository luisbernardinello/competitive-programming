package kata

func RowSumOddNumbers(n int) int {
	first := (n * (n - 1)) + 1
	sum := 0
	for i := 0; i < n; i++ {
		sum += first + (2 * i)
	}
	return sum
}
