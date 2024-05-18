package kata

import (
	"math"
)

func SumOfSquares(n uint64) uint64 {
	if isPerfectSquare(n) {
		return 1
	}

	for i := uint64(1); i*i <= n; i++ {
		if isPerfectSquare(n - i*i) {
			return 2
		}
	}

	//  Lagrange's three-square theorem
	for i := uint64(1); i*i <= n; i++ {
		for j := uint64(1); j*j <= n-i*i; j++ {
			if isPerfectSquare(n - i*i - j*j) {
				return 3
			}
		}
	}
	return 4
}
func isPerfectSquare(x uint64) bool {
	s := uint64(math.Sqrt(float64(x)))
	return s*s == x
}
