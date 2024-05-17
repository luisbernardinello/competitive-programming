package kata

import (
	"math/big"
	"sync"
)

var (
	phiCache = make(map[uint64]uint64)
	mu       sync.RWMutex
)

func Tower(base, height uint64, modulus uint32) uint32 {
	switch {
	case modulus == 1:
		return 0
	case base == 1 || height == 0:
		return 1
	case height == 1:
		return uint32(base % uint64(modulus))
	}

	baseBig := big.NewInt(int64(base))
	return uint32(calculateGrowth(baseBig, height, uint64(modulus)).Uint64() % uint64(modulus))
}

func calculateGrowth(base *big.Int, height uint64, modulus uint64) *big.Int {
	if height == 1 || modulus == 1 {
		return calculateOperation(base, big.NewInt(int64(modulus)))
	}

	nextModulus := totientEuler(modulus)
	nextGrowth := calculateGrowth(base, height-1, nextModulus)
	return towerModPow(base, nextGrowth, big.NewInt(int64(modulus)))
}

func calculateOperation(input, divisor *big.Int) *big.Int {
	fourTimesDivisor := new(big.Int).Mul(divisor, big.NewInt(4))
	if input.Cmp(fourTimesDivisor) < 0 {
		return input
	}
	remainder := new(big.Int).Mod(input, divisor)
	threeTimesDivisor := new(big.Int).Mul(divisor, big.NewInt(3))
	return new(big.Int).Add(remainder, threeTimesDivisor)
}

func towerModPow(base, exp, mod *big.Int) *big.Int {
	result := big.NewInt(1)
	zero := big.NewInt(0)
	one := big.NewInt(1)

	for exp.Cmp(zero) != 0 {
		if new(big.Int).And(exp, one).Cmp(one) == 0 {
			result = calculateOperation(new(big.Int).Mul(result, base), mod)
		}
		base = calculateOperation(new(big.Int).Mul(base, base), mod)
		exp = new(big.Int).Rsh(exp, 1)
	}
	return result
}

func totientEuler(n uint64) uint64 {
	mu.RLock()
	if result, exists := phiCache[n]; exists {
		mu.RUnlock()
		return result
	}
	mu.RUnlock()

	originalN := n
	result := n
	for i := uint64(2); i*i <= n; i++ {
		if n%i == 0 {
			for n%i == 0 {
				n /= i
			}
			result -= result / i
		}
	}
	if n > 1 {
		result -= result / n
	}

	mu.Lock()
	phiCache[originalN] = result
	mu.Unlock()

	return result
}
