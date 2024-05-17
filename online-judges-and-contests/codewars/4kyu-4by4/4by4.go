package kata

import (
	"sort"
	"sync"
)

type Pair struct {
	First  int
	Second int
}

const (
	GridSize = 4
	Sides    = 4
	Mask     = (1 << GridSize) - 1
)

var (
	possibilities [GridSize * GridSize]int
	start         [Sides * GridSize]int
	end           [Sides * GridSize]int
	inc           [Sides * GridSize]int
	results       [GridSize][GridSize]int
	clues         []int
	order         []int
	mutex         sync.Mutex
)

func setBit(n, pos int) int {
	return n | (1 << pos)
}

func clearBit(n, pos int) int {
	return n &^ (1 << pos)
}

func isBitSet(n, pos int) bool {
	return n&(1<<pos) > 0
}

func setValue(cellIndex, value int) {
	mask := Mask ^ (1 << value)
	rowStart := cellIndex - cellIndex%GridSize
	colStart := cellIndex % GridSize
	for i := 0; i < GridSize; i++ {
		possibilities[rowStart+i] &= mask
		possibilities[colStart+i*GridSize] &= mask
	}
	possibilities[cellIndex] = 1 << value
}

func checkUnique() int {
	decisions := 0
	for i := 0; i < Sides/2*GridSize; i++ {
		possibleIndices := make(map[int][]int)
		for j, k := start[i], 0; k < GridSize; j, k = j+inc[i], k+1 {
			for l := 0; l < GridSize; l++ {
				if isBitSet(possibilities[j], l) {
					possibleIndices[l] = append(possibleIndices[l], j)
				}
			}
		}

		for val, indices := range possibleIndices {
			if len(indices) == 1 {
				idx := indices[0]
				if possibilities[idx] != (1 << val) {
					decisions++
					setValue(idx, val)
				}
			}
		}
	}
	return decisions
}

func countPossible(val int) int {
	count := 0
	for val > 0 {
		count += val & 1
		val >>= 1
	}
	return count
}

func isValid() bool {
	for i := 0; i < Sides*GridSize; i++ {
		if clues[i] == 0 {
			continue
		}

		isDecided := true
		for j, k := start[i], 0; k < GridSize; j, k = j+inc[i], k+1 {
			if countPossible(possibilities[j]) != 1 {
				isDecided = false
				break
			}
		}

		if isDecided {
			largest := 0
			clueCount := 0
			for j, k := start[i], 0; k < GridSize; j, k = j+inc[i], k+1 {
				if largest < possibilities[j] {
					clueCount++
					largest = possibilities[j]
				}
			}
			if clueCount != clues[i] {
				return false
			}
		}
	}
	return true
}

func writeResults() {
	for i := 0; i < GridSize*GridSize; i++ {
		x := i / GridSize
		y := i % GridSize
		for j := 0; j < GridSize; j++ {
			if (1 << j) == possibilities[i] {
				results[x][y] = j + 1
				break
			}
		}
	}
}

func dfs(idx int) bool {
	if idx >= len(order) {
		if isValid() {
			writeResults()
			return true
		}
		return false
	}

	i := order[idx]
	possibilitiesBackup := possibilities

	for j := 0; j < GridSize; j++ {
		m := (1 << j) & possibilities[i]
		if m == 0 {
			continue
		}

		setValue(i, j)
		found := false
		if isValid() {
			found = dfs(idx + 1)
		}
		if found {
			return true
		}
		possibilities = possibilitiesBackup
	}
	return false
}

func SolvePuzzle(cluesInput []int) [][]int {
	clues = cluesInput
	for i := 0; i < GridSize*GridSize; i++ {
		possibilities[i] = Mask
	}

	for i := 0; i < GridSize; i++ {
		start[i] = i
		end[i] = (GridSize-1)*GridSize + i
		inc[i] = GridSize
	}

	for i, j := 0, GridSize; i < GridSize; i, j = i+1, j+1 {
		start[j] = i*GridSize + GridSize - 1
		end[j] = i * GridSize
		inc[j] = -1
	}

	for i, j := 0, 2*GridSize; i < GridSize; i, j = i+1, j+1 {
		start[j] = GridSize*GridSize - 1 - i
		end[j] = GridSize - 1 - i
		inc[j] = -GridSize
	}

	for i, j := 0, 3*GridSize; i < GridSize; i, j = i+1, j+1 {
		start[j] = (GridSize-i-1)*GridSize + 0
		end[j] = (GridSize-i-1)*GridSize + GridSize - 1
		inc[j] = 1
	}

	for i := 0; i < Sides*GridSize; i++ {
		if clues[i] == 0 {
			continue
		}
		for j, k := start[i], 0; k < GridSize; j, k = j+inc[i], k+1 {
			mask := Mask
			for l := GridSize + k - clues[i] + 1; l < GridSize; l++ {
				mask ^= 1 << l
			}
			possibilities[j] &= mask
		}
	}

	for checkUnique() > 0 {
	}

	indexPossibilities := make([]Pair, 0)
	for i := 0; i < GridSize*GridSize; i++ {
		nPossible := countPossible(possibilities[i])
		if nPossible > 1 {
			indexPossibilities = append(indexPossibilities, Pair{nPossible, i})
		}
	}

	sort.Slice(indexPossibilities, func(i, j int) bool {
		return indexPossibilities[i].First < indexPossibilities[j].First
	})

	order = make([]int, len(indexPossibilities))
	for i := 0; i < len(indexPossibilities); i++ {
		order[i] = indexPossibilities[i].Second
	}

	dfs(0)

	result := make([][]int, GridSize)
	for i := 0; i < GridSize; i++ {
		result[i] = make([]int, GridSize)
		for j := 0; j < GridSize; j++ {
			result[i][j] = results[i][j]
		}
	}
	return result
}
