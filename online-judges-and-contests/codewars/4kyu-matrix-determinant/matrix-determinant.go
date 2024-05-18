package kata

func Determinant(matrix [][]int) int {
	n := len(matrix)
	if n == 1 {
		return matrix[0][0]
	}
	if n == 2 {
		return matrix[0][0]*matrix[1][1] - matrix[0][1]*matrix[1][0]
	}
	det := 0
	for i := 0; i < n; i++ {
		det += matrix[0][i] * cofactor(matrix, 0, i)
	}
	return det
}

func cofactor(matrix [][]int, row, col int) int {
	minor := minor(matrix, row, col)
	sign := 1
	if (row+col)%2 != 0 {
		sign = -1
	}
	return sign * Determinant(minor)
}

func minor(matrix [][]int, row, col int) [][]int {
	n := len(matrix)
	minor := make([][]int, n-1)
	for i := range minor {
		minor[i] = make([]int, n-1)
	}
	r := 0
	for i := 0; i < n; i++ {
		if i == row {
			continue
		}
		c := 0
		for j := 0; j < n; j++ {
			if j == col {
				continue
			}
			minor[r][c] = matrix[i][j]
			c++
		}
		r++
	}
	return minor
}
