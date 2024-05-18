package kata

import (
	"sync"
)

type ViewClue struct {
	Front int
	Back  int
}

type BuildingRow []int

const gridSize = 6

type PuzzleSolver struct {
	candidateCols [][]BuildingRow
	candidateRows [][]BuildingRow
	columnClues   []ViewClue
	rowClues      []ViewClue
	allPerms      []BuildingRow
	permsMap      map[string]bool
}

func nSolvePuzzle(clues []int) *PuzzleSolver {
	columnClues, rowClues := transformClues(clues)
	perms := generatePermutations(generateFloorOptions(gridSize))

	return &PuzzleSolver{
		allPerms:    perms,
		columnClues: columnClues,
		rowClues:    rowClues,
		permsMap:    createPermsMap(perms),
	}
}

func (ps *PuzzleSolver) Solve() [][]int {
	var wg sync.WaitGroup
	ps.candidateRows = ps.filterRows(ps.rowClues, ps.allPerms, &wg)
	ps.candidateCols = ps.filterRows(ps.columnClues, ps.allPerms, &wg)
	wg.Wait()
	return ps.assembleGrid()
}

func (ps *PuzzleSolver) assembleGrid() [][]int {
	for !ps.isUniqueSolution() {
		var wg sync.WaitGroup
		wg.Add(len(ps.candidateRows) + len(ps.candidateCols))

		for i := range ps.candidateRows {
			go func(i int) {
				defer wg.Done()
				ps.candidateRows[i] = filterValidRows(ps.candidateRows[i], ps.candidateCols, i)
			}(i)
		}

		for i := range ps.candidateCols {
			go func(i int) {
				defer wg.Done()
				ps.candidateCols[i] = filterValidRows(ps.candidateCols[i], ps.candidateRows, i)
			}(i)
		}

		wg.Wait()
	}

	result := make([][]int, gridSize)
	for i, rows := range ps.candidateRows {
		result[i] = rows[0]
	}
	return result
}

func filterValidRows(rows []BuildingRow, opposingRows [][]BuildingRow, position int) (result []BuildingRow) {
	for _, row := range rows {
		if isRowValid(row, position, opposingRows) {
			result = append(result, row)
		}
	}
	return result
}

func isRowValid(row BuildingRow, position int, opposingRows [][]BuildingRow) bool {
	for i, value := range row {
		if !containsValueAtPosition(value, position, opposingRows[i]) {
			return false
		}
	}
	return true
}

func containsValueAtPosition(value, position int, rows []BuildingRow) bool {
	for _, row := range rows {
		if row[position] == value {
			return true
		}
	}
	return false
}

func (ps *PuzzleSolver) isUniqueSolution() bool {
	for _, cols := range ps.candidateCols {
		if len(cols) > 1 {
			return false
		}
	}
	for _, rows := range ps.candidateRows {
		if len(rows) > 1 {
			return false
		}
	}
	return true
}

func (ps *PuzzleSolver) filterRows(clues []ViewClue, rows []BuildingRow, wg *sync.WaitGroup) [][]BuildingRow {
	result := make([][]BuildingRow, len(clues))
	for i, clue := range clues {
		wg.Add(1)
		go func(i int, clue ViewClue) {
			defer wg.Done()
			var validRows []BuildingRow
			for _, row := range rows {
				if row.matchesClue(clue) {
					validRows = append(validRows, row)
				}
			}
			result[i] = validRows
		}(i, clue)
	}
	return result
}

func transformClues(clues []int) (columnClues, rowClues []ViewClue) {
	for i := 0; i < len(clues)/2; i++ {
		c := ViewClue{
			Front: clues[i],
			Back:  clues[gridSize*(3+i/gridSize)-(i%gridSize+1)],
		}
		if i < gridSize {
			columnClues = append(columnClues, c)
		} else {
			c.Front, c.Back = c.Back, c.Front
			rowClues = append(rowClues, c)
		}
	}
	return columnClues, rowClues
}

func generatePermutations(source []int) (result []BuildingRow) {
	p := make([]int, len(source))
	for p[0] < len(p) {
		result = append(result, createPermutation(source, p))
		advancePermutation(p)
	}
	return result
}

func createPermsMap(perms []BuildingRow) map[string]bool {
	permsMap := make(map[string]bool)
	for _, perm := range perms {
		permsMap[rowKey(perm)] = true
	}
	return permsMap
}

func rowKey(row BuildingRow) string {
	key := ""
	for _, val := range row {
		key += string(rune(val + '0'))
	}
	return key
}

func advancePermutation(p []int) {
	for i := len(p) - 1; i >= 0; i-- {
		if i == 0 || p[i] < len(p)-i-1 {
			p[i]++
			return
		}
		p[i] = 0
	}
}

func createPermutation(source, p []int) BuildingRow {
	result := make(BuildingRow, len(source))
	copy(result, source)
	for i, v := range p {
		result[i], result[i+v] = result[i+v], result[i]
	}
	return result
}

func (br BuildingRow) countVisibleBuildings() (count int) {
	max := 0
	for _, val := range br {
		if val > max {
			count++
			max = val
		}
	}
	return count
}

func generateFloorOptions(size int) []int {
	result := make([]int, size)
	for i := 0; i < size; i++ {
		result[i] = i + 1
	}
	return result
}

func (br BuildingRow) matchesClue(clue ViewClue) bool {
	return (clue.Front == 0 || br.countVisibleBuildings() == clue.Front) &&
		(clue.Back == 0 || br.reverseRow().countVisibleBuildings() == clue.Back)
}

func (br BuildingRow) reverseRow() BuildingRow {
	size := len(br)
	result := make(BuildingRow, size)
	for i, v := range br {
		result[size-1-i] = v
	}
	return result
}

///////////////////////////////////////////
//////////////////////////

func SolvePuzzle(clues []int) [][]int {
	return nSolvePuzzle(clues).Solve()
}
