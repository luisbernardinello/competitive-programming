package kata

import (
	"sync"
)

const N = 6

type Line []int

type Clue struct {
	F int
	L int
}

type Solver struct {
	possibleCols [][]Line
	possibleRows [][]Line
	colClues     []Clue
	rowClues     []Clue
	permutations []Line
}

func NewSolver(clues []int) *Solver {
	colClues, rowClues := prepareClues(clues)
	return &Solver{
		permutations: generateAllLinePermutations(possibleFloors(N)),
		colClues:     colClues,
		rowClues:     rowClues,
	}
}

func SolvePuzzle(clues []int) [][]int {
	return NewSolver(clues).Solve()
}

func (s *Solver) Solve() [][]int {
	var wg sync.WaitGroup
	s.possibleRows = s.filterPossibleLines(s.rowClues, s.permutations, &wg)
	s.possibleCols = s.filterPossibleLines(s.colClues, s.permutations, &wg)
	wg.Wait()
	return s.buildMap()
}

func (s *Solver) buildMap() [][]int {
	for !s.hasUniqueSolution() {
		var wg sync.WaitGroup
		wg.Add(len(s.possibleRows) + len(s.possibleCols))

		for i := range s.possibleRows {
			go func(i int) {
				defer wg.Done()
				s.possibleRows[i] = filterMatchingLines(s.possibleRows[i], s.possibleCols, i)
			}(i)
		}

		for i := range s.possibleCols {
			go func(i int) {
				defer wg.Done()
				s.possibleCols[i] = filterMatchingLines(s.possibleCols[i], s.possibleRows, i)
			}(i)
		}

		wg.Wait()
	}

	result := make([][]int, N)
	for i, rows := range s.possibleRows {
		result[i] = rows[0]
	}
	return result
}

func filterMatchingLines(lines []Line, oppositeLines [][]Line, position int) (res []Line) {
	for _, line := range lines {
		if isLineMatching(line, position, oppositeLines) {
			res = append(res, line)
		}
	}
	return res
}

func isLineMatching(line Line, position int, oppositeLines [][]Line) bool {
	for i, elem := range line {
		if !containsElementAtPosition(elem, position, oppositeLines[i]) {
			return false
		}
	}
	return true
}

func containsElementAtPosition(elem, position int, lines []Line) bool {
	for _, line := range lines {
		if line[position] == elem {
			return true
		}
	}
	return false
}

func (s *Solver) hasUniqueSolution() bool {
	for _, cols := range s.possibleCols {
		if len(cols) > 1 {
			return false
		}
	}
	for _, rows := range s.possibleRows {
		if len(rows) > 1 {
			return false
		}
	}
	return true
}

func (s *Solver) filterPossibleLines(clues []Clue, lines []Line, wg *sync.WaitGroup) [][]Line {
	res := make([][]Line, len(clues))
	for i, clue := range clues {
		wg.Add(1)
		go func(i int, clue Clue) {
			defer wg.Done()
			var cluePossibleLines []Line
			for _, line := range lines {
				if line.matchesClue(clue) {
					cluePossibleLines = append(cluePossibleLines, line)
				}
			}
			res[i] = cluePossibleLines
		}(i, clue)
	}
	return res
}

func prepareClues(clues []int) (colClues, rowClues []Clue) {
	for i := 0; i < len(clues)/2; i++ {
		c := Clue{
			F: clues[i],
			L: clues[N*(3+i/N)-(i%N+1)],
		}
		if i < N {
			colClues = append(colClues, c)
		} else {
			c.F, c.L = c.L, c.F
			rowClues = append(rowClues, c)
		}
	}
	return colClues, rowClues
}

func generateAllLinePermutations(src []int) (res []Line) {
	p := make([]int, len(src))
	for p[0] < len(p) {
		res = append(res, permute(src, p))
		nextPerm(p)
	}
	return res
}

func nextPerm(p []int) {
	for i := len(p) - 1; i >= 0; i-- {
		if i == 0 || p[i] < len(p)-i-1 {
			p[i]++
			return
		}
		p[i] = 0
	}
}

func permute(src, p []int) Line {
	res := make(Line, len(src))
	copy(res, src)
	for i, v := range p {
		res[i], res[i+v] = res[i+v], res[i]
	}
	return res
}

func possibleFloors(size int) []int {
	res := make([]int, size)
	for i := 0; i < size; i++ {
		res[i] = i + 1
	}
	return res
}

func (l Line) countVisible() (count int) {
	max := 0
	for _, val := range l {
		if val > max {
			count++
			max = val
		}
	}
	return count
}

func (l Line) reverse() Line {
	size := len(l)
	res := make(Line, size)
	for i, v := range l {
		res[size-1-i] = v
	}
	return res
}

func (l Line) matchesClue(clue Clue) bool {
	return (clue.F == 0 || l.countVisible() == clue.F) &&
		(clue.L == 0 || l.reverse().countVisible() == clue.L)
}
