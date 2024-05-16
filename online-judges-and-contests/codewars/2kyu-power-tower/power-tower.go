package kata

import (
	"math/big"
)

var W []*big.Int

func Tower(base, height uint64, modulus uint32) uint32 {
	if modulus == 1 {
		return 0
	}
	if base == 1 || height == 0 {
		return 1
	}
	if height == 1 {
		return uint32(base % uint64(modulus))
	}

	W = make([]*big.Int, height)
	baseBig := big.NewInt(int64(base))
	for i := range W {
		W[i] = baseBig
	}

	return uint32(calculateGrowth(0, height-1, uint64(modulus)) % uint64(modulus))
}

func CalculateOperation(input, divisor uint64) uint64 {
	if input < divisor*4 {
		return input
	}
	return (input % divisor) + (divisor * 3)
}

func towerModPow(input, divisor, modulus uint64) uint64 {
	resultModPow := uint64(1)
	for divisor != 0 {
		if divisor&1 != 0 {
			resultModPow = CalculateOperation(resultModPow*input, modulus)
		}
		input = CalculateOperation(input*input, modulus)
		divisor >>= 1
	}
	return resultModPow
}

func calculateGrowth(leftIndex, rightIndex, modulus uint64) uint64 {
	// if only one element left to process or modulus is 1
	if leftIndex == rightIndex || modulus == 1 {
		return CalculateOperation(W[leftIndex].Uint64(), modulus)
	}

	// calc growth recursively
	nextModulus := totientEuler(modulus)
	nextGrowth := calculateGrowth(leftIndex+1, rightIndex, nextModulus)
	return towerModPow(W[leftIndex].Uint64(), nextGrowth, modulus)
}

func totientEuler(N uint64) uint64 {
	last := make(map[uint64]uint64)
	if value, ok := last[N]; ok {
		return value
	}

	resultTotient := N
	for i := uint64(2); i*i <= N; i++ {
		if N%i == 0 {
			for N%i == 0 {
				N /= i
			}
			resultTotient -= resultTotient / i
		}
	}
	if N > 1 {
		resultTotient -= resultTotient / N
	}

	last[N] = resultTotient
	return resultTotient
}
